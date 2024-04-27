use std::fmt::Debug;

use async_lazy::Lazy;
use serde::{Deserialize, Serialize};

use crate::database::Database;
use crate::error::BoxResult;

mod database;
mod error;

static DATABASE: Lazy<Database> =
    Lazy::const_new(|| Box::pin(async { Database::open().await.unwrap() }));

macro_rules! db {
    () => {
        DATABASE.force().await
    };
}

#[tokio::main]
async fn main() -> BoxResult<()> {
    db!().insert("abc", &Test("gaa")).await?;
    let test: Test<String> = db!().get("abc").await?;
    println!("{test:?}");
    db!().insert("lol", &Test("gaa")).await?;
    db!().insert("abc", &Test("gsaa")).await?;
    let test: Test<String> = db!().get("abc").await?;
    println!("{test:?}");
    db!().insert("absc", &Test("gsaa")).await?;
    db!().insert("absfc", &Test("gsaa")).await?;
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct Test<T: ToString>(T);
