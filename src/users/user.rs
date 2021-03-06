use diesel;
use diesel::prelude::*;
use crate::db::schema::users;

#[table_name = "users"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: Option<i32>,
    pub firstname: String,
    pub lastname: String,
    pub email: String
}

impl User {
  pub fn create(user: User, connection: &SqliteConnection) -> User {
      diesel::insert_into(users::table)
          .values(&user)
          .execute(connection)
          .expect("Error creating new user");

      users::table.order(users::id.desc()).first(connection).unwrap()
  }

  pub fn read(connection: &SqliteConnection) -> Vec<User> {
      users::table.order(users::id.asc()).load::<User>(connection).unwrap()
  }

  pub fn update(id: i32, user: User, connection: &SqliteConnection) -> bool {
      diesel::update(users::table.find(id)).set(&user).execute(connection).is_ok()
  }

  pub fn delete(id: i32, connection: &SqliteConnection) -> bool {
      diesel::delete(users::table.find(id)).execute(connection).is_ok()
  }
}