# diesel-demo-postgresql
# POSTGRESQL SETUP

### DB example of beginners for beginners

 ### Installation of PostgreSQL
 * [https://www.postgresql.org/download/](here)

 ### Installation of diesel
 * [https://diesel.rs/guides/getting-started](here)

 ### Installation of rust
 * [https://www.rust-lang.org/tools/install](here)


 ### ProgreSQL Setup
 * Open as postgres user
 - `sudo -iu postgres`
 * Initializing the database of regional and specify dirs to save db datas in this case /var/lib/..
 - `initdb --locale $LANG -E UTF8 -D '/var/lib/postgres/data'`
 - `exit`


 ### Startup the service
 * Starting postgres service
 - `sudo systemctl start postgresql`
 * Setup to running when the os is initializing
 - `sudo systemctl enable postgresql`


 ### Get access on PostgreSQL and make the DB
 - `sudo -iu postgres`
 - `psql`
 * Make a database and user
 - `CREATE DATABASE my_own_db;`
 - `CREATE USER diesel_user WITH PASSWORD 'default';`
 - `GRANT ALL PRIVILEGES ON DATABASE my_own_db;`
 * \(!\) Important step, you need specify that you are also the owner of the db in addition to having the permissions.
 - `ALTER DATABASE my_own_db OWNER TO diesel_user;`
 - `\q`
 - `exit`

### Inicializar Diesel
 * Set enviroment vars on files
 - `echo "DATABASE_URL=postgres://diesel_user:default@localhost/my_own_db" >> .env`
 - `diesel setup`
