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

see:  https://www.codementor.io/@engineerapart/getting-started-with-postgresql-on-mac-osx-are8jcopb


```
CREATE ROLE cmwb WITH LOGIN PASSWORD 'cmwb';
```

### list users
```
\du
```


```
ALTER ROLE cmwb CREATEDB;
```

```
quit
```


```
\q
```

```
psql postgres -U cmwb
```

```
CREATE DATABASE cmwb;
```

```
GRANT ALL PRIVILEGES ON DATABASE cmwb TO cmwb;
```
```
\connect cmwb
```


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
curl -vvvv  http://localhost:3000/user/login   \
  -w '\n' \
  -H 'Content-Type: application/json' \
  -d '{"client_id":"foo","client_secret":"bar"}'  | jq

```

### list users as logged in user
```
curl -vvv   http://localhost:3000/user/list \
     -H 'Content-Type: application/json' \
     -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjb21wYW55IjoiY213YiIsImV4cCI6MjAwMDAwMDAwMCwidXNlcm5hbWUiOiJmb28ifQ.RPNOkrPjWuNLSEqeAZXNPwypkqPLMaleuGkeD-08Pf8'   | jq
```
