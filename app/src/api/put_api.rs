use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::{put, web, Responder, HttpResponse, Result};

use chrono::Utc;

use crate::schema;
use crate::util::db;
use crate::model::{UpdateUser, UpdateTask};

// ユーザー情報を更新するAPI
#[put("/todo/update/user/{user_id}")]
async fn update_user_info(db: web::Data<db::Pool>, path: web::Path<i32>, item: web::Json<UpdateUser>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let user_id = path.into_inner();
    // 現在時刻を取得
    let now = Utc::now().naive_utc();
    // 更新対象を選択する
    let target = schema::users::table
        .filter(schema::users::dsl::user_id.eq(user_id));

    diesel::update(target)
        .set((
            schema::users::dsl::e_mail.eq(item.e_mail.to_string()),
            schema::users::dsl::updated.eq(now),
        ))
        .execute(&conn)
        .expect("User info Update Error.");
    
    Ok(HttpResponse::Created().body("update user!"))
}

// ユーザーを論理削除するAPI(後続の削除用APIで物理削除する)
#[put("/todo/update/logicaldelete/user/{user_id}")]
async fn logical_delete(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let user_id = path.into_inner();
    // 現在時刻を取得
    let now = Utc::now().naive_utc();
    // 更新対象を選択する
    let target = schema::users::table
        .filter(schema::users::dsl::user_id.eq(user_id));

        diesel::update(target)
        .set((
            schema::users::dsl::delete_flag.eq(true),
            schema::users::dsl::updated.eq(now),
        ))
        .execute(&conn)
        .expect("User info Update Error.");


    Ok(HttpResponse::Created().body("update user!"))
}

// タスクの期限を変更するAPI
#[put("/todo/update/user/{user_id}/task/{task_id}")]
async fn update_dead_line(db: web::Data<db::Pool>, path: web::Path<(i32, i32)>, item: web::Json<UpdateTask>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let (user_id, task_id) = path.into_inner();
    // 現在時刻を取得
    let now = Utc::now().naive_utc();
    // 更新対象を選択する
    let target = schema::tasks::table
        .filter(schema::tasks::dsl::user_id.eq(user_id))
        .filter(schema::tasks::dsl::task_id.eq(task_id));

    diesel::update(target)
        .set((
            schema::tasks::dsl::dead_line.eq(item.dead_line),
            schema::tasks::dsl::updated.eq(now)
        ))
        .execute(&conn)
        .expect("Update Error.");
    
    Ok(HttpResponse::Created().body("update task!"))
}

// 特定のユーザーのタスクを完了させるAPI
#[put("/todo/update/complete/user/{user_id}/task/{task_id}")]
async fn task_completed(db: web::Data<db::Pool>, path: web::Path<(i32, i32)>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let (user_id, task_id) = path.into_inner();
    // 現在時刻を取得
    let now = Utc::now().naive_utc();
    // 更新対象を選択する
    let target = schema::tasks::table
        .filter(schema::tasks::dsl::user_id.eq(user_id))
        .filter(schema::tasks::dsl::task_id.eq(task_id));

    diesel::update(target)
        .set((
                schema::tasks::dsl::finished_flag.eq(true),
                schema::tasks::dsl::updated.eq(now)
        ))
        .execute(&conn)
        .expect("Update Error.");

    Ok(HttpResponse::Created().body("completed task!"))
}