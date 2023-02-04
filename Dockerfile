FROM rust:latest

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/telegram-todo-bot"]