use crate::db::schema::{person, person::dsl::*, post, post::dsl::*};
use chrono::NaiveDateTime;
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

#[derive(Debug, PartialEq, Identifiable, Queryable, Insertable)]
#[table_name = "person"]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

#[derive(Debug, PartialEq, Identifiable, Associations, Queryable, Insertable)]
#[belongs_to(Person, foreign_key = "person_id")]
#[table_name = "post"]
pub struct Post {
    pub id: Uuid,
    pub person_id: Uuid,
    pub text: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

pub struct PersonWithPosts {
    pub id: Uuid,
    pub name: String,
    pub posts: Vec<Post>,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
}

impl Person {
    pub fn find_all(conn: &diesel::PgConnection) -> Result<Vec<Person>, Error> {
        person.order(person::update_at.desc()).load::<Person>(conn)
    }

    pub fn find_by_id(conn: &diesel::PgConnection, pid: Uuid) -> Result<Person, Error> {
        person.find(pid).first(conn)
    }

    pub fn save(conn: &diesel::PgConnection, new_person: Person) -> Result<Person, Error> {
        diesel::insert_into(person)
            .values(&new_person)
            .get_result(conn)
    }

    pub fn delete(conn: &diesel::PgConnection, pid: Uuid) -> Result<Person, Error> {
        diesel::delete(person.filter(person::id.eq(pid))).get_result(conn)
    }
}

impl Post {
    pub fn find_all(conn: &diesel::PgConnection) -> Result<Vec<Post>, Error> {
        post.order(post::update_at.desc()).load::<Post>(conn)
    }

    pub fn find_by_id(conn: &diesel::PgConnection, pid: Uuid) -> Result<Post, Error> {
        post.find(pid).first(conn)
    }

    pub fn find_by_person_id(conn: &diesel::PgConnection, pid: Uuid) -> Result<Vec<Post>, Error> {
        post.filter(person_id.eq(pid))
            .order(post::update_at.desc())
            .get_results(conn)
    }

    pub fn save(conn: &diesel::PgConnection, new_post: Post) -> Result<Post, Error> {
        diesel::insert_into(post).values(&new_post).get_result(conn)
    }

    pub fn delete(conn: &diesel::PgConnection, pid: Uuid) -> Result<Post, Error> {
        diesel::delete(post.filter(post::id.eq(pid))).get_result(conn)
    }
}

impl PersonWithPosts {
    pub fn find_all(conn: &diesel::PgConnection) -> Result<Vec<PersonWithPosts>, Error> {
        let persons = Person::find_all(conn)?;
        let posts: Vec<Vec<Post>> = Post::belonging_to(&persons)
            .order(post::update_at.desc())
            .load::<Post>(conn)?
            .grouped_by(&persons);

        let results = persons.into_iter().zip(posts).map(Into::into).collect();

        Ok(results)
    }

    pub fn find_by_id(conn: &diesel::PgConnection, pid: Uuid) -> Result<PersonWithPosts, Error> {
        let psn: Person = person.find(pid).first(conn)?;
        let posts: Vec<Post> = Post::belonging_to(&psn)
            .order(post::update_at.desc())
            .load::<Post>(conn)?;

        Ok((psn, posts).into())
    }
}

impl Into<PersonWithPosts> for (Person, Vec<Post>) {
    fn into(self) -> PersonWithPosts {
        PersonWithPosts {
            id: self.0.id,
            name: self.0.name,
            posts: self.1,
            create_at: self.0.create_at,
            update_at: self.0.update_at,
        }
    }
}
