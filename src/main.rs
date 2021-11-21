mod dbpool;
mod error;
mod security;
mod student;

use crate::dbpool::DbPool;
use crate::student::student_filter;
use warp::Filter;

const WEB_FOLDER: &str = "web_folder/";
const JS_FOLDER: &str = "js/";
const CSS_FOLDER: &str = "css/";

#[tokio::main]
async fn main() {
    let db_pool = DbPool::new();

    // APIS
    let apis = student_filter(db_pool);

    // Static Content
    let web = warp::path::end()
        .and(warp::get())
        .and(warp::fs::dir(WEB_FOLDER));
    let css = warp::path("css")
        .and(warp::get())
        .and(warp::fs::dir(CSS_FOLDER));
    let js = warp::path("js")
        .and(warp::get())
        .and(warp::fs::dir(JS_FOLDER));
    let content = web.or(css).or(js);

    let routes = apis.or(content);

    println!("Start web-server!");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
