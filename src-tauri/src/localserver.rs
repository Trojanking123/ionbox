use std::{collections::HashMap, sync::Arc, time::Duration};

use axum::{
    debug_handler,
    extract::{Query, State},
    response::Redirect,
    routing::{get, post},
    Json, Router,
};

use oauth2::{AccessToken, RefreshToken};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

#[derive(Serialize, Deserialize)]
pub struct Tokens {
    pub access_token: Option<AccessToken>,
    pub refresh_token: Option<RefreshToken>,
}

struct StateValue(IonOauth2Provider, Option<String>);

struct LocalState {
    config: IonConfigState,
    client: Oauth2State,
    db: HashMap<String, StateValue>,
    tokens: HashMap<String, (Option<AccessToken>, Option<RefreshToken>)>,
}

use crate::{read_oauth2_provider, IonConfigState, IonOauth2Provider, Oauth2State, LOCAL_ADDR};

pub async fn local_server(cfg: IonConfigState) {
    let client = read_oauth2_provider();
    let oauth2_state = Oauth2State::from_config(client);
    let app_state = Arc::new(Mutex::new(LocalState {
        config: cfg,
        client: oauth2_state,
        db: HashMap::new(),
        tokens: HashMap::new(),
    }));

    // build our application with a route
    let app = Router::new()
        .route("/register", post(register))
        .route("/callback", post(auth))
        .route("/tokens", get(get_tokens))
        .route(
            "/loggedin",
            get(|| async {
                sleep(Duration::from_secs(4)).await;
                "logged in"
            }),
        )
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(LOCAL_ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn get_tokens(
    Query(params): Query<HashMap<String, String>>,
    State(app_state): State<Arc<Mutex<LocalState>>>,
) -> Json<Tokens> {
    let state = params.get("state").unwrap().to_owned();
    let mut app_sate_guard = app_state.lock();
    let tokens = app_sate_guard.tokens.remove(&state).unwrap();

    let tokens = Tokens {
        access_token: tokens.0,
        refresh_token: tokens.1,
    };

    Json(tokens)
}

#[debug_handler]
async fn register(
    Query(params): Query<HashMap<String, String>>,
    State(app_state): State<Arc<Mutex<LocalState>>>,
) {
    let state = params.get("state").unwrap().to_owned();
    let provider: IonOauth2Provider =
        IonOauth2Provider::from(params.get("provider").unwrap().to_owned());
    let veri = params.get("verifier").cloned();
    let value = StateValue(provider, veri);

    let mut app_sate = app_state.lock();
    app_sate.db.insert(state, value);
}

#[debug_handler]
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

        let oauth2_guard = app_sate_guard.client.lock();
        let oauth2_client = oauth2_guard.get(&state_value.0).cloned().unwrap();
        let verifier = state_value.1.clone();
        (oauth2_client, proxy, verifier)
    };

    let token = oauth2_client
        .get_token(auth_code, proxy, verifier)
        .await
        .unwrap();

    let mut app_sate_guard = app_state.lock();
    app_sate_guard.db.remove(state);
    app_sate_guard.tokens.insert(state.to_owned(), token);
    // let redirect_url = format!(
    //     "ionbox://access_token={:?}&refresh_token={:?}",
    //     access_token.unwrap().into_secret(),
    //     refres_token.unwrap().into_secret()
    // );

    Redirect::temporary("/loggedin")
}
