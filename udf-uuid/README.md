# UUID Module

Provide SQL functions to generate various types of UUIDs. This is designed to
mimic Postgres' [uuid-osp] library.

[uuid-osp]: https://www.postgresql.org/docs/current/uuid-ossp.html

## UUID Types

* V1: MAC address + timestamp + small random portion. The MAC address and
  timestamp can be determined from a UUID
* V3: a MD5 hash of a "namespace" UUID and "name" data. This is fully
  deterministic, there is no random component.
* V5: same as V3, uses SHA1 instead
* V4: fully random UUID

## Available Functions

The available functions that return a variable UUID are:

* `uuid_generate_v1()`: Generate a V1 UUID using this node's MAC address
* `uuid_generate_v1mc()`: Generate a V1 UUID using a random multicast MAC address
<!-- * `uuid_generate_v1arg(some_mac)`: Generate a V1 UUID using a specified MAC
  address
* `uuid_generate_v3(namespace, name)`: Generate a V3 UUID from a `namespace`
  UUID and `name` data. For example, `uuid_generate_v3(uuid_ns_url(), 'some
  text')` -->
* `uuid_generate_v4()`: Generate a random V4 UUID
<!-- * `uuid_generate_v5(namespace, name)`: Generate a V5 UUID. This is similar to V3
  but uses SHA1 instead of MD5. -->

There are also some functions that return constant values:

* `uuid_nil()`: Return the `nil` UUID (all zeroes)
* `uuid_ns_dns()`: Return the DNS namespace UUID (used for V3/V5 UUIDs)
* `uuid_ns_url()`: Return the URL namespace UUID (used for V3/V5 UUIDs)
* `uuid_ns_oid()`: Return the ISO OID namespace UUID
* `uuid_ns_x500()`: Return the X.500 namespace UUID

And a helper function:


## Usage

```sql
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
