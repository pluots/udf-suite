#![cfg(feature = "backend")]

// mod backend;

// use backend::get_db_connection;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Text;
use test_integration::get_db_connection;
use uuid::Uuid;

const SETUP: [&str; 3] = [
    "create or replace function uuid_generate_v1
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_generate_v1mc
        returns string
        soname 'libudf_uuid.so'",
    // "create or replace function uuid_generate_v1
    //     returns string
    //     soname 'libudf_uuid.so'",
    "create or replace function uuid_generate_v4
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_generate_v6
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_generate_v7
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_nil
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_max
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_ns_dns
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_ns_url
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_ns_oid
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_ns_x500
        returns string
        soname 'libudf_uuid.so'",
];

#[test]
fn test_nil() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_nil()")
        .get_result(conn)
        .unwrap();

    assert_eq!(res.0, "00000000-0000-0000-0000-000000000000");
    assert_eq!(res.0, Uuid::nil().hyphenated().to_string());
}

#[test]
fn test_max() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_max()")
        .get_result(conn)
        .unwrap();

    assert_eq!(res.0, "ffffffff-ffff-ffff-ffff-ffffffffffff");
    assert_eq!(res.0, Uuid::max().hyphenated().to_string());
}

#[test]
fn test_ns_dns() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_ns_dns()")
        .get_result(conn)
        .unwrap();

    assert_eq!(res.0, "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res.0, Uuid::NAMESPACE_DNS.hyphenated().to_string());
}

#[test]
fn test_ns_url() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_ns_url()")
        .get_result(conn)
        .unwrap();

    assert_eq!(res.0, "7fed185f-0864-319f-875b-a3d5458e30ac");
    assert_eq!(res.0, Uuid::NAMESPACE_URL.hyphenated().to_string());
}

#[test]
fn test_ns_oid() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_ns_oid()")
        .get_result(conn)
        .unwrap();

    assert_eq!(res.0, "6ba7b812-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res.0, Uuid::NAMESPACE_OID.hyphenated().to_string());
}

#[test]
fn test_ns_x500() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_ns_x500()")
        .get_result(conn)
        .unwrap();

    assert_eq!(res.0, "6ba7b814-9dad-11d1-80b4-00c04fd430c8");
    assert_eq!(res.0, Uuid::NAMESPACE_X500.hyphenated().to_string());
}

#[test]
fn test_generate_v1() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_generate_v1()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(res.0).unwrap();

    assert_eq!(uuid.get_version_number(), 1);
}

#[test]
fn test_generate_v1mc() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_generate_v1mc()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(res.0).unwrap();

    assert_eq!(uuid.get_version_number(), 1);
}

#[test]
fn test_generate_v4() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_generate_v4()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(res.0).unwrap();

    assert_eq!(uuid.get_version_number(), 4);
}

#[test]
fn test_generate_v6() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_generate_v6()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(res.0).unwrap();

    assert_eq!(uuid.get_version_number(), 6);

    let node_id = b"abcdef";
    let res: (String,) = sql::<(Text,)>("select uuid_generate_v6()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(res.0).unwrap();

    assert_eq!(uuid.get_version_number(), 6);
    asset!(uuid.as_bytes().ends_with(node_id));
}

#[test]
fn test_generate_v7() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_generate_v7()")
        .get_result(conn)
        .unwrap();

    let uuid = Uuid::try_parse(res.0).unwrap();

    assert_eq!(uuid.get_version_number(), 7);
}

#[test]
fn test_valid() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (i64,) = sql::<(Text,)>("select uuid_is_valid(uuid_generate_v4())")
        .get_result(conn)
        .unwrap();

    assert_eq!(res, 1);
}
