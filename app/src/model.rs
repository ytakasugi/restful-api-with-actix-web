use crate::schema::tasks;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

pub trait Selectable {
    type Columns;
    fn columns() -> Self::Columns;
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[table_name = "tasks"]
pub struct Task {
    pub content: String,
    pub created: NaiveDateTime,
    // データ投入直後は、Null(None)なのでOptionでラップする
    pub updated: Option<NaiveDateTime>,
    pub dead_line: NaiveDateTime,
    pub finished_flag: bool,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "tasks"]
pub struct NewTask {
    pub content: String,
    pub dead_line: NaiveDateTime,
}

impl Selectable for Task {
    type Columns = (tasks::content, tasks::created, tasks::updated, tasks::dead_line, tasks::finished_flag);

    fn columns() -> Self::Columns {
        (tasks::content, tasks::created, tasks::updated, tasks::dead_line, tasks::finished_flag)
    }
}