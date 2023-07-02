# Setup
Copy .example.env and rename to .env. This step is required for sqlx compilation.

Copy configuration.json and rename to configuration.local.json. Optional.

Install cargo-make:
```
cargo install --force cargo-make
```

Install sqlx-cli:
```
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

# Migrations
Change DATABASE_URL Environment Variable in .env to persistent local database:
```powershell
    DATABASE_URL="postgres://postgres:123456@localhost:5432/milo"
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

# Run
Run application:
```
cargo run
```

For pretty printed logs, use Git Bash and bunyan:
```
cargo run | bunyan
```

# Test
The test are run with database in a docker container with port 8001.
A separate database will be created for each test.
To compile sqlx, an additional database is created with all migrations applied.

Run test:
```
cargo make local-test
```
Run test with console logging:
```
cargo make local-test-log
```

# Logs
The logs are written in JSON format, which can be converted using **bunyan**.

Install bunyan:
```
npm install -g bunyan
```

Convert logs:
```
bunyan log_file_name > output_file_name
```