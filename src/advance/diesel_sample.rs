/// This file contains the implementation of the `advance/diesel_sample.rs` file.
///
use diesel::{prelude::*, sqlite::SqliteConnection};
use dotenvy::dotenv;
use models::{NewPost, Post};
use std::env;

///
/// Schema definition for the `posts` table.
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
/// Models representing the data structure of the `posts` table in the database.
mod models {
    use super::schema::posts;
    use diesel::prelude::*;

    /// Represents a post in the `posts` table.
    #[derive(Queryable, Identifiable, Selectable)]
    #[diesel(table_name = posts)]
    pub struct Post {
        /// The unique identifier for the post.
        pub id: i32,
        /// The title of the post.
        pub title: String,
        /// The body of the post.
        pub body: String,
        /// Whether the post is published or not.
        pub published: bool,
    }

    /// Represents a new post to be inserted into the `posts` table.
    #[derive(Insertable)]
    #[diesel(table_name = posts)]
    pub struct NewPost<'a> {
        /// The title of the new post.
        pub title: &'a str,
        /// The body of the new post.
        pub body: &'a str,
    }
}

/// Establishes a connection to the SQLite database using environment variables.
///
/// If the `DATABASE_URL` environment variable is not set, it defaults to "test.db".
///
/// # Returns
///
/// A `SqliteConnection` object representing the established connection to the database.
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("database").unwrap_or_else(|_| "test.db".into()); // 如果未设置环境变量，则使用 test.db

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Sets up the database by creating the necessary tables.
fn setup_database(connection: &mut SqliteConnection) {
    // SQL query to create the 'posts' table if it doesn't already exist.
    let create_table_query = "
        CREATE TABLE IF NOT EXISTS posts  (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title VARCHAR NOT NULL,
        body TEXT ,
        published BOOLEAN  DEFAULT FALSE
        );
    ";

    // Execute the SQL query to create the 'posts' table.
    match diesel::sql_query(create_table_query).execute(connection) {
        Ok(_) => println!("Table created successfully."),
        Err(err) => println!("Error creating table: {:?}", err),
    }
}

/// Inserts a new post into the database.
fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
    use schema::posts;

    // create an new post
    let new_post = NewPost { title, body };

    // insert the new post into the database
    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

/// Query records from posts table
///
/// Retrieves up to 5 published posts from the database.
fn query_post(connection: &mut SqliteConnection) {
    use schema::posts;
    // query record from posts table
    let results = posts::table
        .filter(posts::published.eq(false))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts:", results.len());
    // loop through the results and print each post
    for post in results {
        println!("post.id: {}", post.id);
        println!("post.title: {}", post.title);
        println!("----------\n");
        println!("post.body: {}", post.body);
    }
}

/// Delete records from posts table
fn delete_post(connection: &mut SqliteConnection) {
    use schema::posts;

    let pattern = "%My first post1%";
    let num_deleted = diesel::delete(posts::table.filter(posts::title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}

/// This example demonstrates how to create a SQLite database connection, setup the database,
/// insert posts into the database, and query records from the `posts` table.
///
/// The code includes functions for establishing a connection, creating necessary tables,
/// inserting new posts, and querying published posts. Each function is designed to
/// perform specific tasks related to the database operations.
fn diesel_sample() {
    let mut connection = establish_connection();

    // setup the database
    setup_database(&mut connection);

    // insert new posts
    let post = create_post(&mut connection, "My first post", "Hello, world!");

    println!("Saved post with id: {}", post.id);

    // query published posts
    query_post(&mut connection);

    // delete the inserted post
    delete_post(&mut connection);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diesel_sample() {
        diesel_sample();
    }
}
