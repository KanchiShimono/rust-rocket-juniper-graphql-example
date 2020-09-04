use crate::db::{
    repository::{PgPersonRepository, PgPersonWithPostsRepository, PgPostRepository},
    Db,
};
use rocket::request::{FromRequest, Outcome, Request};

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
