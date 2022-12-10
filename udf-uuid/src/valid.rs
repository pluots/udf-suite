use udf::prelude::*;
use uuid::Uuid;

use crate::validate_arg_count;

/// Check if a given UUID is valid
#[derive(Debug, PartialEq)]
struct UuidIsValid;

#[register]
impl BasicUdf for UuidIsValid {
    type Returns<'a> = i64;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 1, "uuid_is_valid")?;
        args.get(0).unwrap().set_type_coercion(SqlType::String);
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let input = args.get(0).unwrap().value();
        let Some(in_str) = input.as_string() else {
            return Ok(0)
        };

        let in_rep = in_str.replace('-', ""); // Remove hyphens
        let res = match Uuid::try_parse(&in_rep) {
            Ok(_) => 1,
            Err(_) => 0,
        };

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use udf::mock::*;

    use super::*;

    #[test]
    fn test_validate_wrong_args() {
        // Test with 0 and >1 args
        let mut arglists = [
            (mock_args![], 0),
            (mock_args![("", "", false), ("", "", false)], 2),
        ];

        let mut cfg = MockUdfCfg::new();

        for (args, count) in arglists.iter_mut() {
            let res = UuidIsValid::init(cfg.as_init(), args.as_init());
            assert_eq!(
                res,
                Err(format!("uuid_is_valid takes 1 argument but got {count}"))
            );
        }
    }

    #[test]
    fn test_validate() {
        // Test with 0 and >1 args
        let mut arglists = [
            (
                mock_args![("00908d94-c78d-4ea5-8aa5-5a06868f0420", "", false)],
                1,
            ),
            (
                mock_args![("00908d94c78d4ea58aa55a06868f0420", "", false)],
                1,
            ),
            (
                mock_args![("00908d94c78d-4ea5-8aa55a06868f0420", "", false)],
                1,
            ),
            (
                mock_args![("00908d94-c78d-4ea5-8aa5-5a06868f042", "", false)],
                0,
            ),
            (
                mock_args![("00908d94c78d4ea58aa55a06868f042", "", false)],
                0,
            ),
        ];

        let mut cfg = MockUdfCfg::new();
        let mut initialized = UuidIsValid::init(cfg.as_init(), arglists[0].0.as_init()).unwrap();

        for (args, val) in arglists.iter_mut() {
            let res = initialized
                .process(cfg.as_process(), args.as_process(), None)
                .unwrap();
            assert_eq!(res, *val);
        }
    }
}
