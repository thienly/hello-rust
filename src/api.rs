use warp::{http, Filter};
use parking_lot::RwLock;
use std::{
    sync::Arc,
    collections::HashMap,
    env
};
use crate::models::{Pokemons, Pokemon};
use std::str::FromStr;
use std::net::SocketAddr;
use tokio::sync::{mpsc, oneshot};
use std::borrow::Borrow;
use tokio::sync::mpsc::Sender;
use serde;
use crate::{chat};

async fn update_store_list(
    item: Pokemon,
    store: (Pokemons, Sender<Pokemon>)
) -> Result<impl warp::Reply, warp::Rejection> {
    store.0.items.write().insert(item.name.clone(), item.clone());
    let mut sender = store.1.clone();
    let data = Pokemon::new(&item.name, &item.color, item.eye_num, item.nose_num, item.mouth_num);
    sender.try_send(data);
    Ok(warp::reply::with_status(
        "Added items to the list",
        http::StatusCode::CREATED,
    ))
}

async fn get_store_list(
    store: (Pokemons, Sender<Pokemon>)
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut data = Vec::new();
    let r = store.0.items.read();
    for (key,value) in r.iter() {
        data.push(value);
    }
    Ok(warp::reply::json(&data))
}

fn post_json() -> impl Filter<Extract = (Pokemon,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024*16).and(warp::body::json())
}

pub async fn run() {

    let (tx, mut rx) = mpsc::channel::<Pokemon>(36);
    let store = Pokemons::new();
    let store2 = store.clone();
    let store_filter = warp::any().map(move || (store.clone(), tx.clone()));

    tokio::spawn(async move {
        chat::run(&mut rx, store2.clone()).await
    });

    let get_items = warp::get()
        .and(warp::path("pokemons"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_store_list);

    let add_items = warp::post()
        .and(warp::path("pokemons"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_store_list);

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = add_items.or(get_items).or(hello);

    let port = env::var("PORT");
    if port.is_err(){
        panic!("please provide port env")
    }
    let port_number = i16::from_str( &port.unwrap()).unwrap();
    let result = SocketAddr::from_str(&format!("127.0.0.1:{}",port_number));

    warp::serve(routes)
        .run(result.unwrap()).await;
}

