mod oauth2;
mod watery_error;

use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tauri::Listener;
use tauri::State;
use tauri_plugin_deep_link::DeepLinkExt;

use parking_lot::Mutex;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task;
use warp::http::Uri;
use warp::Filter;

use oauth2::*;
use watery_error::*;

const PORT: u16 = 8080;

const GOOGLE_CLIENT_ID: &str =
    "632451831672-mfg1ol2lofb8ntf9og1eblkmgc81hv70.apps.googleusercontent.com";
const GOOGLE_CLIENT_SECRET: &str = "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";
const GOOGLE_REDIRECT_URI: &str = "http://localhost:8080/callback";
const GOOGLE_AUTH_URL: &str = "https://oauth2.googleapis.com/token";

#[derive(Clone, Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct GoogleResp {
    access_token: String,
    expires_in: i32,
    refresh_token: String,
    scope: String,
    token_type: String,
}

#[derive(Default)]
struct AppState {
    server_handle: Option<task::JoinHandle<()>>,
    shutdown_tx: Option<broadcast::Sender<()>>,
}

#[derive(Default)]
struct Oauth2State {
    inner: Mutex<HashMap<String, Oauth2Client>>,
}

impl Deref for Oauth2State {
    type Target = Mutex<HashMap<String, Oauth2Client>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Oauth2State {
    fn from_config(cfg: HashMap<String, Oauth2Cfg>) -> Self {
        let mut map: HashMap<String, Oauth2Client> = HashMap::new();
        let _: Vec<Option<Oauth2Client>> = cfg
            .into_iter()
            .map(|(key, value)| map.insert(key, value.into()))
            .collect();
        Oauth2State {
            inner: Mutex::new(map),
        }
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_vendor_link(vendor: String, auth: State<Oauth2State>) -> WateryResult<String> {
    let mut auth = auth.lock();
    let client = auth.get_mut(&vendor).ok_or(WateryError::NoSuchVendor)?;
    let url = client.get_auth_url();
    Ok(url.to_string())
}

#[tauri::command]
async fn new_server(state: State<'_, Arc<Mutex<AppState>>>) -> Result<(), String> {
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);

    let mut state = state.lock();

    let login_route = warp::path("login")
        .map(|| warp::redirect::temporary(Uri::from_static("https://oauth2-provider.com/auth")));

    let callback_route = warp::path("callback")
        .and(warp::query::<HashMap<String, String>>())
        .and_then(move |params: HashMap<String, String>| {
            let proxy = reqwest::Proxy::https("http://127.0.0.1:10006").unwrap();
            let client = reqwest::Client::builder().proxy(proxy).build().unwrap();
            let mut accese_token = String::new();
            let mut refresh_token = String::new();
            async move {
                if let Some(token) = params.get("code") {
                    println!("{token}");
                    let form = [
                        ("client_id", GOOGLE_CLIENT_ID),
                        ("client_secret", GOOGLE_CLIENT_SECRET),
                        ("code", token),
                        ("redirect_uri", GOOGLE_REDIRECT_URI),
                        ("grant_type", "authorization_code"),
                    ];
                    let resp = client
                        .post(GOOGLE_AUTH_URL)
                        .form(&form)
                        .send()
                        .await
                        .unwrap();
                    println!("response {:?}", resp);
                    let res: GoogleResp = resp.json().await.unwrap();
                    println!("res: {:?}, res", res);
                    accese_token = res.access_token;
                    refresh_token = res.refresh_token;
                }
                let redirect_uri = Uri::from_str(
                    format!("watery://accese_token={accese_token}&refresh_token={refresh_token}")
                        .as_str(),
                )
                .unwrap();
                //Ok(warp::redirect::temporary(redirect_uri))
                Ok(warp::redirect::temporary(redirect_uri)) as Result<_, warp::Rejection>
            }
        });

    let routes = login_route.or(callback_route);

    let (addr, server) =
        warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], PORT), async move {
            shutdown_rx.recv().await.ok(); // 等待关闭信号
        });

    let handle = tokio::task::spawn(server);

    state.server_handle = Some(handle);
    state.shutdown_tx = Some(shutdown_tx);
    println!("bind addr {addr} ok...");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(Mutex::new(AppState::default()));

    let oauth2_cfg = read_oauth2_vendor();
    let oauth2_state = Oauth2State::from_config(oauth2_cfg);

    let mut app_builder = tauri::Builder::default();
    #[cfg(desktop)]
    {
        app_builder = app_builder.plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);
            app.emit("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }));
    }

    let log_plugin = tauri_plugin_log::Builder::new()
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Stdout,
        ))
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir {
                file_name: Some("watery.log".to_string()),
            },
        ))
        .level(log::LevelFilter::Debug)
        .max_file_size(50 * 1024 * 1024 /* bytes */)
        .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepOne)
        .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
        .build();

    app_builder
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(log_plugin)
        .setup(|app| {
            // ensure deep links are registered on the system
            // this is useful because AppImages requires additional setup to be available in the system
            // and calling register() makes the deep links immediately available - without any user input
            #[cfg(target_os = "linux")]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                app.deep_link().register_all()?;
            }

            app.deep_link().register("watery")?;
            app.listen("single-instance", |url| {
                dbg!("--------");
                dbg!(url);
            });
            Ok(())
        })
        .manage(state)
        .manage(oauth2_state)
        .invoke_handler(tauri::generate_handler![greet, new_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
