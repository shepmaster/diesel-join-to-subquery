#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod models {
    use super::schema::{users, posts, votes, total_votes};

    type Id = i32;

    #[derive(Debug, Identifiable, Queryable)]
    pub struct User {
        pub id: Id,
        pub name: String,
    }

    #[derive(Debug, Insertable)]
    #[table_name = "users"]
    pub struct NewUser<'a> {
        pub name: &'a str,
    }

    #[derive(Debug, Queryable)]
    pub struct Post {
        pub id: Id,
        pub name: String,
    }

    #[derive(Debug, Insertable)]
    #[table_name = "posts"]
    pub struct NewPost<'a> {
        pub name: &'a str,
    }

    #[derive(Debug, Queryable, Insertable)]
    #[table_name = "votes"]
    pub struct Vote {
        pub user_id: Id,
        pub post_id: Id,
        pub count: i32,
    }

    #[derive(Debug, Identifiable, Queryable, Associations)]
    #[primary_key(user_id)]
    #[belongs_to(User)]
    pub struct TotalVote {
        pub user_id: Id,
        pub total: i32,
    }
}

mod schema {
    infer_schema!("dotenv:DATABASE_URL");

    // This corresponds to a view; it doesn't seem like those are
    // inferred at the moment.
    table! {
        total_votes (user_id) {
            user_id -> Integer,
            total -> Integer,
        }
    }
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|e| panic!("Error connecting to {}: {}", database_url, e))
}

fn main() {
    use models::*;
    use schema::{users, posts, votes, total_votes};

    let connection = establish_connection();
    let conn = &connection;

    // This is all just setup so we have some users, some posts, and
    // some votes on those posts.

    let user = NewUser { name: "user 1" };

    diesel::insert(&user)
        .into(users::table)
        .execute(conn)
        .expect("Error saving new user");

    let user: User = users::table.first(conn).expect("Couldn't find user");

    for i in 0..5 {
        let name = format!("post {}", i);
        let post = NewPost { name: &name };

        diesel::insert(&post)
            .into(posts::table)
            .execute(conn)
            .expect("Error saving new post");

        let post: Post = posts::table
            .order(posts::id.desc())
            .first(conn).expect("Couldn't find post");

        let vote = Vote {
            user_id: user.id,
            post_id: post.id,
            count: 100,
        };

        diesel::insert(&vote)
            .into(votes::table)
            .execute(conn)
            .expect("Error saving new vote");
    }

    // The query

    let a = users::table
        .inner_join(total_votes::table)
        .select((users::id, users::name, total_votes::total))
        .load::<(i32, String, i32)>(conn);

    println!("{:?}", a);
}
