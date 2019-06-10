[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/cynede/sasaki.svg)](http://isitmaintained.com/project/cynede/sasaki "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/cynede/sasaki.svg)](http://isitmaintained.com/project/cynede/sasaki "Percentage of issues still open")
[![Gentoo discord server](https://img.shields.io/discord/545563271017791488.svg?style=flat-square&label=Cynede)](https://discord.gg/rKZfynu)
[![Build Status](https://travis-ci.org/cynede/sasaki.svg?branch=master)](https://travis-ci.org/cynede/sasaki)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fcynede%2Fsasaki.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fcynede%2Fsasaki?ref=badge_shield)

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
cockroach user set cnd --insecure
cockroach sql --insecure -e 'CREATE DATABASE sasaki_users'
cockroach sql --insecure -e 'GRANT ALL ON DATABASE sasaki_users TO cnd'
cockroach sql --insecure --database=sasaki_users --user=cnd -e 'CREATE TABLE accounts (id bigint PRIMARY KEY, guild bigint, mute boolean default false)'
cockroach sql --insecure --database=sasaki_users --user=cnd -e 'CREATE TABLE user_roles (id bigint, role_id bigint, PRIMARY KEY (id, role_id))'
cockroach sql --insecure --database=sasaki_users --user=cnd -e 'CREATE TABLE todo (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), user_id bigint, text text, FOREIGN KEY (user_id) REFERENCES accounts (id))'
```


## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fcynede%2Fsasaki.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fcynede%2Fsasaki?ref=badge_large)