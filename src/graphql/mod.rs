use crate::db::{models, Db};
use juniper::{FieldError, FieldResult};
use rocket::request::{FromRequest, Outcome, Request};
use schema::{CreatePersonInput, CreatePostInput};

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
        let result = models::PersonWithPost::find_all(&context.conn)
            .map_err(|e| FieldError::from(e))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    fn posts(context: &Context) -> FieldResult<Vec<schema::Post>> {
        let result = models::Post::find_all(&context.conn)
            .map_err(|e| FieldError::from(e))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }
}

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_person(context: &Context, input: CreatePersonInput) -> FieldResult<schema::Person> {
        models::Person::save(&context.conn, input)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn create_post(context: &Context, input: CreatePostInput) -> FieldResult<schema::Post> {
        models::Post::save(&context.conn, input)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }
}
