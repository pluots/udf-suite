# udf-suite

A collection of UDFs for MariaDB & MySQL, written using the rust
[`udf`](docs.rs/udf) library.

## Included UDFs

### UUID

### Jsonify

Provide the function `jsonify`, which quickly creates JSON output for any given
inputs.

```
MariaDB [db]> select jsonify(qty, cost, class) from t1 limit 4;
+-------------------------------------+
| jsonify(qty, cost, class)           |
+-------------------------------------+
| {"class":"a","cost":50.0,"qty":10}  |
| {"class":"c","cost":5.6,"qty":8}    |
| {"class":"a","cost":20.7,"qty":5}   |
| {"class":"b","cost":12.78,"qty":10} |
+-------------------------------------+
4 rows in set (0.000 sec)
```

Aliasing also works to change key names:

```
MariaDB [db]> select jsonify(uuid() as uuid, qty as quantity, cost) from t1 limit 4;
+----------------------------------------------------------------------------+
| jsonify(uuid() as uuid, qty as quantity, cost)                             |
+----------------------------------------------------------------------------+
| {"cost":50.0,"quantity":10,"uuid":"45952863-5b4d-11ed-b214-0242ac110002"}  |
| {"cost":5.6,"quantity":8,"uuid":"4595291b-5b4d-11ed-b214-0242ac110002"}    |
| {"cost":20.7,"quantity":5,"uuid":"45952953-5b4d-11ed-b214-0242ac110002"}   |
| {"cost":12.78,"quantity":10,"uuid":"4595297a-5b4d-11ed-b214-0242ac110002"} |
+----------------------------------------------------------------------------+
4 rows in set (0.001 sec)
```



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
