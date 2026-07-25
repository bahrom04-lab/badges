use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    routing::get,
};
use serde::Deserialize;

use crate::badge::generate_badge;

pub mod badge;
pub mod cli;

#[tokio::main]
async fn main() {
    let router = axum::Router::new().route("/", get(gen_badge));

    let addr = "127.0.0.1:6767";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on: {addr}");
    axum::serve(listener, router).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct Meta {
    instance: String,
    owner: String,
    repo: String,
}

async fn gen_badge(Query(params): Query<Meta>) -> impl IntoResponse {
    let badge_url = generate_badge(params.instance, params.owner, params.repo).await;

    Redirect::to(&badge_url)
}

// Cli Code
// let args = Args::parse();
//
// let owner = args.owner;
// let repo = args.repo;
// let instance = args.instance;
//
// println!("Owner: {owner}");
// println!("Repo: {repo}");
// println!("Host: {instance}");
//
// let badge = generate_badge(instance, owner, repo).await;
// println!("Badge: {badge}");
