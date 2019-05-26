#[macro_use]
extern crate lazy_static;

use chrono::{Date, Local};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server};
use rss::{ChannelBuilder, Item, ItemBuilder};
use serde_json::Value;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

lazy_static! {
    static ref RSS_DATA: Mutex<String> = Mutex::new(String::new());
}

fn get_item(v: &Value, date: String) -> Item {
    ItemBuilder::default()
        .title(v["title"].to_string())
        .link(format!(
            "https://webapp.bupt.edu.cn/extensions/wap/news/detail.html?id={}&classify_id=tzgg",
            v["id"].as_str().unwrap()
        ))
        .description(v["desc"].to_string())
        .content(v["text"].to_string())
        .author(v["author"].to_string())
        .pub_date(date)
        .build()
        .unwrap()
}

fn get_rss() -> String {
    let body: Value =
        reqwest::get("https://webapp.bupt.edu.cn/extensions/wap/news/get-list.html?p=1&type=tzgg")
            .unwrap()
            .json()
            .unwrap();

    let mut items: Vec<Item> = Vec::new();
    let mut date = Local::today();
    for (_, data) in body["data"].as_object().unwrap() {
        let items_ = data.as_array().unwrap();
        for item_ in items_ {
            let item = get_item(item_, date.to_string());
            items.push(item);
        }
        date = date + chrono::Duration::days(-1);
    }
    ChannelBuilder::default()
        .title("北邮信息门户rss")
        .link("http://my.bupt.edu.cn")
        .description("信息门户rss")
        .items(items)
        .build()
        .unwrap()
        .to_string()
}

fn serve_rss(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(RSS_DATA.lock().unwrap().to_string()))
}

fn main() {
    {
        let mut data = RSS_DATA.lock().unwrap();
        *data = get_rss();
    }
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(300));
        let mut data = RSS_DATA.lock().unwrap();
        *data = get_rss();
    });

    let addr = ([0, 0, 0, 0], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let new_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(serve_rss)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
