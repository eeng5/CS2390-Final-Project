// #![feature(plugin)]
// #![plugin(rocket_codegen)]
//
// extern crate rocket;

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

// use serde_json::Value;
// use rocket::config::Value;
use rocket_contrib::json::{Json, JsonValue};

mod db;
mod schema;

mod hero;
use hero::{Hero};

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
    let insert = Hero { id: None, ..hero.into_inner() };
    Json(Hero::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> JsonValue {
    Json(json!(Hero::read(&connection)))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> JsonValue {
    let update = Hero { id: Some(id), ..hero.into_inner() };
    Json(json!({
        "success": Hero::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> JsonValue {
    Json(json!({
        "success": Hero::delete(id, &connection)
    }))
}

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .launch();
}
