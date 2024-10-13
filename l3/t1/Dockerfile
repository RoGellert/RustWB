# официальный образ Rust для сборки
FROM rust:latest as builder

# ицициализация нового проекта
RUN cargo new --bin app

# копирование файлов описывающих зависимости
WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# установка и сборка зависимостей
RUN cargo build --release
RUN rm -rf src && mkdir src

# копирование и сборка исходного кода
COPY ./src ./src
RUN cargo build --bin l3_t1 --release

# ubuntu чтобы не было проблем с библиотеками языка С
FROM ubuntu:latest
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/l3_t1 /usr/local/bin/l3_t1

# запуск сервера
CMD ["l3_t1"]