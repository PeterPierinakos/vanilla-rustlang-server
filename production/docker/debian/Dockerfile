FROM rust:bullseye 

WORKDIR /vrs

RUN apt update -y && apt upgrade -y
COPY . .
RUN cargo clean
RUN rm -rf {production,docs}
RUN make migrate
RUN rm -rf ./media/
RUN cargo build --release

CMD ["cargo", "run", "--release"]

EXPOSE 80/tcp
