use std::collections::HashMap;
use std::ops::Deref;
use parking_lot::Mutex;
use tokio::sync::broadcast;
use tokio::task;

use crate::watery_config::WateryConfig;
use crate::WateryOauth2Cfg;
use crate::WateryOauth2Client;
use crate::WateryOauth2Provider;

#[derive(Default)]
pub struct AppState {
    pub server_handle: Option<task::JoinHandle<()>>,
    pub shutdown_tx: Option<broadcast::Sender<()>>,
}

#[derive(Default)]
pub struct Oauth2State {
    inner: Mutex<HashMap<WateryOauth2Provider, WateryOauth2Client>>,
}

impl Deref for Oauth2State {
    type Target = Mutex<HashMap<WateryOauth2Provider, WateryOauth2Client>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Oauth2State {
    pub fn from_config(cfg: HashMap<WateryOauth2Provider, WateryOauth2Cfg>) -> Self {
        let mut map: HashMap<WateryOauth2Provider, WateryOauth2Client> = HashMap::new();
        let _: Vec<Option<WateryOauth2Client>> = cfg
            .into_iter()
            .map(|(key, value)| map.insert(key, value.into()))
            .collect();
        Oauth2State {
            inner: Mutex::new(map),
        }
    }
}


#[derive(Default)]
pub struct WateryConfigState {
    inner: Mutex<WateryConfig>,
}

impl Deref for WateryConfigState {
    type Target = Mutex<WateryConfig>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<WateryConfig> for WateryConfigState {
    fn from(cfg: WateryConfig) -> WateryConfigState {
        WateryConfigState {
            inner: Mutex::new(cfg)
        }
    }
}