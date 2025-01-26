FROM rust:1.83

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/prod_project"]
