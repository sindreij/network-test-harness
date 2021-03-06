/// Healthy response, this just returns what it should
use std::net::SocketAddr;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
use log::{error, info};

pub fn bind(data: &[u8], addr: impl Into<SocketAddr>) -> impl Future<Item = (), Error = ()> {
    // This is our socket address...
    let addr = addr.into();
    info!("Listening on {}", addr);

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let data = data.to_owned();
    let new_svc = move || {
        // service_fn_ok converts our function into a `Service`
        let data = data.clone();
        service_fn_ok(move |request| {
            info!("{} {}", request.method(), request.uri());
            Response::new(Body::from(data.clone()))
        })
    };

    Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| error!("server error: {}", e))
}
