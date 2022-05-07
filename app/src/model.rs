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

#[derive(Queryable, Deserialize, Serialize)]
pub struct User {
    pub user_id: i32,
    pub user_name: String,
    pub e_mail: String,
}

#[derive(Queryable, Deserialize, Serialize)]
pub struct Task {
    pub task_id: i32,
    pub user_id: i32,
    pub content: String,
    pub dead_line: NaiveDateTime,
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

#[derive(Deserialize, Serialize)]
pub struct  UpdateUser {
    pub e_mail: Option<String>,
    pub finished_flag: Option<bool>
}

#[derive(Deserialize, Serialize)]
pub struct  UpdateTask {
    pub content: Option<String>,
    pub dead_line: Option<NaiveDateTime>,
    pub finished_flag: Option<bool>
}


impl Selectable for UserTask {
    type Columns = (users::user_name, tasks::content, tasks::dead_line, tasks::finished_flag);

    fn columns() -> Self::Columns {
        (users::user_name, tasks::content, tasks::dead_line, tasks::finished_flag)
    }
}

impl Selectable for User {
    type Columns = (users::user_id, users::user_name, users::e_mail);

    fn columns() -> Self::Columns {
        (users::user_id, users::user_name, users::e_mail)
    }
}

impl Selectable for Task {
    type Columns = (tasks::task_id, tasks::user_id, tasks::content, tasks::dead_line);

    fn columns() -> Self::Columns {
        (tasks::task_id, tasks::user_id, tasks::content, tasks::dead_line)
    }
}