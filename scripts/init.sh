#!/bin/bash
sqlx database create --database-url $DATABASE_URL
sqlx migrate run --database-url $DATABASE_URL
./service-product --addr 0.0.0.0:3000
