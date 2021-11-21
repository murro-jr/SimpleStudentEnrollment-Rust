use std::sync::Arc;

use crate::{
    dbpool::{save_db, with_db_pool, DbPool, Student},
    error,
    security::{do_auth, UserCtx},
};
use serde_json::{json, Value};
use warp::{reply::Json, Filter};

pub(crate) fn student_filter(
    db_pool: Arc<DbPool>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let root = warp::path("students");
    // LIST todos
    let list = root
        .and(warp::get())
        .and(warp::path::end())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and_then(find_all);

    let get = root
        .and(warp::get())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::path::param()) // e.g., /todos/123
        .and_then(get_student);

    let create = root
        .and(warp::post())
        .and(do_auth())
        .and(with_db_pool(db_pool.clone()))
        .and(warp::body::json())
        .and_then(create_student);

    list.or(get).or(create)
}

async fn find_all(_user_ctx: UserCtx, db_pool: Arc<DbPool>) -> Result<Json, warp::Rejection> {
    let todos = warp::reply::json(&db_pool.students);
    Ok(todos)
}

async fn get_student(
    _user_ctx: UserCtx,
    _db_pool: Arc<DbPool>,
    id: i64,
) -> Result<Json, warp::Rejection> {
    // TODO - get from DB
    let todo = json!(
        {"id": id, "user_id": _user_ctx.user_id, "title": format!("todo {}", id)}
    );

    // serde-json to warp-reply
    let todo = warp::reply::json(&todo);

    Ok(todo)
}

async fn create_student(
    _user_ctx: UserCtx,
    db_pool: Arc<DbPool>,
    data: Value,
) -> Result<Json, warp::Rejection> {
    let id = data["id"].to_string().replace("\"", "");
    let name = data["name"].to_string().replace("\"", "");
    let level = data["level"].to_string().replace("\"", "");

    println!("Creating student info: {}", id);

    match id.parse::<i64>() {
        Ok(id) => {
            let student = Student::new(id, name, level);

            let mut students = db_pool.students.clone();
            students.push(student.clone());
            if save_db(db_pool.path.clone(), students).is_ok() {
                let todo = warp::reply::json(&student);
                Ok(todo)
            } else {
                Err(warp::reject::custom(error::ServerFailure))
            }
        }
        Err(err) => {
            println!("{}", err);
            Err(warp::reject::custom(error::InvalidID))
        }
    }
}
