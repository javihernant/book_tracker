use diesel::prelude::*;
use crate::schema::users;

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    mail: String,
    api_token: Option<String>
}

#[derive(serde::Deserialize, Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    username: String,
    password: String,
    mail: String,
}

impl NewUser {
    pub fn new(username:String, password:String, mail: String) -> Self {
        NewUser {
            username,
            password,
            mail
        }
    }
    pub fn password(&self) -> &str {
        self.password.as_ref()
    }
    pub fn username(&self) -> &str {
        self.username.as_ref()
    }
    pub fn mail(&self) -> &str {
        self.mail.as_ref()
    }
}