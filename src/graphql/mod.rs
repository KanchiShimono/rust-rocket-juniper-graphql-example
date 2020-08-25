use crate::db::{models, Db};
use juniper::FieldResult;
use rocket::request::{FromRequest, Outcome, Request};

pub mod schema;

pub struct Context {
    pub conn: Db,
}

impl juniper::Context for Context {}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let conn = request.guard::<Db>()?;
        Outcome::Success(Context { conn })
    }
}

pub struct Query;
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Query {
    fn persons(context: &Context) -> FieldResult<Vec<schema::Person>> {
        let result = models::PersonWithPost::find_all(&context.conn)?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    fn posts(context: &Context) -> FieldResult<Vec<schema::Post>> {
        let result = models::Post::find_all(&context.conn)?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }
}

#[juniper::object(Context = Context)]
impl Mutation {}
