#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

#[macro_use] extern crate mysql;

use rocket_contrib::{Json};
use rocket::{State};

mod hero;
use hero::{Hero};

mod db;

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>, connection: State<mysql::Pool>) -> Json<Hero> {
    let hero = hero.into_inner();

    let insert = Hero {
        id: None,
        name: hero.name,
        identity: hero.identity,
        hometown: hero.hometown,
        age: hero.age
    };

    Json(Hero::create(insert, connection))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/hero", routes![create])
        .launch();
}
