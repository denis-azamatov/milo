# Database
## Setup
### Install sqlx-cli:
```
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

### Set Environment Variable:
```powershell
    $env:DATABASE_URL="postgres://postgres:123456@localhost:5432/milo"
```

### Create Database:
```
sqlx database create
```

## Migrations
### Create
```
sqlx migrate add migration_name
```
### Run
```
sqlx migrate run
```