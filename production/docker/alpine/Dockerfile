FROM rust:alpine 

WORKDIR /vrs

RUN apk update
RUN apk add make
COPY . .
RUN cargo clean
RUN rm -rf {production,setup.sh}
RUN mkdir -p /var/www/static/ && mkdir /var/www/logs/
RUN cp ./media/* /var/www/static/
RUN rm -rf ./media/
RUN make build

CMD ["make", "run"]

EXPOSE 80/tcp
