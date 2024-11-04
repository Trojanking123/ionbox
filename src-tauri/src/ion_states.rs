use parking_lot::Mutex;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task;

use crate::ion_config::IonConfig;
use crate::ion_config::IonConfigWrap;
use crate::oauth2::IonOauth2Cfg;
use crate::oauth2::IonOauth2Client;
use crate::oauth2::IonOauth2Provider;

#[derive(Default)]
pub struct AppState {
    pub server_handle: Option<task::JoinHandle<()>>,
    pub shutdown_tx: Option<broadcast::Sender<()>>,
}

#[derive(Default)]
pub struct Oauth2State {
    inner: Mutex<HashMap<IonOauth2Provider, IonOauth2Client>>,
}

impl Deref for Oauth2State {
    type Target = Mutex<HashMap<IonOauth2Provider, IonOauth2Client>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Oauth2State {
    pub fn from_config(cfg: HashMap<IonOauth2Provider, IonOauth2Cfg>) -> Self {
        let mut map: HashMap<IonOauth2Provider, IonOauth2Client> = HashMap::new();
        let _: Vec<Option<IonOauth2Client>> = cfg
            .into_iter()
            .map(|(key, value)| map.insert(key, value.into()))
            .collect();
        Oauth2State {
            inner: Mutex::new(map),
        }
    }
}

#[derive(Default, Clone)]
pub struct IonConfigState {
    inner: IonConfigWrap,
}

impl IonConfigState {
    pub fn inner_clone(&self) -> IonConfigWrap {
        self.inner.clone()
    }
}

impl Deref for IonConfigState {
    type Target = IonConfigWrap;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<IonConfig> for IonConfigState {
    fn from(cfg: IonConfig) -> IonConfigState {
        IonConfigState {
            inner: Arc::new(RwLock::new(cfg)),
        }
    }
}
