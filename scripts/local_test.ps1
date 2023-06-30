$ErrorActionPreference = 'Stop'

$DB_USER = "postgres"
$DB_PASSWORD = "123456"
$DB_HOST = "localhost"
$DB_PORT = "5432"
$DOCKER_PORT = "8001"
$DB_NAME = "milo_test_$(New-Guid)"

$RUNNING_POSTGRES_CONTAINER = $(docker ps --filter 'name=postgres' --format '{{.ID}}')
$CONTAINER_NAME = "postgres_milo_test_$(Get-Date -Format "ddMMyyyy_HHmmss")"

if ($RUNNING_POSTGRES_CONTAINER) {
    Write-Output "+ there is a postgres container already running, kill it with:"
    Write-Output "+     docker rm ${RUNNING_POSTGRES_CONTAINER} --force"
    exit 1
}

Invoke-Expression "docker run -e POSTGRES_USER=`"$DB_USER`" -e POSTGRES_PASSWORD=`"$DB_PASSWORD`" -p $($DOCKER_PORT):$($DB_PORT) -d --name `"$($CONTAINER_NAME)`" postgres -N 1000"

if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

Write-Output "+ Container started: $($CONTAINER_NAME)"

$env:DATABASE_URL = "postgres://$($DB_USER):$DB_PASSWORD@${DB_HOST}:$DOCKER_PORT/$DB_NAME"
Write-Output "+ DATABASE_URL: $($env:DATABASE_URL)"

do {
    Write-Output "+ Waiting while postgres wake up."
    sqlx database create --connect-timeout 2
    Start-Sleep -Seconds 1
} while ($LASTEXITCODE -ne 0)

sqlx migrate run

Write-Output "+ Postgres has started, ready to go!"

cargo test

docker rm $CONTAINER_NAME --force

Write-Output "+ Container removed!"