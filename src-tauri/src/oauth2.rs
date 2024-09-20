use std::collections::HashMap;

use serde::Deserialize;
use tauri::Url;

use oauth2::basic::*;
use oauth2::*;

#[derive(Deserialize, Debug)]
pub struct Oauth2Cfg {
    pub key: String,
    pub vendor: String,
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub redirect_url: RedirectUrl,
    pub scopes: Vec<Scope>,
    pub csrf: bool,
}

type MyClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

pub struct Oauth2Client {
    client: MyClient,
    scopes: Vec<Scope>,
    csrf: bool,
    verifier: Option<PkceCodeVerifier>,
    access_token: Option<AccessToken>,
    refresh_token: Option<RefreshToken>,
}

impl From<Oauth2Cfg> for Oauth2Client {
    fn from(cfg: Oauth2Cfg) -> Self {
        let client = BasicClient::new(cfg.client_id)
            .set_auth_uri(cfg.auth_url)
            .set_token_uri(cfg.token_url)
            .set_client_secret(cfg.client_secret)
            .set_redirect_uri(cfg.redirect_url);

        Self {
            client,
            scopes: cfg.scopes,
            csrf: cfg.csrf,
            verifier: None,
            access_token: None,
            refresh_token: None,
        }
    }
}

impl Oauth2Client {
    pub fn get_auth_url(&mut self) -> Url {
        let (auth_url, _) = match self.csrf {
            true => {
                let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
                self.verifier = Some(verifier);
                self.client
                    .authorize_url(CsrfToken::new_random)
                    .add_scopes(self.scopes.clone())
                    .set_pkce_challenge(challenge)
                    .url()
            }
            false => self
                .client
                .authorize_url(CsrfToken::new_random)
                .add_scopes(self.scopes.clone())
                .url(),
        };
        auth_url
    }

    pub async fn get_token(
        &mut self,
        auth_code: String,
        proxy: Option<String>,
    ) -> Result<AccessToken, Box<dyn std::error::Error>> {
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

        let token_result = match self.csrf {
            true => {
                self.client
                    .exchange_code(auth_code)
                    // Set the PKCE code verifier.
                    .set_pkce_verifier(self.verifier.take().unwrap())
                    .request_async(&http_client)
                    .await?
            }
            false => {
                self.client
                    .exchange_code(auth_code)
                    .request_async(&http_client)
                    .await?
            }
        };
        let access_token = token_result.access_token();
        self.access_token = Some(access_token.to_owned());
        self.refresh_token = token_result.refresh_token().cloned();
        Ok(access_token.to_owned())
    }

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

pub fn read_oauth2_vendor() -> HashMap<String, Oauth2Cfg> {
    let mut oauth2_map = HashMap::new();
    let oauth2_cfg = include_str!("oauth2.json");
    let oauth2_cfg: Vec<Oauth2Cfg> = serde_json::from_str(oauth2_cfg).unwrap();
    let _: Vec<Option<Oauth2Cfg>> = oauth2_cfg
        .into_iter()
        .map(|oauth2| oauth2_map.insert(oauth2.key.clone(), oauth2))
        .collect();
    oauth2_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let result = read_oauth2_vendor();
        let google_oauth2 = result.get("gmail").unwrap();
        assert!(google_oauth2.csrf);
    }
}
