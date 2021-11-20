mod security;
mod todo_rest;

use crate::todo_rest::todo_filter;
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;

const WEB_FOLDER: &str = "web_folder/";

#[tokio::main]
async fn main() {
    let db_pool = Arc::new(DbPool {});

    // APIS
    let hi = warp::path("hi")
        .and(warp::get())
        .map(|| "Hello world from hi");
    let apis = hi.or(todo_filter(db_pool.clone()));

    // Static Content
    let content = warp::fs::dir(WEB_FOLDER);
    let root = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));
    let static_site = content.or(root);

    let routes = apis.or(static_site);

    println!("Start web-server!");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

pub struct DbPool {}

pub fn with_db_pool(
    db_pool: Arc<DbPool>,
) -> impl Filter<Extract = (Arc<DbPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
