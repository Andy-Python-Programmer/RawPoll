#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod poll;
mod templating;

use rocket::{ State };

use rocket_contrib::json::Json;
use rocket_contrib::templates::{ Template };

use templating::{ Templating, PollTemplate };
use templating::{ File, Page };

use poll::Poll;

#[get("/")]
fn index() -> Page {
    return Templating::new().render("index.html");
}

#[get("/api/poll/<id>/<choice>")]
fn api_vote(database: State<sled::Db>, id: String, choice: String) -> Json<PollTemplate> {
    let tree = Poll::tree(&database, &id);
    let mut result = Poll::from(&database, &id);

    println!("Title: {:?}", result.title);
    println!("Description: {:?}", result.description);
    println!("Options: {:?}", result.options);

    println!("Option Current: {:?}", result.options.get(&choice));

    *result.options.get_mut(&choice).unwrap() += 1;

    println!("Option After: {:?}", result.options);

    let parsed = Poll::to(result.options);

    println!("Parsed: {:?}", parsed);

    tree.insert("options", parsed.as_str()).unwrap();


    return Json(
        PollTemplate {
            title: result.title,
            description: result.description,
            options: parsed,
        }
    );
}

#[get("/api/poll/<id>")]
fn api_list(database: State<sled::Db>, id: String) -> Json<PollTemplate> {
    let result = Poll::from(&database, &id);
    let parsed = Poll::to(result.options);

    let context = PollTemplate {
        title: result.title,
        description: result.description,
        options: parsed
    };

    return Json(context);
}

#[get("/poll/<_id>")]
fn vote(_id: String) -> Page {
    return Templating::new().render("poll.html");
}

#[get("/static/<file..>")]
fn static_files(file: File) -> Page {
    return Templating::new().render_static(file);
}

fn main() {
    let db: sled::Db = sled::open("polls").expect("open");
    let value: sled::Tree = db.open_tree("id").unwrap();

    value.insert("title", "Amazing title!").unwrap();
    value.insert("description", "Amazing description!").unwrap();
    value.insert("options", "a: 1, b: 1, c: 1").unwrap();
    
    let app = rocket::ignite()
        .mount("/", routes![static_files, index, api_vote, api_list, vote])
        .attach(Template::fairing())
        .manage(db);

    app.launch();
}
