// use std::collections::HashMap;

// use http::Uri;
// use warp::*;

// async fn local_server() {
//     let login_route = warp::path("login")
//         .map(|| warp::redirect::temporary(Uri::from_static("https://oauth2-provider.com/auth")));

//     let callback_route = warp::path("callback")
//         .and(warp::query::<HashMap<String, String>>())
//         .and_then(move |params: HashMap<String, String>| {
//             let proxy = reqwest::Proxy::https("http://127.0.0.1:10006").unwrap();
//             let client = reqwest::Client::builder().proxy(proxy).build().unwrap();
//             let mut accese_token = String::new();
//             let mut refresh_token = String::new();
//             async move {
//                 if let Some(token) = params.get("code") {
//                     println!("{token}");
//                     let form = [
//                         ("client_id", GOOGLE_CLIENT_ID),
//                         ("client_secret", GOOGLE_CLIENT_SECRET),
//                         ("code", token),
//                         ("redirect_uri", GOOGLE_REDIRECT_URI),
//                         ("grant_type", "authorization_code"),
//                     ];
//                     let resp = client
//                         .post(GOOGLE_AUTH_URL)
//                         .form(&form)
//                         .send()
//                         .await
//                         .unwrap();
//                     println!("response {:?}", resp);
//                     let res: GoogleResp = resp.json().await.unwrap();
//                     println!("res: {:?}, res", res);
//                     accese_token = res.access_token;
//                     refresh_token = res.refresh_token;
//                 }
//                 let redirect_uri = Uri::from_str(
//                     format!("watery://accese_token={accese_token}&refresh_token={refresh_token}")
//                         .as_str(),
//                 )
//                 .unwrap();
//                 //Ok(warp::redirect::temporary(redirect_uri))
//                 Ok(warp::redirect::temporary(redirect_uri)) as Result<_, warp::Rejection>
//             }
//         });

//     let routes = login_route.or(callback_route);

//     let (addr, server) =
//         warp::serve(routes).bind_with_graceful_shutdown(([127, 0, 0, 1], PORT), async move {
//             shutdown_rx.recv().await.ok(); // 等待关闭信号
//         });

//     let handle = tokio::task::spawn(server);
// }
