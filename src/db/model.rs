use super::schema::{ accounts, todo, user_roles };

use uuid::Uuid;

#[derive(Queryable)]
pub struct Account {
  pub id: i64,
  pub guild: i64,
  pub mute: bool
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount {
  pub id: i64,
  pub guild: i64,
  pub mute: bool
}

#[derive(Queryable)]
pub struct UserRole {
  pub id: i64,
  pub role_id: i64,
}

#[derive(Insertable)]
#[table_name="user_roles"]
pub struct NewUserRole {
  pub id: i64,
  pub role_id: i64,
}

#[derive(Queryable)]
pub struct TODO {
  pub id: Uuid,
  pub user_id: i64,
  pub text: String
}

#[derive(Insertable)]
#[table_name="todo"]
pub struct NewTODO {
  pub user_id: i64,
  pub text: String
}
