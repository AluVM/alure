// AluVM Runtime Environment
// To find more on AluVM please check <https://www.aluvm.org>
//
// Designed & written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
// for Pandora Core AG

//! AluVM ALURE ISA extension handling main I/O operations

use aluvm::reg::{Reg16, RegA, RegF, RegR, RegS};

pub enum AluReOp {
    ReadA(RegA, Reg16, u8),
    ReadF(RegF, Reg16, u8),
    ReadR(RegR, Reg16, u8),
    ReadS(RegS, u8),
}
