![Rust](https://github.com/Evergreenn/jdrp/workflows/Rust/badge.svg)

# JDRP App

This is a basic web d&d game app where backend communication is made through web socket.

## Tech Stack

- [rust](https://github.com/rust-lang/rust) (1.49-nightly) main backend language
- [sqlite](https://github.com/sqlite/sqlite) database
- [ws_rs](https://github.com/housleyjk/ws-rs) web socket communication
- [rocket](https://rocket.rs/) web framework
- [diesel](http://diesel.rs/) ORM
- [frontend](#) static html/css/js
- [logger](https://github.com/Geal/rust-syslog) syslog, proccess name `jdrp`

## Install

```sh
$ echo 'DATABASE_URL=database/index.db' > .env # Setup database for SQLite
$ cargo run # set ROCKET_CLI_COLOR=off if you don't want emojis or colors in your syslogs
```

Visit http://localhost:8000

## Todo v1.0
### Frontend
- [ ] Define front template
- [ ] Implement frontend framework
- [ ] Migrate vanila code
- [ ] Rework vanila design (connection, character creation, main page)
- [ ] Update all existing informations to main documents ones (characters stats, characters descriptions, ...)
- [ ] Create character panel
- [ ] Create inventory (with all actions related)
- [ ] Create round based fight interface (round process, update after each actions0
- [ ] Create loot interface
- [ ] Create tooltip interface
- [ ] Create GM interface
### Backend
- [x] Create logging system 
- [x] Implement ItemGenerator project 
- [ ] Implement ItemGenerator project for each character based object
- [ ] Implement ItemGenerator project for villages, towns and marchands
- [ ] Create websocket interface
- [ ] Create inventory check and process
- [ ] Create fight event calculation
- [ ] Create websocket binding for GM
### Graphics
- [ ] Characters portrait
- [x] Graphics elements
- [ ] Create logo
### Game Design
- [x] Create game economy
- [ ] Create ennemies core mecanics 
- [ ] Balance race - class stats
- [ ] Balance spells, items ilvl and damage progression