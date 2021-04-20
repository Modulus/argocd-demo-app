FROM rust:1.51-alpine3.13
WORKDIR /opt/app
COPY . /opt/app/
RUN ls -la /opt/app
RUN apt update && apt install  build-essential -y
#postgresql-client -y
# build-essential zlib1g-dev openssl -y
RUN cargo build --release && cp -r /opt/app/gui /opt/app/target/release

FROM ubuntu:20.04 as runner
WORKDIR /opt/app/target/release
RUN apt update && apt install postgresql-client -y
COPY --from=builder /opt/app/target/release /opt/app/target/release
RUN ls -a /opt/app/target/release
ENTRYPOINT ["/opt/app/target/release/main"]
EXPOSE 8000