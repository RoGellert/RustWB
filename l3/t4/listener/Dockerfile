# официальный образ Rust для сборки
FROM rust:latest as builder

# копирование файлов описывающих зависимости
WORKDIR /app
COPY ./listener/Cargo.toml ./listener/Cargo.lock ./

# создание dummy-файла проекта и установка зависимостей
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# копирование и сборка исходного кода
COPY ./listener/src ./src
RUN cargo build --bin listener --release

# ubuntu чтобы не было проблем с библиотеками языка С
FROM ubuntu:latest
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/listener /usr/local/bin/listener

# запуск сервера
CMD ["listener"]