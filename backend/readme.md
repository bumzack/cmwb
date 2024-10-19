# postgres install mac
```
brew install libpq postgresql@16
brew link postgresql@16
cargo install diesel_cli --no-default-features --features postgres
```

```
  echo 'export PATH="/opt/homebrew/opt/postgresql@16/bin:$PATH"' >> ~/.zshrc
```


  ## postgres commands

  https://www.codementor.io/@engineerapart/getting-started-with-postgresql-on-mac-osx-are8jcopb


```
CREATE ROLE cmwb WITH LOGIN PASSWORD 'cmwb';
```

list users
```
\du
```

ALTER ROLE cmwb CREATEDB;


quit


\q


psql postgres -U cmwb


CREATE DATABASE cmwb;


GRANT ALL PRIVILEGES ON DATABASE cmwb TO cmwb;

\connect cmwb



## requests


### insert user

```
curl http://localhost:3000/user/create -vvv -X POST -d '{ "username": "bumzack", "password": "bumzack" }' -H "Content-Type: application/json" 
 ```

 
 ### list users
 
```
curl http://localhost:3000/user/list    | jq  
```


### login

```
  curl -vvvv \
  -w '\n' \
  -H 'Content-Type: application/json' \
  -d '{"client_id":"foo","client_secret":"bar"}' \
  http://localhost:3000/user/login   | jq 
```