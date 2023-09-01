FROM rust

WORKDIR /usr/src/api-service
COPY . .

EXPOSE 8000/tcp

RUN cargo install --path .

CMD ["cargo", "run"]