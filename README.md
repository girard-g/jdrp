![Rust](https://github.com/Evergreenn/RustChatApp/workflows/Rust/badge.svg)

# JDRP App

This is a basic web d&d game app where backend communication is made through web socket.

## Tech Stack

- [rust](https://github.com/rust-lang/rust) (1.49-nightly) main backend language
- [sqlite](https://github.com/sqlite/sqlite) database
- [ws_rs](https://github.com/housleyjk/ws-rs) web socket communication
- [rocket](https://rocket.rs/) web framework
- [diesel](http://diesel.rs/) ORM
- [frontend](#) static html/css/js

## Install

```sh
$ echo 'DATABASE_URL=database/index.db' > .env # Setup database for SQLite
$ cargo run
```

Visit http://localhost:8000

<!-- ## Todo
- [ ] add rooms
- [ ] sent proper formatted objects over the websocket for getting all displayed information
- [ ] create proper admin messages
- [ ] check gif apis
- [ ] remove the 5 users limitation -->