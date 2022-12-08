//! Adds the `jsonify` function to turn any arguments into a JSON string
//!
//! Add with
//!
//! ```sql
//! CREATE FUNCTION jsonify RETURNS string SONAME 'libudf_jsonify.so';
//! ```

use serde_json::{Map, Number, Value};
use udf::prelude::*;
use udf::MaxLenOptions;

#[derive(Default, Debug)]
struct Jsonify(String);

#[register]
impl BasicUdf for Jsonify {
    type Returns<'a> = &'a str;

    /// Just set our maximum length here
    fn init<'a>(cfg: &UdfCfg<Init>, _args: &'a ArgList<'a, Init>) -> Result<Self, String> {
        cfg.set_max_len(MaxLenOptions::Blob as u64);
        Ok(Self::default())
    }

    /// All we do here is map our arguments,
    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let json_map: Map<String, Value> = args
            .iter()
            .map(|a| (a.attribute().to_owned(), res_to_json_val(a.value())))
            .collect();
        self.0 = Value::Object(json_map).to_string();
        Ok(&self.0)
    }
}

/// Convert a `SqlResult` to a `serde_json::Value`
///
/// Return the apropriate type if possible (`String` or `Number`), `Null`
/// otherwise
fn res_to_json_val(source: SqlResult) -> Value {
    match source {
        SqlResult::String(Some(v)) => Value::String(String::from_utf8_lossy(v).into_owned()),
        SqlResult::Real(Some(v)) => Number::from_f64(v)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        SqlResult::Int(Some(v)) => Value::Number(v.into()),
        SqlResult::Decimal(Some(v)) => v.parse().map(Value::Number).unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use udf::mock::*;

    use super::*;

    #[test]
    fn test_empty() {
        // We need to verify that we handle null variables correctly
        let mut cfg = MockUdfCfg::new();
        let mut arglist = mock_args![];
        let mut jsonify = Jsonify::init(cfg.as_init(), arglist.as_init()).unwrap();

        let expected = r#"{}"#;

        let res = jsonify.process(cfg.as_process(), arglist.as_process(), None);
        assert_eq!(res, Ok(expected));
    }

    #[test]
    fn test_types() {
        let mut cfg = MockUdfCfg::new();
        let mut arglist = mock_args![
                (String None, "empty_string", true),
                (Int None, "empty_int", true),
                (Decimal None, "empty_decimal", true),
                (Real None, "empty_float", true),
                ("some string", "string", true),
                (500, "int", true),
                (-500, "neg_int", true),
                (Decimal "1234.56", "decimal", true),
                (111.001, "float", true),
        ];

        let mut s = String::from("{");
        s.push_str(r#""decimal":1234.56,"#);
        s.push_str(r#""empty_decimal":null,"#);
        s.push_str(r#""empty_float":null,"#);
        s.push_str(r#""empty_int":null,"#);
        s.push_str(r#""empty_string":null,"#);
        s.push_str(r#""float":111.001,"#);
        s.push_str(r#""int":500,"#);
        s.push_str(r#""neg_int":-500,"#);
        s.push_str(r#""string":"some string""#);
        s.push_str(r#"}"#);

        let mut jsonify = Jsonify::init(cfg.as_init(), arglist.as_init()).unwrap();
        let res = jsonify.process(cfg.as_process(), arglist.as_process(), None);

        assert_eq!(res, Ok(s.as_str()));
    }
}
