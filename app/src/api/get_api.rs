use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::{get, web, Responder, Result};

use crate::schema;
use crate::util::db;
use crate::model::{UserTask, Selectable};

// 特定のユーザーのタスクを取得
#[get("/todo/search/user/{user_id}/{task_id}")]
async fn get_task(db: web::Data<db::Pool>, path: web::Path<(i32, i32)>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let (user_id, task_id) = path.into_inner();
    
    let user_task: Vec<UserTask> = schema::users::table
        .filter(schema::users::user_id.eq(user_id))
        .inner_join(schema::tasks::table)
        .filter(schema::tasks::task_id.eq(task_id))
        .select(UserTask::columns())
        .get_results(&conn)
        .expect("Connection Error.");

    Ok(web::Json(user_task))
}