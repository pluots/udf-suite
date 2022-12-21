//! UUID generators
//!
//! We store our results in our structs to avoid some allocations

use mac_address::get_mac_address;
use udf::prelude::*;
use uuid::Uuid;

use crate::{validate_arg_count, HYPHENATED_UUID_LEN, HYPHENATED_UUID_LEN_U64};

#[derive(Debug)]
struct UuidGenerateV1 {
    /// We save the mac address during the `init` call because that won't change.
    /// Saves a few ms, maybe
    mac: [u8; 6],
    res: [u8; HYPHENATED_UUID_LEN],
}

#[register]
impl BasicUdf for UuidGenerateV1 {
    type Returns<'a> = &'a [u8];

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_generate_v1")?;
        cfg.set_max_len(HYPHENATED_UUID_LEN_U64);

        // Try to get the mac address; just return zeroes if there are any issues
        Ok(Self {
            mac: get_mac_address()
                .ok()
                .flatten()
                .map(|m| m.bytes())
                .unwrap_or([0u8; 6]),
            res: [0; HYPHENATED_UUID_LEN],
        })
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Uuid::now_v1(&self.mac)
            .hyphenated()
            .encode_lower(&mut self.res);

        Ok(&self.res)
    }
}

/// V1 UUID with randomized MAC address
#[derive(Debug)]
struct UuidGenerateV1mc([u8; HYPHENATED_UUID_LEN]);

#[register]
impl BasicUdf for UuidGenerateV1mc {
    type Returns<'a> = &'a [u8];

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_generate_v1mc")?;
        cfg.set_max_len(HYPHENATED_UUID_LEN_U64);
        Ok(Self([0; HYPHENATED_UUID_LEN]))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let mut fake_mac: [u8; 6] = rand::random();

        // magic bits for multicast address
        fake_mac[0..=2].copy_from_slice(&[0x01u8, 0x00, 0x5e]);
        Uuid::now_v1(&fake_mac)
            .hyphenated()
            .encode_lower(&mut self.0);
        Ok(&self.0)
    }
}

/// V4 (completely random) UUID
#[derive(Debug)]
struct UuidGenerateV4([u8; HYPHENATED_UUID_LEN]);

#[register]
impl BasicUdf for UuidGenerateV4 {
    type Returns<'a> = &'a [u8];

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_generate_v4")?;
        cfg.set_max_len(HYPHENATED_UUID_LEN_U64);
        Ok(Self([0; HYPHENATED_UUID_LEN]))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Uuid::new_v4().hyphenated().encode_lower(&mut self.0);
        Ok(&self.0)
    }
}

/// V6 UUID, rearranged V1
///
/// Allows specifying the mac address if desired
#[derive(Debug)]
struct UuidGenerateV6([u8; HYPHENATED_UUID_LEN]);

#[register]
impl BasicUdf for UuidGenerateV6 {
    type Returns<'a> = &'a [u8];

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        if args.len() > 1 {
            return Err(format!(
                "uuid_generate_v6 takes 0 or 1 arguments but got {} (usage: `uuid_generate_v6()` or `uuid_generate_v6(node_id)`)",
                args.len()
            ));
        }

        if let Some(mut arg) = args.get(0) {
            arg.set_type_coercion(SqlType::String);
        }

        cfg.set_max_len(HYPHENATED_UUID_LEN_U64);
        Ok(Self([0; HYPHENATED_UUID_LEN]))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let uuid = if let Some(arg) = args.get(0) {
            let tmp = arg.value();
            let bytes = tmp.as_bytes().unwrap();
            let Ok(node_id) = bytes.try_into() else {
                udf_log!(Warning: "uuid_generate_v6 expected argument of length 6; got {}", bytes.len());
                return Err(ProcessError);
            };

            Uuid::now_v1(node_id)
        } else {
            let rand_node: [u8; 6] = rand::random();
            Uuid::now_v1(&rand_node)
        };

        uuid.hyphenated().encode_lower(&mut self.0);
        Ok(&self.0)
    }
}

/// V7 UUID: random UUID that starts with the current UNIX timestamp
#[derive(Debug)]
struct UuidGenerateV7([u8; HYPHENATED_UUID_LEN]);

#[register]
impl BasicUdf for UuidGenerateV7 {
    type Returns<'a> = &'a [u8];

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_generate_v7")?;
        cfg.set_max_len(HYPHENATED_UUID_LEN_U64);
        Ok(Self([0; HYPHENATED_UUID_LEN]))
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Uuid::now_v7().hyphenated().encode_lower(&mut self.0);
        Ok(&self.0)
    }
}
