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
