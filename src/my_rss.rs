//
// my_rss.rs
// Copyright (C) 2019 zekin <zekin@DESKTOP-UR3A57I>
// Distributed under terms of the MIT license.
//

use chrono::Local;
use rss::{ChannelBuilder, Item, ItemBuilder};
use serde_json::Value;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

type MyRss = Arc<RwLock<String>>;

fn get_rss() -> String {
    let body: Value = match reqwest::get(
        "https://webapp.bupt.edu.cn/extensions/wap/news/get-list.html?p=1&type=tzgg",
    ) {
        Ok(mut data) => data.json().unwrap(),
        Err(e) => return e.to_string(),
    };

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

pub fn new() -> MyRss {
    let new_rss = Arc::new(RwLock::new(String::new()));
    let clone_rss = new_rss.clone();
    thread::spawn(move || loop {
        println!("sync start!");
        *clone_rss.write().unwrap() = get_rss();
        println!("sync done!");
        thread::sleep(Duration::from_secs(300));
    });
    new_rss
}
