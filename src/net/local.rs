use std::collections::HashMap;

// vim: fdm=indent fdn=1
use super::remote::broadcast;
use hyper::{Method, Request, Response, StatusCode};

pub async fn process(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<String>, hyper::Error> {
    println!("+ {} {}", req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new("Overheard Node\r\n".to_string())),
        (&Method::GET, "/join") => Ok(Response::new("Joined\r\n".to_string())),
        (&Method::GET, "/broadcast") => broadcast_cmd(req).await,
        _ => {
            let mut response = Response::new("Not Found".to_string());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}

pub async fn broadcast_cmd(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<String>, hyper::Error> {
    let params = req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    let data = params.get("data").unwrap_or(&String::from("")).clone();
    let _ = broadcast(data.into_bytes().to_vec()).await;
    Ok(Response::new("Broadcast sent\r\n".to_string()))
}
