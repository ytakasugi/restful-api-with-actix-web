use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::{get, web, Responder, Result};

use crate::schema;
use crate::util::db;
use crate::model::{Task, Selectable};

#[get("/task/{task_id}")]
async fn get_task(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let task_id = path.into_inner();
    let task: Task = schema::tasks::table
        .select(Task::columns())
        .filter(schema::tasks::task_id.eq(task_id))
        .get_result(&conn)
        .expect("error");

    Ok(web::Json(task))
}