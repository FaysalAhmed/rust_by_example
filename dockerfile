FROM rust:1.63

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["myapp"]