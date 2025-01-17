use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenvy::dotenv;
use models::{NewPost, Post};
use std::env;
//sqlite> .schema
// CREATE TABLE posts (
//     id SERIAL PRIMARY KEY,
//     title VARCHAR NOT NULL,
//     body TEXT NOT NULL,
//     published BOOLEAN NOT NULL DEFAULT FALSE
//   );

mod schema {
    use diesel::table;
    table! {
        posts {
            id -> Integer,
            title -> Text,
            body -> Text,
            published -> Bool,
        }
    }
}

mod models {
    use super::schema::posts;
    use diesel::prelude::*;

    #[derive(Queryable, Identifiable, Selectable)]
    #[diesel(table_name = posts)]
    pub struct Post {
        pub id: i32,
        pub title: String,
        pub body: String,
        pub published: bool,
    }

    #[derive(Insertable)]
    #[diesel(table_name = posts)]
    pub struct NewPost<'a> {
        pub title: &'a str,
        pub body: &'a str,
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("database").unwrap_or_else(|_| "test.db".into()); // 如果未设置环境变量，则使用 test.db

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
    use schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

fn query_post(mut connection: SqliteConnection) {
    use schema::posts;
    let results = posts::table
        .filter(posts::published.eq(true))
        .limit(5)
        .load::<Post>(&mut connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}

fn diesel_sample() {
    let mut connection = establish_connection();

    let post = create_post(&mut connection, "My first post", "Hello, world!");

    println!("Saved post with id: {}", post.id);

    query_post(connection);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diesel_sample() {
        diesel_sample();
    }
}
