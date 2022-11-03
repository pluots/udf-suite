# udf-suite

A collection of UDFs for MariaDB & MySQL, written using the rust
[`udf`](docs.rs/udf) library.

## Included UDFs

### UUID

### Jsonify

Provide the function `jsonify`


## Building

To build in docker:

```sh
docker run --rm -it \
  -v "$(pwd):/build" \
  -e CARGO_HOME=/build/.docker-cargo \
  rustlang/rust:nightly \
  bash -c "cd /build; cargo build --release"
```

To run:

```bash
# Add -d to the arguments if you don't want to keep the window open
docker run --rm -it  \
  -v $(pwd)/target:/target \
  -e MARIADB_ROOT_PASSWORD=banana \
  --name mariadb_udf_suite \
  mariadb
```

```bash
docker exec -it mariadb_udf_suite bash
```

```bash
cp /target/release/libudf*.so /usr/lib/mysql/plugin/
mysql -pbanana
```


```sql
CREATE FUNCTION jsonify RETURNS string SONAME 'libudf_jsonify.so';
```
