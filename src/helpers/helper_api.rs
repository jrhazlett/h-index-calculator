use std::error::Error;

use hyper::body;
use hyper::server::conn::Http;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

use crate::helpers::helper_h_index;

const ADDRESS: &str = "127.0.0.1:8001";

pub async fn run_server() -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind(ADDRESS).await?;
    println!("Running server: {}", ADDRESS);

    loop {
        let (item_stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(item_stream, service_fn(handler_request))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DataRequest {
    citations: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DataResponse {
    result: u32,
}

async fn handler_request(
    arg_req: Request<Body>,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync>> {
    println!("Request received.");

    match (arg_req.method(), arg_req.uri().path()) {
        (&Method::GET, "/h_index") => return Ok(Response::new(Body::from("GET received"))),
        (&Method::POST, "/h_index") => {
            let body = arg_req.into_body();

            let vec = match body::to_bytes(body).await {
                Ok(result) => result,
                Err(err) => {
                    return Ok(Response::new(Body::from(format!(
                        "Error: Failed to get bytes from request body; {:?}",
                        err
                    ))))
                }
            }
            .to_vec();

            let string = match String::from_utf8(vec) {
                Ok(result) => result,
                Err(err) => {
                    return Ok(Response::new(Body::from(format!(
                        "Error: Failed to get string from Vec<u8>; {:?}",
                        err
                    ))))
                }
            };

            let data_request: DataRequest = match serde_json::from_str(string.as_str()) {
                Ok(result) => result,
                Err(err) => {
                    return Ok(Response::new(Body::from(format!(
                        "Error: Failed to get DataRequest from json. err = {:?}",
                        err
                    ))))
                }
            };

            let data_response = DataResponse {
                result: helper_h_index::get_h_index(data_request.citations.as_slice()),
            };

            let string_json = match serde_json::to_string(&data_response) {
                Ok(result) => result,
                Err(err) => {
                    return Ok(Response::new(Body::from(format!(
                        "Error: Failed to convert DataRequest to json. err = {:?}",
                        err
                    ))))
                }
            };

            return Ok(Response::new(Body::from(string_json)));
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            return Ok(not_found);
        }
    }
}
