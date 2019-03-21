use super::schema::accounts;

#[derive(Queryable)]
pub struct Account {
  pub id: i64,
  pub guild: i64,
  pub role: i64,
  pub mute: bool
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount {
  pub id: i64,
  pub guild: i64,
  pub role: i64,
  pub mute: bool
}
