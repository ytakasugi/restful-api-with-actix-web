use diesel::RunQueryDsl;
use actix_web::{post, web, Responder, HttpResponse, Result};

use crate::schema;
use crate::util::db;
use crate::model::NewTask;

#[post("/task")]
async fn new_task(db: web::Data<db::Pool>, item: web::Json<NewTask>) -> Result<impl Responder> {
    let conn = db.get().unwrap();
    let new_task = NewTask {
        content: item.content.to_string(),
        dead_line: item.dead_line,
    };
    diesel::insert_into(schema::tasks::dsl::tasks)
        .values(&new_task)
        .execute(&conn)
        .expect("Error saving new post");

    Ok(HttpResponse::Created().body("create new task!"))
}