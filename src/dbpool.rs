use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fs::{File, OpenOptions};
use std::sync::Arc;
use warp::Filter;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Student {
    id: i64,
    name: String,
    level: String,
}

impl Student {
    pub(crate) fn new(id: i64, name: String, level: String) -> Self {
        Self { id, name, level }
    }
}

pub(crate) struct DbPool {
    pub(crate) students: Vec<Student>,
    pub(crate) path: String,
}

impl DbPool {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).map_err(|err| println!("{}", err));
        let mut students = Vec::<Student>::new();

        if let Ok(file) = file {
            students = serde_json::from_reader(file).unwrap_or(students);
        }

        Self {
            students,
            path: path.to_string(),
        }
    }
}

pub(crate) fn with_db_pool(
    db_pool: Arc<DbPool>,
) -> impl Filter<Extract = (Arc<DbPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub(crate) fn save_db(path: String, students: Vec<Student>) -> Result<(), serde_json::Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .map_err(|err| println!("{}", err));
    serde_json::ser::to_writer_pretty(file.unwrap(), &students)
}
