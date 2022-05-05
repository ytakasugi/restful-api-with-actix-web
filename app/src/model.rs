use crate::schema::{users, tasks};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

pub trait Selectable {
    type Columns;
    fn columns() -> Self::Columns;
}

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct UserTask {
    pub user_name: String,
    pub content: String,
    pub dead_line: NaiveDateTime,
    pub finished_flag: bool,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub user_name: String,
    pub e_mail: String,
}


#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub user_id: i32,
    pub content: String,
    pub dead_line: NaiveDateTime,
}

impl Selectable for UserTask {
    type Columns = (users::user_name, tasks::content, tasks::dead_line, tasks::finished_flag);

    fn columns() -> Self::Columns {
        (users::user_name, tasks::content, tasks::dead_line, tasks::finished_flag)
    }
}