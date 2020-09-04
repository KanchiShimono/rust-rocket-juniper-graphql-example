#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod db;
pub mod graphql;
pub mod routes;

use db::Db;
use graphql::resolver::{Mutation, Query};
use routes::Schema;

fn main() {
    rocket::ignite()
        .attach(Db::fairing())
        .manage(Schema::new(Query, Mutation))
        .mount(
            "/",
            routes![
                routes::graphiql,
                routes::get_graphql_handler,
                routes::post_graphql_handler
            ],
        )
        .launch();
}
