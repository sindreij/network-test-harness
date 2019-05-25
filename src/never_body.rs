/// Returns the the headers quickly, but then never returns any body
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use futures::compat::Future01CompatExt;
use futures::prelude::*;
use futures::stream::unfold;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Response, Server};
use snafu::Snafu;
use tokio::timer::Delay;

#[derive(Debug, Snafu)]
enum Error {}

pub fn bind(addr: impl Into<SocketAddr>) -> impl Future<Item = (), Error = ()> {
    let addr = addr.into();

    let new_svc = move || service_fn(move |_| response().boxed().compat());

    Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server (slow) error: {}", e))
}

async fn response() -> Result<Response<Body>, Error> {
    let stream = unfold(
        (),
        async move |mut data| -> Option<(Result<Vec<u8>, Error>, ())> {
            let () = future::empty().await;
            Some((Ok(vec![12, 13, 14]), ()))
        },
    );

    Ok(Response::new(Body::wrap_stream(stream.boxed().compat())))
}