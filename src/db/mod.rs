pub mod schema;
pub mod model;

use serenity::model::id::{UserId, GuildId};

use self::schema::accounts;
use self::schema::accounts::dsl::*;
use self::model::{Account, NewAccount};

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
  let database_url = "postgresql://cnd@localhost:26257/sasaki";
  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

//cockroach sql --insecure --database=sasaki_users --user=cnd -e
// 'CREATE TABLE accounts (id bigint PRIMARY KEY, guild bigint, role bigint, mute boolean default false)'
pub fn register(new_id: i64, new_guild: i64, new_role: i64) -> Option<Account> {
  let connection = establish_connection();
  let results = accounts
      .filter(id.eq(new_id))
      .load::<Account>(&connection)
      .expect("Error loading accounts");
  if results.len() > 0 {
    warn!("registration declined, already exists");
    return None;
  }

  let new_acc = NewAccount {
    id: new_id,
    guild: new_guild,
    role: new_role,
    mute: false
  };

  Some(diesel::insert_into(accounts::table)
        .values(&new_acc)
        .get_result(&connection)
        .expect("Error registering new account"))
}

pub fn reset_role(serenity_user_id: UserId, serenity_guild_id: GuildId) -> Option<u64> {
  let old_id : i64 = serenity_user_id.as_u64().clone() as i64;
  let old_guild : i64 = serenity_guild_id.as_u64().clone() as i64;
  let connection = establish_connection();
  let results = accounts
      .filter(id.eq(old_id))
      .filter(guild.eq(old_guild))
      .load::<Account>(&connection)
      .expect("Error loading accounts");
  if results.len() > 0 {
    warn!("user not found");
    None
  } else {
    let user = &results[0];
    Some(user.role as u64)
  }
}

pub fn lookup() {
  let connection = establish_connection();
  let results = accounts.load::<Account>(&connection)
      .expect("Error loading accounts");

  println!("Displaying {} accounts", results.len());
  for acc in results {
    println!("{}: {}{}", acc.id, acc.guild, if acc.mute { " (muted)" } else { "" });
    println!("----------\n");
    println!("{}", acc.role);
  }
}
