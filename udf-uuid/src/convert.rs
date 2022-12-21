#![allow(unused)]

use std::cmp::min;

use udf::prelude::*;
use uuid::Uuid;

use crate::UUID_BYTES;

/// To save an allocation, we return
#[derive(Debug, PartialEq)]
struct UuidToBin([u8; UUID_BYTES]);

#[register]
impl BasicUdf for UuidToBin {
    type Returns<'a> = &'a [u8];

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        if args.len() < 1 || args.len() > 2 {
            return Err(format!(
                "uuid_to_bin takes 1 or 2 arguments but got {} (usage: `uuid_to_bin(str_uuid)` or `uuid_to_bin(str_uuid, swap)`)",
                args.len()
            ));
        }

        // Coerce first arg to a string, coerce second arg to an int if it exists
        args.get(0).unwrap().set_type_coercion(SqlType::String);
        if let Some(mut arg) = args.get(1) {
            if let Some(val) = arg.value().as_int() {
                // Good chance this value is const, try to validate it if so
                if val < 0 || val > 1 {
                    return Err(format!(
                        "uuid_to_bin's second argument must be 0 or 1; got {}",
                        val
                    ));
                }
            }
            arg.set_type_coercion(SqlType::Int);
        }

        cfg.set_is_const(true);
        cfg.set_max_len(UUID_BYTES as u64);
        Ok(Self([0; UUID_BYTES]))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        // Extract the input UUID string and optional flag, default to false
        let tmp = args.get(0).unwrap().value();
        let input_uuid = binding.as_bytes().unwrap();
        let swap: bool = args
            .get(1)
            .map(|a| a.value().as_int())
            .flatten()
            .unwrap_or(0)
            != 0;

        // Parse the UUID; if we can't print a message (that's the best error
        // handling at this point)
        let res = Uuid::try_parse_ascii(&input_uuid).map_err(|e| {
            let print_slice = &input_uuid[..min(20, input_uuid.len())];
            udf_log!(Warning: "uuid input invalid: {:02x?}", print_slice);
            ProcessError
        })?;

        if swap {
        } else {
            self.0 = res.into_bytes();
        }

        Ok(&self.0)
        // Ok(Uuid::NAMESPACE_DNS.as_hyphenated().to_string())
    }
}
