# Dockerfile to build the udf-examples crate and load it. Build with:
#
# ```
# docker build . --tag mdb-udf-suite
# ```
#
# And run with
#
# ```
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
