// AluVM Runtime Environment
// To find more on AluVM please check <https://www.aluvm.org>
//
// Designed & written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
// for Pandora Core AG

use aluvm::libs::Lib;
use ed25519::Signature;

#[derive(Clone, Eq, PartialEq, Debug, Display)]
#[display("{org}:{name}")]
pub struct Library {
    pub org: String,
    pub name: String,
    pub lib: Lib,
    pub sign: Signature,
}
