use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fs::{remove_file, File, OpenOptions};
use warp::Filter;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct Student {
    pub(crate) id: i64,
    name: String,
    level: String,
}

impl Student {
    pub(crate) fn new(id: i64, name: String, level: String) -> Self {
        Self { id, name, level }
    }
}

const JSON_FILE: &str = "data/students.json";

#[derive(Copy, Clone)]
pub(crate) struct DbPool;

impl DbPool {
    pub(crate) fn new() -> Self {
        Self
    }

    pub(crate) fn get_db_path(self) -> String {
        std::env::var("DATABASE_PATH").unwrap_or(JSON_FILE.to_string())
    }

    pub(crate) fn load(self) -> Vec<Student> {
        let file = File::open(self.get_db_path()).map_err(|err| println!("{}", err));
        let mut students = Vec::<Student>::new();

        if let Ok(file) = file {
            students = serde_json::from_reader(file).unwrap_or(students);
        }

        students
    }
}

pub(crate) fn with_db_pool(
    db_pool: DbPool,
) -> impl Filter<Extract = (DbPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool)
}

pub(crate) fn save_db(path: String, students: Vec<Student>) -> Result<(), String> {
    let _result = remove_file(path.clone()).map_err(|err| println!("{}", err));

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .map_err(|err| format!("{}", err));

    match file {
        Ok(file) => {
            serde_json::ser::to_writer_pretty(file, &students).map_err(|err| format!("{}", err))
        }
        Err(err) => Err(err),
    }
}
