//! UUID namespaces (const)

use udf::prelude::*;

use crate::validate_arg_count;

/// Empty UUID
#[derive(Debug, PartialEq)]
struct UuidNil;

#[register]
impl BasicUdf for UuidNil {
    type Returns<'a> = &'static str;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_nil")?;
        cfg.set_is_const(true);
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok("00000000-0000-0000-0000-000000000000")
    }
}

/// MAX UUID (all ones)
struct UuidMax;

#[register]
impl BasicUdf for UuidMax {
    type Returns<'a> = &'static str;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_max")?;
        cfg.set_is_const(true);
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok("ffffffff-ffff-ffff-ffff-ffffffffffff")
    }
}

/// DNS namespace UUID
#[derive(Debug, PartialEq)]
struct UuidNsDns;

#[register]
impl BasicUdf for UuidNsDns {
    type Returns<'a> = &'static str;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_ns_dns")?;
        cfg.set_is_const(true);
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok("6ba7b810-9dad-11d1-80b4-00c04fd430c8")
    }
}

/// URL namespace UUID
#[derive(Debug, PartialEq)]
struct UuidNsUrl;

#[register]
impl BasicUdf for UuidNsUrl {
    type Returns<'a> = &'static str;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_ns_url")?;
        cfg.set_is_const(true);
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok("6ba7b811-9dad-11d1-80b4-00c04fd430c8")
    }
}

/// OID namespace UUID
#[derive(Debug, PartialEq)]
struct UuidNsOid;

#[register]
impl BasicUdf for UuidNsOid {
    type Returns<'a> = &'static str;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_ns_oid")?;
        cfg.set_is_const(true);
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok("6ba7b812-9dad-11d1-80b4-00c04fd430c8")
    }
}

/// X.500 namespace UUID
#[derive(Debug, PartialEq)]
struct UuidNsX500;

#[register]
impl BasicUdf for UuidNsX500 {
    type Returns<'a> = &'static str;

    fn init(cfg: &UdfCfg<Init>, args: &ArgList<Init>) -> Result<Self, String> {
        validate_arg_count(args.len(), 0, "uuid_ns_x500")?;
        cfg.set_is_const(true);
        Ok(Self)
    }

    fn process<'a>(
        &'a mut self,
        _cfg: &UdfCfg<Process>,
        _args: &ArgList<Process>,
        _error: Option<NonZeroU8>,
    ) -> Result<Self::Returns<'a>, ProcessError> {
        Ok("6ba7b814-9dad-11d1-80b4-00c04fd430c8")
    }
}
