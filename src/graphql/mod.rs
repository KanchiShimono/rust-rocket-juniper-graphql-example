use crate::db::{
    repository::{
        PersonRepository, PersonWithPostsRepository, PgPersonRepository,
        PgPersonWithPostsRepository, PgPostRepository, PostRepository,
    },
    Db,
};
use juniper::{FieldError, FieldResult};
use rocket::request::{FromRequest, Outcome, Request};
use schema::{CreatePersonInput, CreatePostInput};
use uuid::Uuid;

pub mod schema;

pub struct Context {
    pub person_repo: PgPersonRepository,
    pub post_repo: PgPostRepository,
    pub person_with_posts_repo: PgPersonWithPostsRepository,
}

impl juniper::Context for Context {}

impl<'a, 'r> FromRequest<'a, 'r> for Context {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let person_repo = PgPersonRepository {
            conn: request.guard::<Db>()?,
        };
        let post_repo = PgPostRepository {
            conn: request.guard::<Db>()?,
        };
        let person_with_posts_repo = PgPersonWithPostsRepository {
            conn: request.guard::<Db>()?,
        };
        Outcome::Success(Context {
            person_repo,
            post_repo,
            person_with_posts_repo,
        })
    }
}

pub struct Query;
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Query {
    fn all_persons(context: &Context) -> FieldResult<Vec<schema::Person>> {
        let result = context
            .person_repo
            .find_all()
            .map_err(|e| FieldError::from(e))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    fn all_person_with_posts(context: &Context) -> FieldResult<Vec<schema::PersonWithPosts>> {
        let result = context
            .person_with_posts_repo
            .find_all()
            .map_err(|e| FieldError::from(e))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    fn person(context: &Context, id: Uuid) -> FieldResult<schema::Person> {
        context
            .person_repo
            .find_by_id(id)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn person_with_posts(context: &Context, id: Uuid) -> FieldResult<schema::PersonWithPosts> {
        context
            .person_with_posts_repo
            .find_by_id(id)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn all_posts(context: &Context) -> FieldResult<Vec<schema::Post>> {
        let result = context
            .post_repo
            .find_all()
            .map_err(|e| FieldError::from(e))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    fn posts(context: &Context, person_id: Uuid) -> FieldResult<Vec<schema::Post>> {
        let result = context
            .post_repo
            .find_by_person_id(person_id)
            .map_err(|e| FieldError::from(e))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    fn post(context: &Context, id: Uuid) -> FieldResult<schema::Post> {
        context
            .post_repo
            .find_by_id(id)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }
}

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_person(context: &Context, input: CreatePersonInput) -> FieldResult<schema::Person> {
        context
            .person_repo
            .save(input)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn delete_person(context: &Context, id: Uuid) -> FieldResult<schema::Person> {
        context
            .person_repo
            .delete(id)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn create_post(context: &Context, input: CreatePostInput) -> FieldResult<schema::Post> {
        context
            .post_repo
            .save(input)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn delete_post(context: &Context, id: Uuid) -> FieldResult<schema::Post> {
        context
            .post_repo
            .delete(id)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }
}
