use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::{put, web, Responder, HttpResponse, Result};

use chrono::Utc;

use crate::schema;
use crate::util::db;
use crate::model::{UpdateUser, UpdateTask};

// ユーザー情報を更新するAPI
#[put("/todo/users/{user_id}")]
async fn update_user_info(db: web::Data<db::Pool>, path: web::Path<i32>, item: web::Json<UpdateUser>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let user_id = path.into_inner();
    // 現在時刻を取得
    let now = Utc::now().naive_utc();
    // 更新対象を選択する
    let target = schema::users::table
        .filter(schema::users::dsl::user_id.eq(user_id));

    // e-mailに変更があった場合
    if let Some(x) = &item.e_mail {
        diesel::update(target)
            .set((
                schema::users::dsl::e_mail.eq(x),
                schema::users::dsl::updated.eq(now),
            ))
            .execute(&conn)
            .expect("User info Update Error.");
    }

    // ユーザーを論理削除(後続の削除用APIで物理削除する)
    if let Some(x) = item.delete_flag {
        diesel::update(target)
            .set((
                schema::users::dsl::delete_flag.eq(x),
                schema::users::dsl::updated.eq(now),
            ))
            .execute(&conn)
            .expect("User info Update Error.");
    }
    
    Ok(HttpResponse::Created().body("update user!"))
}

// タスクの内容を変更するAPI
#[put("/todo/users/{user_id}/tasks/{task_id}")]
async fn update_task_info(db: web::Data<db::Pool>, path: web::Path<(i32, i32)>, item: web::Json<UpdateTask>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let (user_id, task_id) = path.into_inner();
    // 現在時刻を取得
    let now = Utc::now().naive_utc();
    // 更新対象を選択する
    let target = schema::tasks::table
        .filter(schema::tasks::dsl::user_id.eq(user_id))
        .filter(schema::tasks::dsl::task_id.eq(task_id));

    // タスクの内容に変更があった場合
    if let Some(x) = &item.content {
        diesel::update(target)
            .set((
                schema::tasks::dsl::content.eq(x),
                schema::tasks::dsl::updated.eq(now)
            ))
            .execute(&conn)
            .expect("Update Error.");
    }
    
    // 期限に変更があった場合
    if let Some(x) = item.dead_line {
        diesel::update(target)
            .set((
                schema::tasks::dsl::dead_line.eq(x),
                schema::tasks::dsl::updated.eq(now)
            ))
            .execute(&conn)
            .expect("Update Error.");
    }

    // 完了フラグが設定されていた場合
    if let Some(x) = item.finished_flag {
        diesel::update(target)
            .set((
                schema::tasks::dsl::finished_flag.eq(x),
                schema::tasks::dsl::updated.eq(now)
            ))
            .execute(&conn)
            .expect("Update Error.");
    }

    // タスク内容と期限に変更があった場合
    if let (Some(x), Some(y)) = (&item.content, item.dead_line) {
        diesel::update(target)
            .set((
                schema::tasks::dsl::content.eq(x),
                schema::tasks::dsl::dead_line.eq(y),
                schema::tasks::dsl::updated.eq(now)
            ))
            .execute(&conn)
            .expect("Update Error.");
    }
 
    Ok(HttpResponse::Created().body("update task!"))
}
