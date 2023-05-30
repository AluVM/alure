// AluVM Runtime Environment
// To find more on AluVM please check <https://www.aluvm.org>
//
// Designed & written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
// for Pandora Core AG

use aluvm::library::Lib;
use ed25519::Signature;

use crate::dyn_data::DynData;

#[derive(Clone, Eq, PartialEq, Debug, Display)]
#[display("{main}")]
pub struct Executable {
    pub entry_point: u16,
    pub main: Lib,
    pub dyn_data: DynData,
    pub signature: Signature,
}
