use crate::db::repository::{PersonRepository, PersonWithPostsRepository, PostRepository};
use crate::graphql::{
    context::Context,
    schema::{CreatePersonInput, CreatePostInput, Person, PersonWithPosts, Post},
};
use juniper::{FieldError, FieldResult};
use uuid::Uuid;

pub struct Query;
pub struct Mutation;

#[juniper::object(Context = Context)]
impl Query {
    fn all_persons(context: &Context) -> FieldResult<Vec<Person>> {
        let result = context
            .person_repo
            .find_all()
            .map_err(|e| FieldError::from(e))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    fn all_person_with_posts(context: &Context) -> FieldResult<Vec<PersonWithPosts>> {
        let result = context
            .person_with_posts_repo
            .find_all()
            .map_err(|e| FieldError::from(e))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    fn person(context: &Context, id: Uuid) -> FieldResult<Person> {
        context
            .person_repo
            .find_by_id(id)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn person_with_posts(context: &Context, id: Uuid) -> FieldResult<PersonWithPosts> {
        context
            .person_with_posts_repo
            .find_by_id(id)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn all_posts(context: &Context) -> FieldResult<Vec<Post>> {
        let result = context
            .post_repo
            .find_all()
            .map_err(|e| FieldError::from(e))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    fn posts(context: &Context, person_id: Uuid) -> FieldResult<Vec<Post>> {
        let result = context
            .post_repo
            .find_by_person_id(person_id)
            .map_err(|e| FieldError::from(e))?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    fn post(context: &Context, id: Uuid) -> FieldResult<Post> {
        context
            .post_repo
            .find_by_id(id)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }
}

#[juniper::object(Context = Context)]
impl Mutation {
    fn create_person(context: &Context, input: CreatePersonInput) -> FieldResult<Person> {
        context
            .person_repo
            .save(input)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn delete_person(context: &Context, id: Uuid) -> FieldResult<Person> {
        context
            .person_repo
            .delete(id)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn create_post(context: &Context, input: CreatePostInput) -> FieldResult<Post> {
        context
            .post_repo
            .save(input)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }

    fn delete_post(context: &Context, id: Uuid) -> FieldResult<Post> {
        context
            .post_repo
            .delete(id)
            .map(Into::into)
            .map_err(|e| FieldError::from(e))
    }
}
