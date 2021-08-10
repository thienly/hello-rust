use async_std::{ task};
use tokio::sync::{mpsc};
use crate::models::Pokemons;
use std::sync::Arc;
use parking_lot::lock_api::RwLock;
use std::collections::HashMap;

mod models;
mod api;
mod chat;

#[tokio::main]
async  fn main() {
    api::run().await;
}