// AluVM Runtime Environment
// To find more on AluVM please check <https://www.aluvm.org>
//
// Designed & written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
// for Pandora Core AG

//! Dynamic data segment

use alloc::collections::{BTreeMap, BTreeSet};
use core::ops::{RangeBounds, RangeFull, RangeInclusive};

use aluvm::data::{ByteStr, MaybeNumber, Number};
use aluvm::reg::{Reg32, RegA, RegAFR, RegS};

pub struct DynData {
    pub input: Vec<Input>,
    pub output: Vec<Output>,
}

pub struct NumBounds<R = RangeFull>
where
    R: RangeBounds<Number>,
{
    pub default: Option<Number>,
    pub allow_none: bool,
    pub range: R,
}

pub struct StrBounds {
    pub default: Option<ByteStr>,
    pub allow_none: bool,
    pub length: RangeInclusive<u16>,
}

pub enum BoundedValue<R = RangeFull>
where
    R: RangeBounds<Number>,
{
    Number { reg: RegAFR, index: Reg32, bounds: NumBounds<R> },
    DecExp { reg: RegA, index: Reg32, exp: u8, bounds: NumBounds<R> },
    String { index: RegS, bounds: StrBounds },
}

pub struct Input<R = RangeFull>
where
    R: RangeBounds<Number>,
{
    pub info: String,
    pub bounds: BoundedValue<R>,
}

pub struct Output {
    pub info: String,
    pub states: BTreeSet<RegOutput>,
}

pub enum RegOutput {
    Number {
        reg: RegAFR,
        index: Reg32,
        states: BTreeMap<MaybeNumber, String>,
    },
    DecExp {
        reg: RegA,
        index: Reg32,
        exp: u8,
        states: BTreeMap<MaybeNumber, String>,
    },
    String {
        index: RegS,
        states: BTreeMap<Option<ByteStr>, String>,
    },
}
