Sasaki the stupid bot
---------------------

Building and running:
```SHELL
cargo update
RUST_BACKTRACE=1 cargo run
```

Creating/using distributed database (suggested version 2.1.6+)
```SHELL
emerge dev-db/cockroach
cockroach start --insecure --store=sasaki --listen-addr=localhost
```

Create database scheme (tables)
```SHELL
cockroach user set gay --insecure
cockroach sql --insecure -e 'CREATE DATABASE sasaki_users'
cockroach sql --insecure -e 'GRANT ALL ON DATABASE sasaki_users TO gay'
cockroach sql --insecure --database=sasaki_users --user=gay -e 'CREATE TABLE accounts (id bigint PRIMARY KEY, guild bigint, mute boolean default false)'
cockroach sql --insecure --database=sasaki_users --user=gay -e 'CREATE TABLE user_roles (id bigint, role_id bigint, PRIMARY KEY (id, role_id))'
cockroach sql --insecure --database=sasaki_users --user=gay -e 'CREATE TABLE todo (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), user_id bigint, text text, FOREIGN KEY (user_id) REFERENCES accounts (id))'
```
