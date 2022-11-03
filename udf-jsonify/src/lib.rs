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
        cfg.set_max_len(MaxLenOptions::Blob as u32);
        Ok(Self::default())
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let json_map: Map<String, Value> = args
            .iter()
            .map(|a| (a.attribute.to_owned(), res_to_json_val(a.value)))
            .collect();
        self.0 = Value::Object(json_map).to_string();
        Ok(&self.0)
    }
}

/// Convert a `SqlResult` to a `serde_json::Value`
///
/// Return the apropriate type if possible, `Null` otherwise
fn res_to_json_val(source: SqlResult) -> Value {
    match source {
        SqlResult::String(Some(v)) => Value::String(String::from_utf8_lossy(v).into_owned()),
        SqlResult::Real(Some(v)) => Number::from_f64(v)
            .map(|f| Value::Number(f))
            .unwrap_or(Value::Null),
        SqlResult::Int(Some(v)) => Value::Number(v.into()),
        SqlResult::Decimal(Some(v)) => {
            eprintln!("Got decimal {v:?}");
            Value::String("decimal".to_owned())
            },
        _ => Value::Null,
    }
}
