# официальный образ Rust для сборки
FROM rust:latest as builder

# копирование файлов описывающих зависимости
WORKDIR /app
COPY ./server/Cargo.toml ./server/Cargo.lock ./

# создание dummy-файла проекта и установка зависимостей
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# копирование и сборка исходного кода
COPY ./server/src ./src
RUN cargo build --bin server --release

# ubuntu чтобы не было проблем с библиотеками языка С
FROM ubuntu:latest
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/server /usr/local/bin/server

# запуск сервера
CMD ["server"]