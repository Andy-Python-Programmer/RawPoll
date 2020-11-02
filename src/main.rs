#![feature(proc_macro_hygiene, decl_macro)]

use mongodb::options::ClientOptions;
use mongodb::options::ResolverConfig;

use mongodb::sync::Client;

use std::env;

use futures::executor::block_on;

mod routes {
    pub mod index;
    pub mod poll;
}

async fn get_client() -> Client {
    let mongo_url: String = env::var("CLIENT").unwrap();
    let options = ClientOptions::parse_with_resolver_config(mongo_url.as_str(), ResolverConfig::cloudflare()).await;

    let client = Client::with_options(options.unwrap()).unwrap();

    return client;
}

fn main() {
    dotenv::dotenv().ok();

    let routes = rocket::routes![
        routes::index::index,
        routes::poll::post
    ];

    let app: rocket::Rocket = rocket::ignite();
    let client: Client = block_on(get_client());

    let db = client.database("raw_poll");

    app
        .mount("/", routes)
        .manage(db)
        .launch();
}
