use juniper::RootNode;
use rocket::response::content;
use rocket::State;
use rocket::{get, post};

use crate::graphql::{
    context::Context,
    resolver::{Mutation, Query},
};

pub type Schema = RootNode<'static, Query, Mutation>;

#[get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
    context: Context,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: Context,
    request: juniper_rocket::GraphQLRequest,
    schema: State<Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}
