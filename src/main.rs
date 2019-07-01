#[macro_use]
extern crate log;
mod my_rss;

use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};
use log::Level;

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();
    let addr = ([0, 0, 0, 0], 3000).into();
    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let rss_data = my_rss::new();
    // Run this server for... forever!
    hyper::rt::run({
        Server::bind(&addr)
            .serve(move || {
                let data = rss_data.clone();
                // service_fn_ok converts our function into a `Service`
                service_fn_ok(move |_: Request<Body>| {
                    Response::new(Body::from((*data.read().unwrap()).clone()))
                })
            })
            .map_err(|e| eprintln!("server error: {}", e))
    });
}
