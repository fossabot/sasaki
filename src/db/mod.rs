pub mod schema;
pub mod model;

use serenity::model::id::{ UserId, GuildId };

use self::schema::accounts;
use self::schema::user_roles;
use self::schema::todo;

use self::model::{ Account, NewAccount, NewUserRole, UserRole, TODO, NewTODO };

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
  let database_url = "postgresql://cnd@localhost:26257/sasaki_users";
  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

pub fn register(new_id: i64, new_guild: i64, new_roles: &Vec<i64>) {
  let connection = establish_connection();
  let ifresults = accounts::dsl::accounts
      .filter(accounts::dsl::id.eq(new_id))
      .load::<Account>(&connection);
  match ifresults {
    Ok(results) => {
      if results.len() > 0 {
        warn!("registration declined, already exists, processing roles update");
      } else {
        let new_acc = NewAccount {
          id: new_id,
          guild: new_guild,
          mute: false
        };
        let _na : Account =
          diesel::insert_into(accounts::table)
            .values(&new_acc)
            .get_result(&connection)
            .expect("Error registering new account");
      }
    }, Err(error) => {
      error!("Error loading accounts {:?}", error);
    }
  }
  for r in new_roles {
    if let Ok(ifrole) = user_roles::dsl::user_roles
        .filter(user_roles::dsl::id.eq(new_id))
        .filter(user_roles::dsl::role_id.eq(r))
        .load::<UserRole>(&connection) {
      if ifrole.len() == 0 {
        let user_role = NewUserRole {
          id: new_id,
          role_id: r.clone()
        };
        let _nr : UserRole =
          diesel::insert_into(user_roles::table)
            .values(&user_role)
            .get_result(&connection)
            .expect("Error registering new account roles");
      }
    }
  }
}

pub fn update_member(user_id: i64, user_guild: i64, user_roles: &Vec<i64>) {
  let connection = establish_connection();
  let ifresults = accounts::dsl::accounts
      .filter(accounts::dsl::id.eq(user_id))
      .load::<Account>(&connection);
  match ifresults {
    Ok(results) => {
      if results.len() == 0 {
        let new_acc = NewAccount {
          id: user_id,
          guild: user_guild,
          mute: false
        };
        let _na : Account =
          diesel::insert_into(accounts::table)
            .values(&new_acc)
            .get_result(&connection)
            .expect("Error registering new account");
      }
    }, Err(error) => {
      error!("Error loading accounts {:?}", error);
    }
  }
  if let Err(err) =
    diesel::delete(user_roles::table)
      .filter(user_roles::dsl::id.eq(user_id))
      .execute(&connection) {
    error!("error removing from user_roles: {:?}", err);
  }
  for r in user_roles {
    if let Ok(ifrole) = user_roles::dsl::user_roles
        .filter(user_roles::dsl::id.eq(user_id))
        .filter(user_roles::dsl::role_id.eq(r))
        .load::<UserRole>(&connection) {
      if ifrole.len() == 0 {
        let user_role = NewUserRole {
          id: user_id,
          role_id: r.clone()
        };
        let _nr : UserRole =
          diesel::insert_into(user_roles::table)
            .values(&user_role)
            .get_result(&connection)
            .expect("Error registering new account roles");
      }
    }
  }
}

pub fn reset_roles(serenity_user_id: UserId, serenity_guild_id: GuildId) -> Vec<u64> {
  let old_id : i64 = serenity_user_id.as_u64().clone() as i64;
  let old_guild : i64 = serenity_guild_id.as_u64().clone() as i64;
  let connection = establish_connection();
  let ifresults = accounts::dsl::accounts
      .filter(accounts::dsl::id.eq(old_id))
      .filter(accounts::dsl::guild.eq(old_guild))
      .load::<Account>(&connection);
  let mut result : Vec<u64> = Vec::new();;
  match ifresults {
    Ok(results) => {
      if results.len() == 0 {
        warn!("user not found");
      } else {
        let user = &results[0];
        if let Ok(ifroles) = user_roles::dsl::user_roles
            .filter(user_roles::dsl::id.eq(user.id))
            .load::<UserRole>(&connection) {
          for role in ifroles {
            result.push(role.role_id as u64);
          }
        }
      }
    }, Err(error) => {
      error!("Error loading accounts {:?}", error);
    }
  }
  result
}

pub fn lookup() -> String {
  let connection = establish_connection();
  let results = accounts::dsl::accounts.load::<Account>(&connection)
      .expect("Error loading accounts");

  let mut res = format!("{} accounts\n", results.len());
  for acc in results {
    res = format!("{}{}: {}{}\n", res, acc.id, acc.guild, if acc.mute { " (muted)" } else { "" });
  }
  res
}

pub fn todo(user_id: i64) -> String {
  let connection = establish_connection();
  let results = todo::dsl::todo
      .filter(todo::dsl::user_id.eq(user_id))
      .load::<TODO>(&connection)
      .expect("Error loading todo list");
  let mut res = format!("{} items\n", results.len());
  let mut row_i = 1;
  for i in results {
    res = format!("{}{}. {}\n", res, row_i, i.text);
    row_i += 1;
  }
  res
}

pub fn todo_rm(user_id: i64, number : usize) {
  let connection = establish_connection();
  let results = todo::dsl::todo
      .filter(todo::dsl::user_id.eq(user_id))
      .load::<TODO>(&connection)
      .expect("Error loading todo list");
  if number <= results.len() {
    if let Err(err) =
      diesel::delete(todo::table)
        .filter(todo::dsl::id.eq(results[number - 1].id))
        .execute(&connection) {
      error!("error removing from TODO list: {:?}", err)
    }
  }
}

pub fn todo_add(user_id: i64, text: String) {
  let connection = establish_connection();
    let new_todo = NewTODO {
      user_id: user_id,
      text: text
    };
    let _na : TODO =
      diesel::insert_into(todo::table)
        .values(&new_todo)
        .get_result(&connection)
        .expect("Error registering new account");
}
