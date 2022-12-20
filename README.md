# udf-suite

A collection of UDFs for MariaDB & MySQL, written using the rust [`udf`]
library. For instructions on how to use these libraries, jump to the
[Installation](#installation) section.

[`udf`]: docs.rs/udf

## Included UDFs

### UUID

Provide UUID functions similar to the Postges [`uuid-osp`] package:

- Generate v1 and v4 UUIDs (v3 & v5 coming soon)
- Validate UUIDs
- Create namespace UUIDs

See the [UUID Readme](/udf-uuid/README.md) for more information

[`uuid-osp`]: https://www.postgresql.org/docs/current/uuid-ossp.html

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


## Lipsum

Uses the [lipsum crate] to generate lipsum strings with a specified word count.


```
MariaDB [(none)]> select lipsum(10);
+------------------------------------------------------------------+
| lipsum(10)                                                       |
+------------------------------------------------------------------+
| Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do. |
+------------------------------------------------------------------+
1 row in set (0.000 sec)
```

[lipsum crate]: https://docs.rs/lipsum/latest/lipsum/


## Installation

Compiled library binaries can be downloaded from this library's [releases] page.
The desired files can be copied to the plugin directory (usually
`/usr/lib/mysql/plugin`) and selectively loaded:

```sql
CREATE FUNCTION jsonify RETURNS string SONAME 'libudf_jsonify.so';
CREATE FUNCTION lipsum RETURNS string SONAME 'libudf_lipsum.so';
CREATE FUNCTION uuid_generate_v1 RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_generate_v1mc RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_generate_v4 RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_nil RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_ns_dns RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_ns_url RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_ns_oid RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_ns_x500 RETURNS string SONAME 'libudf_uuid.so';
CREATE FUNCTION uuid_is_valid RETURNS integer SONAME 'libudf_uuid.so';
```

Note that Windows `.dll`s are built but have not been tested - please open an
issue if you encounter any errors.

[releases]: https://github.com/pluots/udf-suite/releases


## Building from Source

To build the binaries yourself, you can clone this repository and run:

```sh
cargo build --release
```

Which will produce the desired dynamic library files in `target/release`.
Specific functions can also be specified with `-p` (e.g.
`cargo build --release -p udf-uuid`).

This repository also comes with a docker file that simplifies getting an image
up and running:

```sh
# build the image
docker build . --tag mdb-udf-suite

# run it
docker run --rm -d \
  -e MARIADB_ROOT_PASSWORD=example \
  --name mariadb_udf_suite \
  mdb-udf-suite

# Enter a SQL shell
docker exec -it mariadb_udf_suite mysql -pexample
```

The UDFs can then be loaded using the `CREATE FUNCTION` statements above.
