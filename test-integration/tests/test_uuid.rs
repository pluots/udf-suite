#![cfg(feature = "backend")]

// mod backend;

// use backend::get_db_connection;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Text;
use test_integration::get_db_connection;

const SETUP: [&str; 3] = [
    "create or replace function uuid_generate_v1
        returns string
        soname 'libudf_uuid.so'",
    // "create or replace function uuid_generate_v1
    //     returns string
    //     soname 'libudf_uuid.so'",
    "create or replace function uuid_generate_v4
        returns string
        soname 'libudf_uuid.so'",
    "create or replace function uuid_nil
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
        .expect("bad result");

    assert_eq!(res.0, "00000000-0000-0000-0000-000000000000");
}

#[test]
fn test_ns_dns() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_ns_dns()")
        .get_result(conn)
        .expect("bad result");

    assert_eq!(res.0, "6ba7b810-9dad-11d1-80b4-00c04fd430c8");
}

#[test]
fn test_ns_url() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_ns_url()")
        .get_result(conn)
        .expect("bad result");

    assert_eq!(res.0, "7fed185f-0864-319f-875b-a3d5458e30ac");
}

#[test]
fn test_ns_oid() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_ns_oid()")
        .get_result(conn)
        .expect("bad result");

    assert_eq!(res.0, "6ba7b812-9dad-11d1-80b4-00c04fd430c8");
}

#[test]
fn test_ns_x500() {
    let conn = &mut get_db_connection(&SETUP);

    let res: (String,) = sql::<(Text,)>("select uuid_ns_x500()")
        .get_result(conn)
        .expect("bad result");

    assert_eq!(res.0, "6ba7b814-9dad-11d1-80b4-00c04fd430c8");
}
