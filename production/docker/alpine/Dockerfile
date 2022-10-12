FROM rust:alpine 

WORKDIR /vrs

RUN apk update
RUN apk add make
COPY . .
RUN cargo clean
RUN rm -rf {production,docs}
RUN make migrate
RUN rm -rf ./media
RUN cargo build --release

CMD ["cargo", "run", "--release"]

EXPOSE 80/tcp
