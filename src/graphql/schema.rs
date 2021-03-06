use crate::db::models;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(GraphQLObject)]
pub struct Person {
    id: Uuid,
    name: String,
    create_at: NaiveDateTime,
    update_at: NaiveDateTime,
}

#[derive(GraphQLObject)]
pub struct Post {
    id: Uuid,
    person_id: Uuid,
    text: String,
    create_at: NaiveDateTime,
    update_at: NaiveDateTime,
}

#[derive(GraphQLObject)]
pub struct PersonWithPosts {
    id: Uuid,
    name: String,
    posts: Vec<Post>,
    create_at: NaiveDateTime,
    update_at: NaiveDateTime,
}

#[derive(GraphQLInputObject)]
pub struct CreatePersonInput {
    pub name: String,
}

#[derive(GraphQLInputObject)]
pub struct CreatePostInput {
    pub person_id: Uuid,
    pub text: String,
}

impl Into<Person> for models::Person {
    fn into(self) -> Person {
        Person {
            id: self.id,
            name: self.name,
            create_at: self.create_at,
            update_at: self.update_at,
        }
    }
}

impl Into<PersonWithPosts> for models::PersonWithPosts {
    fn into(self) -> PersonWithPosts {
        PersonWithPosts {
            id: self.id,
            name: self.name,
            posts: self.posts.into_iter().map(Into::into).collect(),
            create_at: self.create_at,
            update_at: self.update_at,
        }
    }
}

impl Into<Post> for models::Post {
    fn into(self) -> Post {
        Post {
            id: self.id,
            person_id: self.person_id,
            text: self.text,
            create_at: self.create_at,
            update_at: self.update_at,
        }
    }
}
