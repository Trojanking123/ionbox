use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tauri::Url;

use oauth2::basic::*;
use oauth2::*;

use crate::{WateryError, WateryResult};

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum WateryOauth2Provider {
    Google,
    Outlook,
    Other(String),
}

impl ToString for WateryOauth2Provider {
    fn to_string(&self) -> String {
        match self {
            WateryOauth2Provider::Google => "Google".to_string(),
            WateryOauth2Provider::Outlook => "Outlook".to_string(),
            WateryOauth2Provider::Other(value) => value.clone(),
        }
    }
}

// 实现 String 到 WateryOauth2Provider 的转换
impl From<String> for WateryOauth2Provider {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Google" => WateryOauth2Provider::Google,
            "Outlook" => WateryOauth2Provider::Outlook,
            _ => WateryOauth2Provider::Other(value),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct WateryOauth2Cfg {
    pub provider: WateryOauth2Provider,
    pub client_id: ClientId,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub redirect_url: RedirectUrl,
    pub scopes: Vec<Scope>,
    pub csrf: bool,
    pub client_secret: Option<ClientSecret>,
}

type MyClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

#[derive(Clone)]
pub struct WateryOauth2Client {
    provider: WateryOauth2Provider,
    client: MyClient,
    scopes: Vec<Scope>,
    csrf: bool,
    access_token: Option<AccessToken>,
    refresh_token: Option<RefreshToken>,
}

impl From<WateryOauth2Cfg> for WateryOauth2Client {
    fn from(cfg: WateryOauth2Cfg) -> Self {
        let mut client = BasicClient::new(cfg.client_id)
            .set_auth_uri(cfg.auth_url)
            .set_token_uri(cfg.token_url)
            .set_redirect_uri(cfg.redirect_url);
        if let Some(secret) = cfg.client_secret {
            client = client.set_client_secret(secret);
        };

        Self {
            provider: cfg.provider,
            client,
            scopes: cfg.scopes,
            csrf: cfg.csrf,
            access_token: None,
            refresh_token: None,
        }
    }
}

impl WateryOauth2Client {
    // client side only
    pub fn get_auth_url(&mut self) -> (Url, CsrfToken, Option<PkceCodeVerifier>) {
        let mut verifier = None;
        let (auth_url, oauth2_state) = match self.csrf {
            true => {
                let (challenge, veri) = PkceCodeChallenge::new_random_sha256();
                verifier = Some(veri);
                self.client
                    .authorize_url(CsrfToken::new_random)
                    .add_scopes(self.scopes.clone())
                    //.add_extra_param("provider", self.provider.to_string())
                    .set_pkce_challenge(challenge)
                    .url()
            }
            false => self
                .client
                .authorize_url(CsrfToken::new_random)
                .add_scopes(self.scopes.clone())
                .url(),
        };
        (auth_url, oauth2_state, verifier)
    }

    // server side only
    pub async fn get_token(
        &self,
        auth_code: String,
        proxy: Option<String>,
        verifier: Option<String>,
    ) -> WateryResult<(Option<AccessToken>, Option<RefreshToken>)> {
        let auth_code = AuthorizationCode::new(auth_code);
        let mut http_client = reqwest::ClientBuilder::new();
        if let Some(proxy) = proxy {
            let proxy = reqwest::Proxy::https(proxy).unwrap();
            http_client = http_client.proxy(proxy);
        };
        // Following redirects opens the client up to SSRF vulnerabilities.
        let http_client = http_client
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let mut client = self.client.exchange_code(auth_code);
        if let Some(inner) = verifier {
            let veri = PkceCodeVerifier::new(inner);
            client = client.set_pkce_verifier(veri);
        }

        let token_result = client
            .request_async(&http_client)
            .await
            .map_err(|_| WateryError::AuthConnectionFailed)?;

        let access_token = token_result.access_token();
        Ok((
            Some(access_token.to_owned()),
            token_result.refresh_token().cloned(),
        ))
    }

    // client side only
    pub async fn refresh_token(
        &mut self,
        proxy: Option<String>,
    ) -> Result<AccessToken, Box<dyn std::error::Error>> {
        let mut http_client = reqwest::ClientBuilder::new();
        if let Some(proxy) = proxy {
            let proxy = reqwest::Proxy::https(proxy).unwrap();
            http_client = http_client.proxy(proxy);
        };
        // Following redirects opens the client up to SSRF vulnerabilities.
        let http_client = http_client
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");
        let token_result = self
            .client
            .exchange_refresh_token(self.refresh_token.as_ref().unwrap())
            .request_async(&http_client)
            .await?;
        let access_token = token_result.access_token();
        self.access_token = Some(access_token.to_owned());
        if let Some(refresh_token) = token_result.refresh_token() {
            self.refresh_token = Some(refresh_token.to_owned())
        }
        Ok(access_token.to_owned())
    }
}

pub fn read_oauth2_provider() -> HashMap<WateryOauth2Provider, WateryOauth2Cfg> {
    let mut oauth2_map = HashMap::new();
    let oauth2_cfg = include_str!("oauth2.json");
    let oauth2_cfg: Vec<WateryOauth2Cfg> = serde_json::from_str(oauth2_cfg).unwrap();
    let _: Vec<Option<WateryOauth2Cfg>> = oauth2_cfg
        .into_iter()
        .map(|oauth2| oauth2_map.insert(oauth2.provider.clone(), oauth2))
        .collect();
    oauth2_map
}

#[cfg(test)]
mod test {
    use crate::Oauth2State;

    use super::*;

    #[test]
    fn test_add() {
        let result = read_oauth2_provider();
        let client = Oauth2State::from_config(result);
        let mut client = client.lock();
        let provider: WateryOauth2Provider = "google".to_string().into();
        let client = client.get_mut(&provider).unwrap();
        let auth_url = client.get_auth_url();
        dbg!(auth_url);
    }
}
