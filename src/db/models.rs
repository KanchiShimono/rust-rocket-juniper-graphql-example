use crate::db::schema::{person, person::dsl::*, post, post::dsl::*};
use crate::graphql::schema::{CreatePersonInput, CreatePostInput};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, PartialEq, Identifiable, Queryable, Insertable)]
#[table_name = "person"]
pub struct Person {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, PartialEq, Identifiable, Associations, Queryable, Insertable)]
#[belongs_to(Person, foreign_key = "person_id")]
#[table_name = "post"]
pub struct Post {
    pub id: Uuid,
    pub person_id: Uuid,
    pub text: String,
}

pub struct PersonWithPost {
    pub id: Uuid,
    pub name: String,
    pub posts: Vec<Post>,
}

impl Person {
    pub fn find_all(conn: &diesel::PgConnection) -> Result<Vec<Person>, diesel::result::Error> {
        person.order(person::name).load::<Person>(conn)
    }

    pub fn save(
        conn: &diesel::PgConnection,
        new_person: CreatePersonInput,
    ) -> Result<Person, diesel::result::Error> {
        let new_person = Person {
            id: Uuid::new_v4(),
            name: new_person.name,
        };

        diesel::insert_into(person)
            .values(&new_person)
            .get_result(conn)
    }
}

impl Post {
    pub fn find_all(conn: &diesel::PgConnection) -> Result<Vec<Post>, diesel::result::Error> {
        post.order(post::id).load::<Post>(conn)
    }

    pub fn save(
        conn: &diesel::PgConnection,
        new_post: CreatePostInput,
    ) -> Result<Post, diesel::result::Error> {
        let new_post = Post {
            id: Uuid::new_v4(),
            person_id: new_post.person_id,
            text: new_post.text,
        };

        diesel::insert_into(post).values(&new_post).get_result(conn)
    }
}

impl PersonWithPost {
    pub fn find_all(
        conn: &diesel::PgConnection,
    ) -> Result<Vec<PersonWithPost>, diesel::result::Error> {
        let persons = Person::find_all(conn)?;
        let posts: Vec<Vec<Post>> = Post::belonging_to(&persons)
            .order(post::id)
            .load::<Post>(conn)?
            .grouped_by(&persons);

        let results = persons.into_iter().zip(posts).map(Into::into).collect();

        Ok(results)
    }
}

impl Into<PersonWithPost> for (Person, Vec<Post>) {
    fn into(self) -> PersonWithPost {
        PersonWithPost {
            id: self.0.id,
            name: self.0.name,
            posts: self.1,
        }
    }
}
