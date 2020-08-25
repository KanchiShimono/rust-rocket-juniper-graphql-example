use crate::db::schema::{person, person::dsl::*, post, post::dsl::*};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, PartialEq, Identifiable, Queryable)]
#[table_name = "person"]
pub struct Person {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Identifiable, Associations, Queryable, PartialEq)]
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
}

impl Post {
    pub fn find_all(conn: &diesel::PgConnection) -> Result<Vec<Post>, diesel::result::Error> {
        post.order(post::id).load::<Post>(conn)
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
