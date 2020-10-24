#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate uuid;

mod templating;

use uuid::Uuid;

use rocket::{ State };

use rocket_contrib::json::Json;
use rocket_contrib::templates::{ Template };

use templating::{ Templating, PollNew };
use templating::{ File, Page };

#[get("/")]
fn index() -> Page {
    return Templating::new().render("index.html");
}

#[get("/api/poll/<id>/<choice>")]
fn api_vote(database: State<dino::Database>, id: String, choice: String) -> String {
    let mut tree = database.find(id.as_str()).unwrap();
    let mut choices = tree.find("options").unwrap();

    println!("Title: {:?}", tree.find("title").unwrap());
    println!("Description: {:?}", tree.find("description").unwrap());
    println!("Options: {}", tree.find("options").unwrap().to_string());

    let cur_opts = str::replace(choices.find(&choice).unwrap().to_string().as_str(), '"', "").parse::<usize>().unwrap();

    choices.insert(&choice, (cur_opts + 1).to_string().as_str());

    tree.insert_tree("options", choices);
    database.insert_tree(id.as_str(), tree);

    return database.find(id.as_str()).unwrap().to_string();
}

#[get("/api/poll/<id>")]
fn api_list(database: State<dino::Database>, id: String) -> String {
    return database.find(id.as_str()).unwrap().to_string();
}

#[get("/poll/<_id>")]
fn vote(_id: String) -> Page {
    return Templating::new().render("poll.html");
}

#[get("/static/<file..>")]
fn static_files(file: File) -> Page {
    return Templating::new().render_static(file);
}

#[get("/api/new?<title>&<description>&<options>")]
fn api_new(database: State<dino::Database>, title: String, description: String, options: String) -> Json<PollNew> {
    let id = Uuid::new_v4();

    let mut value: dino::Tree = dino::Tree::new();
    println!("{}", ("{".to_string() + &options + &"}".to_string()).as_str());
    let data: dino::Tree = dino::Tree::from(("{".to_string() + &options + &"}".to_string()).as_str());

    value.insert("title", title.as_str());
    value.insert("description", description.as_str());
    value.insert_tree("options", data);

    database.insert_tree(id.to_string().as_str(), value);

    let context = PollNew {
        id: id.to_string()
    };

    println!("UUID: {}", id.to_string());

    return Json(context);
}

#[get("/new")]
fn new_poll() -> Page {
    return Templating::new().render("new.html");
}

fn main() {
    let mut db: dino::Database = dino::Database::new("polls.dino");
    db.load();

    let mut value: dino::Tree = dino::Tree::new();
    let mut data: dino::Tree = dino::Tree::new();

    data.insert("a", "1");
    data.insert("b", "1");
    data.insert("c", "1");

    value.insert("title", "Amazing title!");
    value.insert("description", "Amazing description!");
    value.insert_tree("options", data);

    db.insert_tree("id", value);
    
    let app = rocket::ignite()
        .mount("/", routes![
            static_files, 
            index, 
            api_vote, vote, 
            api_list,
            api_new, new_poll
        ])
        .attach(Template::fairing())
        .manage(db);

    app.launch();
}
