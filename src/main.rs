#![feature(proc_macro_hygiene, decl_macro)]

use rocket::fairing::AdHoc;

mod routes {
    pub mod public;

    pub mod index;
    pub mod create;
    pub mod vote;
    pub mod poll;
    
    pub mod api {
        pub mod poll;
        pub mod vote;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let routes = rocket::routes![
        routes::public::get,

        routes::index::get,
        routes::create::get,
        routes::vote::get,
        routes::poll::get,

        routes::api::poll::post,
        routes::api::poll::get,

        routes::api::vote::post
    ];

    let app: rocket::Rocket = rocket::ignite();

    let mut db: dino::Database = dino::Database::new("polls.json");

    db.load();

    app
        .mount("/", routes)
        .manage(db)
        .attach(AdHoc::on_launch("Socket Server", |_| {
            println!("Rocket is launched! Loading Database Was Success");
        }))
        .launch();

    Ok(())
}