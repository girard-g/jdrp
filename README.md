![Rust](https://github.com/Evergreenn/jdrp/workflows/Rust/badge.svg)

# JDRP App

This is a basic web d&d game app where backend communication is made through web socket.

## Tech Stack

- [rust](https://github.com/rust-lang/rust) (1.49-nightly) main backend language
- [sqlite](https://github.com/sqlite/sqlite) database
- [ws_rs](https://github.com/housleyjk/ws-rs) web socket communication
- [rocket](https://rocket.rs/) web framework
- [diesel](http://diesel.rs/) ORM
- [frontend](https://fr.reactjs.org/) react
- [logger](https://github.com/Geal/rust-syslog) syslog, proccess name `jdrp`

## Install

```sh
$ echo 'DATABASE_URL=database/index.db' > .env # Setup database for SQLite
$ cargo run # set ROCKET_CLI_COLOR=off if you don't want emojis or colors in your syslogs
```

## Run

If you are running on wsl don't forget to start syslog

```sh
$ sudo service rsyslog start
```

Visit http://localhost:8000
