use crate::db::models;
use uuid::Uuid;

#[derive(GraphQLObject)]
pub struct Person {
    id: Uuid,
    name: String,
    posts: Vec<Post>,
}

#[derive(GraphQLObject)]
pub struct Post {
    id: Uuid,
    person_id: Uuid,
    text: String,
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
            posts: vec![],
        }
    }
}

impl Into<Person> for models::PersonWithPost {
    fn into(self) -> Person {
        Person {
            id: self.id,
            name: self.name,
            posts: self.posts.into_iter().map(Into::into).collect(),
        }
    }
}

impl Into<Post> for models::Post {
    fn into(self) -> Post {
        Post {
            id: self.id,
            person_id: self.person_id,
            text: self.text,
        }
    }
}
