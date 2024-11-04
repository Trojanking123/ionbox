mod commands;
mod migration;
pub mod ion_config;
pub mod ion_const;
pub mod ion_error;
pub mod ion_states;
pub mod localserver;
pub mod oauth2;
pub mod utils;
pub mod entities;

use std::collections::HashMap;
use std::sync::Arc;

use log::info;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tauri::Listener;
use tauri_plugin_deep_link::DeepLinkExt;

use ion_config::IonConfig;
use ion_const::*;
use ion_error::*;
use ion_states::*;
use localserver::*;
use migration::*;
use oauth2::*;
use commands::{get_provider_link, poll, register};

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(Mutex::new(AppState::default()));

    let oauth2_cfg = {
        let mut oauth2_map = HashMap::new();
        let oauth2_cfg = include_str!("oauth2.json");
        let oauth2_cfg: Vec<IonOauth2Cfg> = serde_json::from_str(oauth2_cfg).unwrap();
        let _: Vec<Option<IonOauth2Cfg>> = oauth2_cfg
            .into_iter()
            .map(|oauth2| oauth2_map.insert(oauth2.provider.clone(), oauth2))
            .collect();
        oauth2_map
    };
    let oauth2_state = Oauth2State::from_config(oauth2_cfg);

    let cfg = IonConfig::read_from_file(CONFIG_PATH).unwrap();
    let cfg_state = IonConfigState::from(cfg);
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
        // .target(tauri_plugin_log::Target::new(
        //     tauri_plugin_log::TargetKind::Stdout,
        // ))
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::LogDir {
                file_name: Some("ionbox.log".to_string()),
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

            app.deep_link().register("ionbox")?;
            app.listen("single-instance", |url| {
                println!("-----------------");
                info!("--------------{:?}", url);
            });

            let db_url = utils::get_db_abs_path();
            tauri::async_runtime::spawn(local_server(cfg_state_clone));
            tauri::async_runtime::spawn( refresh_db(db_url));
            Ok(())
        })
        .manage(state)
        .manage(oauth2_state)
        .manage(cfg_state)
        .invoke_handler(tauri::generate_handler![get_provider_link, poll, register])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
