FROM rust:1.76 

WORKDIR /app

COPY . .

RUN cargo install --path .

EXPOSE 3000

CMD ["rust-microservice"]