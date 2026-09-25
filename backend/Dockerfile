FROM rust:1.82-bookworm AS build
WORKDIR /src
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /src/target/release/consensus-backend /usr/local/bin/consensus-backend
EXPOSE 8080
CMD ["consensus-backend"]
