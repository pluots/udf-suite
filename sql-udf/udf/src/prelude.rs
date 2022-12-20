//! Module that can be imported with `use udf::prelude::*;` to quickly get the
//! most often used imports.

pub use std::num::NonZeroU8;

pub use crate::{
    register, udf_log, AggregateUdf, ArgList, BasicUdf, Init, Process, ProcessError, SqlArg,
    SqlResult, SqlType, UdfCfg,
};
