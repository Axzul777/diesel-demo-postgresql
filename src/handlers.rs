use crate::models::{NewUser, User};
use crate::schema;
use crate::schema::users::dsl::users;
use crate::schema::users::dsl::{self, *};
use diesel::dsl::exists;
use diesel::{prelude::*, select};
use diesel::{Connection, PgConnection};
use dotenv::dotenv;
use std::env;
use std::io;

pub fn establish_conection() -> Result<PgConnection, io::Error> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("VarError");

    let connection =
        PgConnection::establish(&db_url).expect(&format!("ERROR TO CONECTO {}", db_url));

    Ok(connection)
}

pub fn create_new_user<'a>(cnn: &mut PgConnection, u_name: &'a str) /*-> User*/
{
    let new_user = NewUser { name: u_name };

    //if u_name == users.filter(name.like(u_name)) {
    //    println!("Print User exisit");
    //    panic!("YOU ARE TRYINING MAKE AN EXIST USER");
    //}

    match select(exists(users.filter(name.eq(u_name)))).get_result(cnn) {
        Ok(true) => {
            println!("Exist {}", u_name)
        }
        Ok(false) => {
            diesel::insert_into(schema::users::table)
                .values(&new_user)
                .execute(cnn)
                .expect("ERROR SAVING USER");
        }
        Err(_) => todo!(),
    }
}

pub fn get_users(cnn: &mut PgConnection) {
    // LOAD USERS

    let results = dsl::users
        .load::<User>(cnn)
        .expect("DON'T WORRIES MR. WE DON'T DIE TODAY");

    dbg!(results);
}

pub fn delete_user(cnn_set: &mut PgConnection, user_n: &str) {
    let num_deleted = diesel::delete(users.filter(name.like(user_n)))
        .execute(cnn_set)
        .expect("ERROR DELETING USER");

    println!("SUCESS");
    println!("Deleted {} user", num_deleted);
}

pub fn update_username(cnn_set: &mut PgConnection, user_remame: &str) {
    todo!();
}
