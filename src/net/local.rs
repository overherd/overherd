// vim: fdm=indent fdn=1
// use super::remote::broadcast;
use hyper::{Method, Request, Response, StatusCode};

pub async fn process(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<String>, hyper::Error> {
    println!("Received {} request to {}", req.method(), req.uri().path());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => Ok(Response::new("Overheard Node\r\n".to_string())),
        (&Method::GET, "/join") => Ok(Response::new("Joined\r\n".to_string())),
        _ => {
            let mut response = Response::new("Not Found".to_string());
            *response.status_mut() = StatusCode::NOT_FOUND;
            Ok(response)
        }
    }
}
