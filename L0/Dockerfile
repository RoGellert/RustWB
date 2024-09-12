# официальный образ Rust для сборки
FROM rust:latest as builder

# ицициализация нового проекта
RUN cargo new --bin app

# копирование файлов описывающих зависимости
WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# установка с сборка зависимостей
RUN cargo build --release
RUN rm -rf src && mkdir src

# копирование и сборка исходного кода
COPY ./src ./src
RUN cargo build --bin l0 --release

# ubuntu чтобы не было проблем с библиотеками языка С
FROM ubuntu:latest
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/l0 /usr/local/bin/l0

# запуск сервера
CMD ["l0"]