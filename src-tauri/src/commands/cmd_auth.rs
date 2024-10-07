use std::time::Duration;

use backon::BackoffBuilder;
use backon::ConstantBuilder;
use backon::ExponentialBuilder;
use backon::Retryable;
use reqwest::Client;
use tauri::{State, Url};

use crate::localserver::Tokens;
use crate::LOCAL_ADDR;
use crate::{IonError, IonOauth2Provider, IonResult, Oauth2State};

#[tauri::command]
pub fn get_provider_link(
    provider: String,
    auth: State<Oauth2State>,
) -> IonResult<(String, String, Option<String>)> {
    let provider: IonOauth2Provider = provider.into();
    let mut auth = auth.lock();
    let client = auth.get_mut(&provider).ok_or(IonError::NoSuchProvider)?;
    let (url, csrf_token, veri) = client.get_auth_url();
    Ok((
        url.to_string(),
        csrf_token.into_secret(),
        veri.map(|v| v.into_secret()),
    ))
}

#[tauri::command]
pub async fn register(state: String, provider: String, verifier: Option<String>) -> IonResult<()> {
    let url = format!("http://{:?}", LOCAL_ADDR);
    let mut url = Url::parse(url.as_str()).unwrap();
    url.set_path("/register");
    let mut query_params = format!("state={:?}&provider={:?}", state, provider);
    if let Some(veri) = verifier {
        query_params = format!("{query_params}&verifier={veri}");
    };
    url.set_query(Some(&query_params));
    let client = reqwest::Client::new();
    let _ = client
        .post(url.as_str())
        .send()
        .await
        .map_err(|e| IonError::LocalServerConnectionError(e.to_string()))?
        .text()
        .await
        .map_err(|e| IonError::LocalServerConnectionError(e.to_string()))?;
    Ok(())
}

async fn poll_tokens(clinet: &Client, url: &Url) -> IonResult<Tokens> {
    let token = clinet
        .get(url.as_str())
        .send()
        .await
        .map_err(|e| IonError::LocalServerConnectionError(e.to_string()))?
        .json::<Tokens>()
        .await
        .map_err(|e| IonError::LocalServerConnectionError(e.to_string()))?;

    Ok(token)
}

#[tauri::command]
pub async fn poll(state: String) -> IonResult<Tokens> {
    let url = format!("http://{:?}", LOCAL_ADDR);
    let mut url = Url::parse(url.as_str()).unwrap();
    url.set_path("/tokens");
    let query_params = format!("state={:?}", state);
    url.set_query(Some(&query_params));

    let client = reqwest::Client::new();

    let backoff = ConstantBuilder::default()
        .with_delay(Duration::from_millis(500))
        .with_max_times(10);

    let tokens = (|| async { poll_tokens(&client, &url).await })
        .retry(backoff)
        .await?;

    Ok(tokens)
}