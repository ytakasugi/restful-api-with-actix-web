```rust
    let parameter: (Option<i32>, &Option<String>, &Option<String>) = (param.user_id, &param.user_name, &param.e_mail);
    
    match &parameter {
        (Some(user_id), None, None) => {
            let user: Vec<User> = schema::users::table
                .select(User::columns())
                .filter(schema::users::user_id.eq(user_id))
                .get_results(&conn)
                .expect("Error.");
            Ok(web::Json(user))
        },
        (None, Some(user_name), None) => {
            let user: Vec<User> = schema::users::table
                .select(User::columns())
                .filter(schema::users::user_name.eq(user_name))
                .get_results(&conn)
                .expect("Error.");
            Ok(web::Json(user))
        },
        (None, None, Some(e_mail)) => {
            let user: Vec<User> = schema::users::table
                .select(User::columns())
                .filter(schema::users::e_mail.eq(e_mail))
                .get_results(&conn)
                .expect("Error.");
            Ok(web::Json(user))
        },
        (Some(user_id), Some(user_name), None) => {
            let user: Vec<User> = schema::users::table
                .select(User::columns())
                .filter(schema::users::user_id.eq(user_id))
                .filter(schema::users::user_name.eq(user_name))
                .get_results(&conn)
                .expect("Error.");
            Ok(web::Json(user))
        },
        (Some(user_id), None, Some(e_mail)) => {
            let user: Vec<User> = schema::users::table
                .select(User::columns())
                .filter(schema::users::user_id.eq(user_id))
                .filter(schema::users::e_mail.eq(e_mail))
                .get_results(&conn)
                .expect("Error.");
            Ok(web::Json(user))
        },
        (None, Some(user_name), Some(e_mail)) => {
            let user: Vec<User> = schema::users::table
                .select(User::columns())
                .filter(schema::users::user_name.eq(user_name))
                .filter(schema::users::e_mail.eq(e_mail))
                .get_results(&conn)
                .expect("Error.");
            Ok(web::Json(user))
        },
        (Some(user_id), Some(user_name), Some(e_mail)) => {
            let user: Vec<User> = schema::users::table
                .select(User::columns())
                .filter(schema::users::user_id.eq(user_id))
                .filter(schema::users::user_name.eq(user_name))
                .filter(schema::users::e_mail.eq(e_mail))
                .get_results(&conn)
                .expect("Error.");
            Ok(web::Json(user))
        },
        (None, None, None) => {
            let user: Vec<User> = schema::users::table
                .select(User::columns())
                .get_results(&conn)
                .expect("Error.");
            Ok(web::Json(user))
        }
    }
```