FROM rust:1.31.1-stretch AS builder

RUN apt update && apt install -y make
RUN rustup target install x86_64-unknown-linux-musl

WORKDIR /misspell
COPY ./ ./
RUN make build_static

RUN useradd -ms /bin/bash bloom

####################################################################################################
## Image
####################################################################################################

FROM scratch

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /misspell/target/x86_64-unknown-linux-musl/release/misspell /bin/misspell

USER bloom

WORKDIR /misspell

CMD ["/bin/misspell"]
