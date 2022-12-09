# tests-integration

This crate handles integration testing.

It will set the expected database URI whatever environment variable
`UDF_TEST_BACKEND_URI` is set to, defaulting to
`mysql://root:example@0.0.0.0:12305/udf_tests`
