// vim: fdm=indent fdn=1
use super::remote::broadcast;
use crate::net::gossip::refresh_peers;
use crate::net::list::update_peer_list;
use hyper::{Method, Request, Response, StatusCode};
use std::collections::HashMap;
use tokio::net::lookup_host;

pub async fn process(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<String>, hyper::Error> {
    println!("+ {} {}", req.method(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new("Overherd Node\r\n".to_string())),
        (&Method::GET, "/join") => join_cmd(req).await,
        (&Method::GET, "/broadcast") => broadcast_cmd(req).await,
        _ => {
            let mut response = Response::new("Not Found\r\n".to_string());
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

    let data = match params.get("data") {
        Some(d) if !d.is_empty() => d,
        _ => {
            let mut response = Response::new("No data to send\r\n".to_string());
            *response.status_mut() = StatusCode::BAD_REQUEST;
            return Ok(response);
        }
    };
    let _ = broadcast(data.as_bytes().to_vec()).await;
    Ok(Response::new("Broadcast sent\r\n".to_string()))
}

pub async fn join_cmd(
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
    let peer = match params.get("peer") {
        Some(p) => p,
        _ => {
            let mut response = Response::new("No peer given\r\n".to_string());
            *response.status_mut() = StatusCode::BAD_REQUEST;
            return Ok(response);
        }
    };

    let mut addrs = lookup_host(format!("{}:8080", peer)).await.unwrap();
    if let Some(addr) = addrs.next() {
        let _ = update_peer_list(&[addr.ip().to_string()]).await;
    }

    refresh_peers().await;

    Ok(Response::new(
        format!("Joining peer \"{}\"\r\n", peer).to_string(),
    ))
}
