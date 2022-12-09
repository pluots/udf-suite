use mac_address::get_mac_address;
use udf::prelude::*;
use uuid::Uuid;

use crate::validate_arg_count;

#[derive(Debug)]
struct UuidGenerateV1 {
    /// We save the mac address during the `init` call because that won't change.
    /// Saves a few ms, maybe
    mac: [u8; 6],
}

#[register]
impl BasicUdf for UuidGenerateV1 {
    type Returns<'a> = String;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_generate_v1")?;

        Ok(Self {
            mac: get_mac_address()
                .ok()
                .flatten()
                .map(|m| m.bytes())
                .unwrap_or([0u8; 6]),
        })
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        // Try to get the mac address; just return zeroes if there are any issues
        Ok(Uuid::now_v1(&self.mac).as_hyphenated().to_string())
    }
}

// /// V1 UUID with specified MAC address
// struct UuidGenerateV1arg;

// #[register]
// impl BasicUdf for UuidGenerateV1arg {
//     type Returns<'a> = String;

//     fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
//         if args.len() != 1 {
//             Err(format!(
//                 "uuid_generate_v1arg takes 1 argument but got {}",
//                 args.len()
//             ))
//         } else {
//             args.get(0).unwrap().set_type_coercion(SqlType::String);
//             Ok(Self)
//         }
//     }

//     fn process<'a>(
//         &'a mut self,
//         _cfg: &UdfCfg<Process>,
//         _args: &ArgList<Process>,
//         _error: Option<NonZeroU8>,
//     ) -> Result<Self::Returns<'a>, ProcessError> {

//         Ok(Uuid::now_v1(&fake_mac).as_hyphenated().to_string())
//     }
// }

/// V1 UUID with randomized MAC address
#[derive(Debug)]
struct UuidGenerateV1mc;

#[register]
impl BasicUdf for UuidGenerateV1mc {
    type Returns<'a> = String;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_generate_v1mc")?;
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let mut fake_mac: [u8; 6] = rand::random();

        // magic bits for multicast address
        fake_mac[0..=3].copy_from_slice(&[0x01u8, 0x00, 0x5e]);

        Ok(Uuid::now_v1(&fake_mac).as_hyphenated().to_string())
    }
}

/// V4 (completely random) UUID
#[derive(Debug)]
struct UuidGenerateV4;

#[register]
impl BasicUdf for UuidGenerateV4 {
    type Returns<'a> = String;

    fn init(_cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_generate_v4")?;
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok(Uuid::new_v4().as_hyphenated().to_string())
    }
}
