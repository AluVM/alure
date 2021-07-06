// AluVM Runtime Environment
// To find more on AluVM please check <https://www.aluvm.org>
//
// Designed & written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
// for Pandora Core AG

use super::alu::Library;
use super::rex::Executable;

#[derive(Clone, Eq, PartialEq, Debug, Display)]
#[display("{exec}.rpk")]
pub struct Package {
    pub exec: Executable,
    pub libs: Vec<Library>,
}
