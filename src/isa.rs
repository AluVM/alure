// AluVM Runtime Environment
// To find more on AluVM please check <https://www.aluvm.org>
//
// Designed & written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
// for Pandora Core AG

//! AluVM ALURE ISA extension handling main I/O operations

use aluvm::reg::{Reg16, RegA, RegF, RegR, RegS};

pub enum AluReOp {
    Read(u16),
    Check(u16),
    // Dec(RegS, u16),
    // Enc(RegS, u16),
    StoreA(RegA, Reg16, u8),
    StoreF(RegF, Reg16, u8),
    StoreR(RegR, Reg16, u8),
    StoreS(RegS, Reg16, u8),

    LoadA(RegA, Reg16, u8),
    LoadF(RegF, Reg16, u8),
    LoadR(RegR, Reg16, u8),
    LoadS(RegS, Reg16, u8),
}
