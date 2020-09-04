use crate::db::{
    models::{Person, PersonWithPosts, Post},
    Db,
};
use crate::graphql::schema::{CreatePersonInput, CreatePostInput};
use chrono::Utc;
use diesel::result::Error;
use uuid::Uuid;

pub trait PersonRepository {
    fn find_all(&self) -> Result<Vec<Person>, Error>;
    fn find_by_id(&self, id: Uuid) -> Result<Person, Error>;
    fn save(&self, input: CreatePersonInput) -> Result<Person, Error>;
    fn delete(&self, id: Uuid) -> Result<Person, Error>;
}

pub trait PostRepository {
    fn find_all(&self) -> Result<Vec<Post>, Error>;
    fn find_by_id(&self, id: Uuid) -> Result<Post, Error>;
    fn find_by_person_id(&self, person_id: Uuid) -> Result<Vec<Post>, Error>;
    fn save(&self, input: CreatePostInput) -> Result<Post, Error>;
    fn delete(&self, id: Uuid) -> Result<Post, Error>;
}

pub trait PersonWithPostsRepository {
    fn find_all(&self) -> Result<Vec<PersonWithPosts>, Error>;
    fn find_by_id(&self, id: Uuid) -> Result<PersonWithPosts, Error>;
}

pub struct PgPersonRepository {
    pub conn: Db,
}

impl PersonRepository for PgPersonRepository {
    fn find_all(&self) -> Result<Vec<Person>, Error> {
        Person::find_all(&self.conn)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Person, Error> {
        Person::find_by_id(&self.conn, id)
    }

    fn save(&self, input: CreatePersonInput) -> Result<Person, Error> {
        let now = Utc::now().naive_utc();

        let new_person = Person {
            id: Uuid::new_v4(),
            name: input.name,
            create_at: now,
            update_at: now,
        };

        Person::save(&self.conn, new_person)
    }

    fn delete(&self, id: Uuid) -> Result<Person, Error> {
        Person::delete(&self.conn, id)
    }
}

pub struct PgPostRepository {
    pub conn: Db,
}

impl PostRepository for PgPostRepository {
    fn find_all(&self) -> Result<Vec<Post>, Error> {
        Post::find_all(&self.conn)
    }

    fn find_by_id(&self, id: Uuid) -> Result<Post, Error> {
        Post::find_by_id(&self.conn, id)
    }

    fn find_by_person_id(&self, person_id: Uuid) -> Result<Vec<Post>, Error> {
        Post::find_by_person_id(&self.conn, person_id)
    }

    fn save(&self, input: CreatePostInput) -> Result<Post, Error> {
        let now = Utc::now().naive_utc();

        let new_post = Post {
            id: Uuid::new_v4(),
            person_id: input.person_id,
            text: input.text,
            create_at: now,
            update_at: now,
        };

        Post::save(&self.conn, new_post)
    }

    fn delete(&self, id: Uuid) -> Result<Post, Error> {
        Post::delete(&self.conn, id)
    }
}

pub struct PgPersonWithPostsRepository {
    pub conn: Db,
}

impl PersonWithPostsRepository for PgPersonWithPostsRepository {
    fn find_all(&self) -> Result<Vec<PersonWithPosts>, Error> {
        PersonWithPosts::find_all(&self.conn)
    }

    fn find_by_id(&self, id: Uuid) -> Result<PersonWithPosts, Error> {
        PersonWithPosts::find_by_id(&self.conn, id)
    }
}
