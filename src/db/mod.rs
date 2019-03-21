pub mod schema;
pub mod model;

use self::schema::accounts;
use self::schema::accounts::dsl::*;
use self::model::{Account, NewAccount};

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn establish_connection() -> PgConnection {
  let database_url = "postgresql://root@localhost:26257/sasaki";
  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

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
