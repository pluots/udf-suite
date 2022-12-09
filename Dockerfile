# Dockerfile to build the udf-examples crate and load it. Usage:
#
# ```
# # build image
# docker build . --tag mdb-udf-suite
# # Run image
# docker run --rm -e MARIADB_ROOT_PASSWORD=example mdb-udf-suite
# ```

FROM rust:latest AS build

WORKDIR /build

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release \
    && mkdir /output \
    && cp target/release/*.so /output

FROM mariadb:10.9

COPY --from=build /output/* /usr/lib/mysql/plugin/
