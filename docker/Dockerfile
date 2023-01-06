FROM rust:buster as builder
WORKDIR /app

RUN rustup default nightly && \
	rustup target add wasm32-unknown-unknown --toolchain nightly

RUN apt-get update && \
	apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
	apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev protobuf-compiler

COPY . .

RUN make build

# =============

FROM phusion/baseimage:focal-1.2.0
LABEL maintainer="sweatpotato13@gmail.com"

RUN useradd -m -u 1000 -U -s /bin/sh -d /parachain-template-node parachain-template-node

COPY --from=builder /app/target/release/parachain-template-node /usr/local/bin

RUN ldd /usr/local/bin/parachain-template-node && \
	/usr/local/bin/parachain-template-node --version

# Shrinking
RUN rm -rf /usr/lib/python* && \
	rm -rf /usr/sbin /usr/share/man

USER parachain-template-node
EXPOSE 30333 9933 9944

RUN mkdir /parachain-template-node/data

VOLUME ["/parachain-template-node/data"]

ENTRYPOINT ["/usr/local/bin/parachain-template-node"]