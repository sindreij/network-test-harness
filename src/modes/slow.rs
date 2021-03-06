/// Returns the response with the body, but wait some time before delivering it
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use futures::compat::Future01CompatExt;
use futures::{FutureExt, TryFutureExt};
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server};
use log::{error, info};
use snafu::{ResultExt, Snafu};
use tokio::timer::Delay;

#[derive(Debug, Snafu)]
enum Error {
    Delaying { source: tokio::timer::Error },
}

pub fn bind(data: &[u8], addr: impl Into<SocketAddr>) -> impl Future<Item = (), Error = ()> {
    let addr = addr.into();
    info!("Listening on {}", addr);

    let data = data.to_owned();
    let new_svc = move || {
        let data = data.clone();
        service_fn(move |request| {
            info!("{} {}", request.method(), request.uri());
            response(data.clone()).boxed().compat()
        })
    };

    Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| error!("server error: {}", e))
}

async fn response(data: Vec<u8>) -> Result<Response<Body>, Error> {
    Delay::new(Instant::now() + Duration::from_secs(9))
        .compat()
        .await
        .context(Delaying)?;

    Ok(Response::new(Body::from(data)))
}
