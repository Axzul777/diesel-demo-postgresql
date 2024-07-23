use super::schema::users;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
}
