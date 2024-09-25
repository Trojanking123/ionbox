use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::post, Router,
};

use parking_lot::Mutex;


struct StateValue(WateryOauth2Provider, Option<String>);

struct LocalState {
    config: WateryConfigState,
    client: Oauth2State,
    db: HashMap<String, StateValue>,
}

use crate::{
    read_oauth2_provider, Oauth2State, WateryConfigState, WateryOauth2Provider, LOCAL_ADDR,
};

pub async fn local_server(cfg: WateryConfigState) {
    let client = read_oauth2_provider();
    let oauth2_state = Oauth2State::from_config(client);
    let app_state = Arc::new(Mutex::new(LocalState {
        config: cfg,
        client: oauth2_state,
        db: HashMap::new(),
    }));

    // build our application with a route
    let app = Router::new()
        .route("/register", post(register))
        .route("/callback", post(auth))
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(LOCAL_ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn register(
    Query(params): Query<HashMap<String, String>>,
    State(app_state): State<Arc<Mutex<LocalState>>>,
) {
    let state = params.get("state").unwrap().to_owned();
    let provider: WateryOauth2Provider =
        WateryOauth2Provider::from(params.get("provider").unwrap().to_owned());
    let veri = params.get("kkk").cloned();
    let value = StateValue(provider, veri);

    let mut app_sate = app_state.lock();

    app_sate.db.insert(state, value);
}


async fn auth(
    Query(params): Query<HashMap<String, String>>,
    State(app_state): State<Arc<Mutex<LocalState>>>,
) -> Redirect {
    let state = params.get("state").unwrap();
    let auth_code = params.get("code").unwrap().to_owned();

    let (oauth2_client, proxy, verifier) = {
        let app_sate_guard = app_state.lock();

        let state_value = app_sate_guard.db.get(state).unwrap();

        let proxy_guard = app_sate_guard.config.read();
        let proxy = proxy_guard.proxy.clone().map(|url| url.to_string());
        drop(proxy_guard);

        let oauth2_guard = app_sate_guard.client.lock();
        let oauth2_client = oauth2_guard.get(&state_value.0).cloned().unwrap();
        let verifier = state_value.1.clone();
        (oauth2_client, proxy, verifier)
    };

    let (access_token, refres_token) = oauth2_client
        .get_token(auth_code, proxy, verifier)
        .await
        .unwrap();

    let mut app_sate_guard = app_state.lock();
    app_sate_guard.db.remove(state);
    let redirect_url = format!(
        "watery://access_token={:?}&refresh_token={:?}",
        access_token.unwrap().into_secret(),
        refres_token.unwrap().into_secret()
    );

    Redirect::temporary(redirect_url.as_str())
}
