mod localserver;
mod oauth2;
mod watery_config;
mod watery_const;
mod watery_error;
mod watery_states;

use std::collections::HashMap;

use log::info;
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tauri::Listener;
use tauri::State;
use tauri_plugin_deep_link::DeepLinkExt;

use parking_lot::Mutex;
use std::sync::Arc;
use watery_config::WateryConfig;

pub use localserver::*;
pub use oauth2::*;
pub use watery_const::*;
pub use watery_error::*;
pub use watery_states::*;

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

#[tauri::command]
fn get_provider_link(
    provider: String,
    auth: State<Oauth2State>,
) -> WateryResult<(String, String, Option<String>)> {
    let provider: WateryOauth2Provider = provider.into();
    let mut auth = auth.lock();
    let client = auth.get_mut(&provider).ok_or(WateryError::NoSuchProvider)?;
    let (url, csrf_token, veri) = client.get_auth_url();
    Ok((
        url.to_string(),
        csrf_token.into_secret(),
        veri.map(|v| v.into_secret()),
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(Mutex::new(AppState::default()));

    let oauth2_cfg = {
        let mut oauth2_map = HashMap::new();
        let oauth2_cfg = include_str!("oauth2.json");
        let oauth2_cfg: Vec<WateryOauth2Cfg> = serde_json::from_str(oauth2_cfg).unwrap();
        let _: Vec<Option<WateryOauth2Cfg>> = oauth2_cfg
            .into_iter()
            .map(|oauth2| oauth2_map.insert(oauth2.provider.clone(), oauth2))
            .collect();
        oauth2_map
    };
    let oauth2_state = Oauth2State::from_config(oauth2_cfg);

    let cfg = WateryConfig::read_from_file(CONFIG_PATH).unwrap();
    let cfg_state = WateryConfigState::from(cfg);
    let cfg_state_clone = cfg_state.clone();

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
                info!("{:?}", url);
            });
            dbg!("start local server...");
            tauri::async_runtime::spawn(local_server(cfg_state_clone));
            dbg!("start local server ok");
            Ok(())
        })
        .manage(state)
        .manage(oauth2_state)
        .manage(cfg_state)
        .invoke_handler(tauri::generate_handler![get_provider_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
