use crate::{
    dbpool::{save_db, with_db_pool, DbPool, Student},
    error,
    security::{do_auth, UserCtx},
};
use serde_json::{json, Value};
use warp::{reply::Json, Filter};

pub(crate) fn student_filter<'a>(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let root = warp::path("students");

    // @desc    Lists all registered student info
    // @route   GET /students
    let list = root
        .and(warp::get())
        .and(warp::path::end())
        .and(do_auth())
        .and(with_db_pool(db_pool))
        .and_then(find_all);

    // @desc    Gets specific student info
    // @route   GET /students/:index
    let get = root
        .and(warp::get())
        .and(do_auth())
        .and(with_db_pool(db_pool))
        .and(warp::path::param()) // e.g., /todos/123
        .and_then(get_student);

    // @desc    Registers new student info
    // @route   POST /students
    let create = root
        .and(warp::post())
        .and(do_auth())
        .and(with_db_pool(db_pool))
        .and(warp::body::json())
        .and_then(create_student);

    // @desc    Updates an existing student info
    // @route   PUT /students
    let update = root
        .and(warp::put())
        .and(do_auth())
        .and(with_db_pool(db_pool))
        .and(warp::body::json())
        .and_then(update_student);

    // @desc    Deletes a specific student info
    // @route   DELETE /students
    let delete = root
        .and(warp::delete())
        .and(do_auth())
        .and(with_db_pool(db_pool))
        .and(warp::path::param())
        .and_then(delete_student);

    list.or(get).or(create).or(update).or(delete)
}

// @desc    Method to retrieve all info from database
// @params
//      #TODO UserCtx -> User authorization to access database
//      db_pool: DbPool -> Database instance
async fn find_all(_user_ctx: UserCtx, db_pool: DbPool) -> Result<Json, warp::Rejection> {
    let students = db_pool.load();
    let todos = warp::reply::json(&students);
    Ok(todos)
}

// @desc    Method to get a specific student info from database
// @params
//      #TODO UserCtx -> User authorization to access database
//      db_pool: DbPool -> Database instance
//      id: i64 -> Student ID
async fn get_student(
    _user_ctx: UserCtx,
    db_pool: DbPool,
    id: i64,
) -> Result<Json, warp::Rejection> {
    let students = db_pool.load();
    let student = students.into_iter().find(|s| s.id == id);

    // serde-json to warp-reply
    let todo = warp::reply::json(&student);
    Ok(todo)
}

// @desc    Method to create a specific student info into the database
// @params
//      #TODO UserCtx -> User authorization to access database
//      db_pool: DbPool -> Database instance
//      data: Value -> JSON value received from request
async fn create_student(
    _user_ctx: UserCtx,
    db_pool: DbPool,
    data: Value,
) -> Result<Json, warp::Rejection> {
    let id = data["id"].to_string().replace("\"", "");
    let name = data["name"].to_string().replace("\"", "");
    let level = data["level"].to_string().replace("\"", "");

    println!("Creating student info: {:?}", data);

    match id.parse::<i64>() {
        Ok(id) => {
            let student = Student::new(id, name, level);

            let mut students = db_pool.load();
            students.push(student.clone());
            if save_db(db_pool.get_db_path(), students).is_ok() {
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

// @desc    Method to create a specific student info into the database
// @params
//      #TODO UserCtx -> User authorization to access database
//      db_pool: DbPool -> Database instance
//      data: Value -> JSON value received from request
async fn update_student(
    _user_ctx: UserCtx,
    db_pool: DbPool,
    data: Value,
) -> Result<Json, warp::Rejection> {
    let id = data["id"].to_string().replace("\"", "");
    let name = data["name"].to_string().replace("\"", "");
    let level = data["level"].to_string().replace("\"", "");

    println!("Updating student info: {:?}", data);
    match id.parse::<i64>() {
        Ok(id) => {
            let student = Student::new(id, name, level);

            let mut students = db_pool.load();
            let index = students.clone().into_iter().position(|s| s.id == id);

            if let Some(index) = index {
                students[index] = student.clone();

                println!("Updated list: {:?}", students);

                if save_db(db_pool.get_db_path(), students.clone()).is_ok() {
                    let todo = warp::reply::json(&student);
                    Ok(todo)
                } else {
                    Err(warp::reject::custom(error::ServerFailure))
                }
            } else {
                Err(warp::reject::custom(error::NotFound))
            }
        }
        Err(err) => {
            println!("{}", err);
            Err(warp::reject::custom(error::InvalidID))
        }
    }
}

// @desc    Method to create a specific student info into the database
// @params
//      #TODO UserCtx -> User authorization to access database
//      db_pool: DbPool -> Database instance
//      id: i64 -> Student ID
async fn delete_student(
    _user_ctx: UserCtx,
    db_pool: DbPool,
    id: i64,
) -> Result<Json, warp::Rejection> {
    let mut students = db_pool.load();
    let index = students.clone().into_iter().position(|s| s.id == id);

    if let Some(index) = index {
        students.remove(index);

        if save_db(db_pool.get_db_path(), students.clone()).is_ok() {
            let todo = warp::reply::json(&json!({ "message": "Deleted student info" }));
            Ok(todo)
        } else {
            Err(warp::reject::custom(error::ServerFailure))
        }
    } else {
        Err(warp::reject::custom(error::NotFound))
    }
}
