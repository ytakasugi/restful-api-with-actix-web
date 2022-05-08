use diesel::RunQueryDsl;
use actix_web::{post, web, Responder, HttpResponse, Result};

use crate::schema;
use crate::util::db;
use crate::model::{NewUser, NewTask};

// ユーザを新規登録
#[post("/todo/users")]
async fn new_user(db: web::Data<db::Pool>, item: web::Json<NewUser>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let new_user = NewUser {
        user_name: item.user_name.to_string(),
        e_mail: item.e_mail.to_string(),
    };
    diesel::insert_into(schema::users::dsl::users)
        .values(&new_user)
        .execute(&conn)
        .expect("Error saving new post");

    Ok(HttpResponse::Created().body("create new user!"))
}

// タスクを新規登録
#[post("/todo/tasks")]
async fn new_task(db: web::Data<db::Pool>, item: web::Json<NewTask>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let new_task = NewTask {
        user_id: item.user_id,
        content: item.content.to_string(),
        dead_line: item.dead_line,
    };
    diesel::insert_into(schema::tasks::dsl::tasks)
        .values(&new_task)
        .execute(&conn)
        .expect("Error saving new post");

    Ok(HttpResponse::Created().body("create new task!"))
}
