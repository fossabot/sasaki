// cockroach sql --insecure --database=sasaki_users --user=cnd -e 'CREATE TABLE accounts (id bigint PRIMARY KEY, guild bigint, mute boolean default false)'
// cockroach sql --insecure --database=sasaki_users --user=cnd -e 'CREATE TABLE user_roles (user_id bigint, role_id bigint, PRIMARY KEY (user_id, role_id))'
// cockroach sql --insecure --database=sasaki_users --user=cnd -e 'CREATE TABLE todo (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), user_id bigint, text text, FOREIGN KEY (user_id) REFERENCES accounts (id))'

table! {
  accounts {
    id -> BigInt,
    guild -> BigInt,
    mute -> Bool,
  }
}

table! {
  user_roles {
    id -> BigInt,
    role_id -> BigInt,
  }
}

table! {
  todo {
    id -> Uuid,
    user_id -> BigInt,
    text -> Text,
  }
}
