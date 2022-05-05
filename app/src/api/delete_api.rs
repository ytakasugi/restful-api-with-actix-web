use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

use actix_web::{delete, web, Responder, HttpResponse, Result};

use crate::schema;
use crate::util::db;

// ユーザーを物理削除するAPI
#[delete("/todo/delete/user/{user_id}")]
async fn delete_user(db: web::Data<db::Pool>, path: web::Path<i32>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let user_id = path.into_inner();
    // 更新対象を選択する
    let target = schema::users::table
        .filter(schema::users::dsl::user_id.eq(user_id))
        .filter(schema::users::dsl::delete_flag.eq(true));

    diesel::delete(target)
        .execute(&conn)
        .expect("No delete target.");

    Ok(HttpResponse::Created().body("delete user!"))
}