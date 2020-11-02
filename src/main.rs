#![feature(proc_macro_hygiene, decl_macro)]

use mongodb::options::ClientOptions;
use mongodb::options::ResolverConfig;

use mongodb::sync::Client;

use futures::executor::block_on;

mod routes {
    pub mod index;
    pub mod poll;
}

async fn get_client() -> Client {
    let mongo_url: &str = dotenv_codegen::dotenv!("CLIENT");
    let options = ClientOptions::parse_with_resolver_config(mongo_url, ResolverConfig::cloudflare()).await;

    let client = Client::with_options(options.unwrap()).unwrap();

    return client;
}

fn main() {
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
