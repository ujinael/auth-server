#!/bin/bash
    # Выводим значения переменных окружения для отладки
    echo "PORT: $PORT"
    echo "DATABASE_URL: $DATABASE_URL"
    echo "POSTGRES_AUTH_HOST: $POSTGRES_AUTH_HOST"
    echo "POSTGRES_AUTH_PORT: $POSTGRES_AUTH_PORT"
    echo "REDIS_URL: $REDIS_URL"

# Ожидание готовности Redis
until redis-cli -u $REDIS_URL ping; do
    echo "Waiting for Redis (auth)..."
    sleep 1
done
# Ожидание готовности PostgreSQL
# until pg_isready -h $POSTGRES_AUTH_HOST -p $POSTGRES_AUTH_PORT; do
#   echo "Waiting for PostgreSQL (auth)..."
#   sleep 1
# done

# Выполнение миграций
# sqlx migrate run --database-url $DATABASE_URL

# Сборка и запуск сервера
cargo sqlx prepare
# cargo build &&
cargo run
