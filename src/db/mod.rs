pub mod schema;
pub mod model;

use serenity::model::id::{UserId, GuildId};

use self::schema::accounts;
use self::schema::user_roles;
use self::schema::todo;

use self::model::{Account, NewAccount, NewUserRole, UserRole, TODO};

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
      if results.len() > 0 {
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

pub fn todo() -> String {
  let connection = establish_connection();
  let results = todo::dsl::todo.load::<TODO>(&connection)
      .expect("Error loading todo list");

  let mut res = format!("{} items\n", results.len());
  for i in results {
    res = format!("{}\n", i.text);
  }
  res
}
