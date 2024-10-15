# официальный образ Rust для сборки
FROM rust:latest as builder

# копирование файлов описывающих зависимости
WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# создание dummy-файла проекта и установка зависимостей
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# копирование и сборка исходного кода
COPY ./src ./src
RUN cargo build --bin t1 --release

# ubuntu чтобы не было проблем с библиотеками языка С
FROM ubuntu:latest
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/t1 /usr/local/bin/t1

# запуск сервера
CMD ["t1"]