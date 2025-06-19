# Сервер авторизации с помощью JWT и Basic Auth
##  Старт проекта
  1. Создать env файл
    ```
      DATABASE_URL=postgres://postgres:password@localhost:5432/db_name
      PORT=YOUR_PORT_NUMBER
      REDIS_URL=redis://localhost
      JWT_SECRET="some_secret_key"
      ACCESS_TOKEN_EXP=15
      REFRESH_TOKEN_EXP=720
    ```
  2. Запустить сервер базы данных postgres локально или в докер контейнере
  3. Создать базу данных с именем db_name
  4. Запустить миграции через терминал// sqlx migrate run
  5. Запустить сервер через терминал// cargo run
