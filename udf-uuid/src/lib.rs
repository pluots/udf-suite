
use udf::prelude::*;
use uuid::Uuid;
use mac_address::get_mac_address;
use rand::Rng;


struct UuidGenerateV1();

// #[register]
impl BasicUdf for UuidGenerateV1 {
    type Returns<'a> = [u8; 36];

    fn init<'a>(_cfg: &UdfCfg<Init>, args: &'a ArgList<'a, Init>) -> Result<Self, String> {
        if args.len() != 0 {
            Err(format!("uuid_generate_v1 takes 0 arguments; got {}", args.len()))
        } else {
            Ok(Self())
        }
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        // Try to get the mac address; just return zeroes if there are any issues
        let mac = get_mac_address().ok().flatten().map(|m| m.bytes()).unwrap_or([0u8; 6]);
        let mut buf = [0u8; 36];
        Uuid::now_v1(&mac).as_hyphenated().encode_lower(&mut buf);
        Ok(buf)
        // Ok(Uuid::now_v1(&mac).as_hyphenated().to_string())
    }
}

/// V1 UUID with randomized MAC address
struct UuidGenerateV1mc();

impl BasicUdf for UuidGenerateV1mc {
    type Returns<'a> = String;

    fn init<'a>(_cfg: &UdfCfg<Init>, args: &'a ArgList<'a, Init>) -> Result<Self, String> {
        if args.len() != 0 {
            Err(format!("uuid_generate_v1 takes 0 arguments; got {}", args.len()))
        } else {
            Ok(Self())
        }
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        let mac: [u8; 6] = rand::random();
        Ok(Uuid::now_v1(&mac).as_hyphenated().to_string())
    }
}




struct UuidGenerateV4();

impl BasicUdf for UuidGenerateV4 {
    type Returns<'a> = String;

    fn init<'a>(_cfg: &UdfCfg<Init>, args: &'a ArgList<'a, Init>) -> Result<Self, String> {
        if args.len() != 0 {
            Err(format!("uuid_generate_v4 takes 0 arguments; got {}", args.len()))
        } else {
            Ok(Self())
        }
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


// ** Namespace helpers **


/// Empty UUID
struct UuidNil();

impl BasicUdf for UuidNil {
    type Returns<'a> = String;

    fn init<'a>(_cfg: &UdfCfg<Init>, args: &'a ArgList<'a, Init>) -> Result<Self, String> {
        if args.len() != 0 {
            Err(format!("uuid_nil takes 0 arguments; got {}", args.len()))
        } else {
            Ok(Self())
        }
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok(Uuid::nil().as_hyphenated().to_string())
    }
}



struct UuidNsDns();

impl BasicUdf for UuidNsDns {
    type Returns<'a> = String;

    fn init<'a>(_cfg: &UdfCfg<Init>, args: &'a ArgList<'a, Init>) -> Result<Self, String> {
        if args.len() != 0 {
            Err(format!("uuid_ns_dns takes 0 arguments; got {}", args.len()))
        } else {
            Ok(Self())
        }
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok(Uuid::NAMESPACE_DNS.as_hyphenated().to_string())
    }
}


struct UuidNsUrl();

impl BasicUdf for UuidNsUrl {
    type Returns<'a> = String;

    fn init<'a>(_cfg: &UdfCfg<Init>, args: &'a ArgList<'a, Init>) -> Result<Self, String> {
        if args.len() != 0 {
            Err(format!("uuid_ns_url takes 0 arguments; got {}", args.len()))
        } else {
            Ok(Self())
        }
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok(Uuid::NAMESPACE_URL.as_hyphenated().to_string())
    }
}


struct UuidNsOid();

impl BasicUdf for UuidNsOid {
    type Returns<'a> = String;

    fn init<'a>(_cfg: &UdfCfg<Init>, args: &'a ArgList<'a, Init>) -> Result<Self, String> {
        if args.len() != 0 {
            Err(format!("uuid_ns_oid takes 0 arguments; got {}", args.len()))
        } else {
            Ok(Self())
        }
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok(Uuid::NAMESPACE_OID.as_hyphenated().to_string())
    }
}


struct UuidNsX500();

impl BasicUdf for UuidNsX500 {
    type Returns<'a> = String;

    fn init<'a>(_cfg: &UdfCfg<Init>, args: &'a ArgList<'a, Init>) -> Result<Self, String> {
        if args.len() != 0 {
            Err(format!("uuid_ns_x500 takes 0 arguments; got {}", args.len()))
        } else {
            Ok(Self())
        }
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok(Uuid::NAMESPACE_X500.as_hyphenated().to_string())
    }
}
