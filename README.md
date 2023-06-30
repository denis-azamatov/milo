# Setup
Copy .example.env and rename to .env

Copy configuration.json and rename to configuration.local.json

Install cargo-make:
```
cargo install --force cargo-make
```

Install sqlx-cli:
```
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

# Migrations
Set DATABASE_URL Environment Variable to persistent local database:
```powershell
    $env:DATABASE_URL="postgres://postgres:123456@localhost:5432/milo"
```

Create Database:
```
sqlx database create
```

Create migration:
```
sqlx migrate add migration_name
```

Run migration:
```
sqlx migrate run
```

# Test
The test are run with database in a docker container with port 8001.
A separate database will be created for each test.
To compile sqlx, an additional database is created with all migrations applied.

Run test
```
cargo make local-test
```