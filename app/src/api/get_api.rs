use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::{get, web, Responder, Result};

use crate::schema;
use crate::util::db;
use crate::model::{UserTask, Selectable};


// 特定のユーザーの特定のタスクを取得
#[get("/todo/search/user/{user_id}/task/{task_id}")]
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

// 特定のユーザーの全タスクを取得するAPI
#[get("/todo/search/user/{user_id}/task/all")]
async fn get_user_all_task(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let user_id = path.into_inner();

    let user_all_task: Vec<UserTask> = schema::users::table
        .inner_join(schema::tasks::table)
        .select(UserTask::columns())
        .filter(schema::users::user_id.eq(user_id))
        .get_results(&conn)
        .expect("Error.");

        Ok(web::Json(user_all_task))
}

// 全てのユーザーを参照するAPI
#[get("/todo/search/user/all")]
async fn get_all_user(db: web::Data<db::Pool>) -> Result<impl Responder> {
    let conn = db.get().unwrap();

    let all_user = schema::users::table
        .select((schema::users::user_id, schema::users::user_name, schema::users::e_mail))
        .load::<(i32, String, String)>(&conn)
        .expect("Error.");

        Ok(web::Json(all_user))
}

// 全てのタスクを参照するAPI
#[get("/todo/search/task/all")]
async fn get_all_task(db: web::Data<db::Pool>) -> Result<impl Responder> {
    let conn = db.get().unwrap();

    let all_task = schema::tasks::table
        .select((schema::tasks::task_id, schema::tasks::user_id, schema::tasks::content))
        .load::<(i32, i32, String)>(&conn)
        .expect("Error.");

        Ok(web::Json(all_task))
}
