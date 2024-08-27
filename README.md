
# Database Setup
Since this project is deployed on Fedora39 I refer to :https://docs.fedoraproject.org/en-US/quick-docs/postgresql/.
The setup on other distros follows the same steps, changing the package manager. 

## Installing the DB-Server
1. ```sudo dnf install postgresql-server postgresql-contrib```
2. ```sudo systemctl enable postgresql```
3. ```sudo postgresql-setup --initdb --unit postgresql```
4. ```sudo systemctl start postgresql```
## User and DB Creation
1. ```sudo -u postgres psql```
2. ```postgres=# CREATE USER your_user_account WITH PASSWORD 'some_pw';```

3. ```postgres=# CREATE DATABASE my_project OWNER lenny```
4. ```postgres=# \password postgres``` 
5. ```psql my_project```
## Changing Identification
```sudo nvim /var/lib/pgsql/data/pg_hba.conf```
Change: 
```
# TYPE  DATABASE        USER            ADDRESS                 METHOD

# "local" is for Unix domain socket connections only
local   all             all                                     peer
# IPv4 local connections:
host    all             all             127.0.0.1/32            ident
# IPv6 local connections:
host    all             all             ::1/128                 ident
```
 To:
```
# "local" is for Unix domain socket connections only
local   all             all                                     peer
# IPv4 local connections:
host    all             all             127.0.0.1/32            md5
# IPv6 local connections:
host    all             all             ::1/128                 md5
```
# Requirements

DB-Migrations need to be executed ahead of calling the query! macro otherwise the macro will fail and abort compilation. ```cargo install sqlx-cli```

I refer to: https://book.leptos.dev/ and https://github.com/leptos-rs/cargo-leptos/blob/main/README.md for a more detailed setup. The minimum Requirements include:

1. ```cargo install cargp-leptos```
2. ```rustup target add wasm32-unknown-unknown```
In order to handle chnages in the .sql files without having to change the code :  
1. ```cargo install slqx-cli```
2. ```migrate build-script``` 
 ## Quick Start
Run ```cargo leptos watch``` to run the application.
