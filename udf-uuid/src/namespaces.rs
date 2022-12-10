//! UUID namespaces (const)

use udf::prelude::*;
use uuid::Uuid;

use crate::validate_arg_count;

/// Empty UUID
#[derive(Debug, PartialEq)]
struct UuidNil;

#[register]
impl BasicUdf for UuidNil {
    type Returns<'a> = String;

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
        Ok(Uuid::nil().as_hyphenated().to_string())
    }
}

/// DNS namespace UUID
#[derive(Debug, PartialEq)]
struct UuidNsDns;

#[register]
impl BasicUdf for UuidNsDns {
    type Returns<'a> = String;

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
        Ok(Uuid::NAMESPACE_DNS.as_hyphenated().to_string())
    }
}

/// URL namespace UUID
#[derive(Debug, PartialEq)]
struct UuidNsUrl;

#[register]
impl BasicUdf for UuidNsUrl {
    type Returns<'a> = String;

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
        Ok(Uuid::NAMESPACE_URL.as_hyphenated().to_string())
    }
}

/// OID namespace UUID
#[derive(Debug, PartialEq)]
struct UuidNsOid;

#[register]
impl BasicUdf for UuidNsOid {
    type Returns<'a> = String;

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
        Ok(Uuid::NAMESPACE_OID.as_hyphenated().to_string())
    }
}

/// X.500 namespace UUID
#[derive(Debug, PartialEq)]
struct UuidNsX500;

#[register]
impl BasicUdf for UuidNsX500 {
    type Returns<'a> = String;

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
        Ok(Uuid::NAMESPACE_X500.as_hyphenated().to_string())
    }
}
