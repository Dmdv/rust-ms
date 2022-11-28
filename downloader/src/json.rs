//! This example illustrates the way to send and receive statically typed JSON.
//!
//! In contrast to the arbitrary JSON example, this brings up the full power of
//! Rust compile-time type system guaranties though it requires a little bit
//! more code.

// https://jsonplaceholder.typicode.com

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: Option<i32>,
    #[serde(rename = "userId")]
    user_id: i32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let new_post = Post {
        id: None,
        title: "Reqwest.rs".into(),
        body: "https://docs.rs/reqwest".into(),
        user_id: 1,
    };

    let new_post: Post = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/posts")
        .json(&new_post)
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_post);

    let todo: Todo = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos/1")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", todo);

    Ok(())
}