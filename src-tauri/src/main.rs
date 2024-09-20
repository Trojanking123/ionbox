// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod oauth2;
mod watery_error;

use cfg_aliases::cfg_aliases;

#[tokio::main]
async fn main() {
    cfg_aliases! {
        desktop: {any(targetos = "linux", targetos = "macos", targetos="windows")},
    }

    watery_lib::run()
}
