// AluVM Runtime Environment
// To find more on AluVM please check <https://www.aluvm.org>
//
// Designed & written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
// for Pandora Core AG

use std::collections::{btree_map, BTreeMap};
use std::marker::PhantomData;

use aluasm::product::{DyBin, DyLib};
use aluvm::isa::InstructionSet;
use aluvm::library::constants::LIBS_MAX_TOTAL;
use aluvm::library::{Lib, LibId, LibSite};
use aluvm::{ProgError, Program};

pub struct DyProg<Isa, const RUNTIME_MAX_TOTAL_LIBS: u16 = 1024>
where
    Isa: InstructionSet,
{
    bin: DyBin,
    /// Libraries known to the runtime, identified by their hashes.
    libs: BTreeMap<LibId, DyLib>,

    // #[cfg_attr(feature = "strict_encoding", strict_encoding(skip))]
    // #[cfg_attr(feature = "serde", serde(skip))]
    phantom: PhantomData<Isa>,
}

impl<Isa, const RUNTIME_MAX_TOTAL_LIBS: u16> DyProg<Isa, RUNTIME_MAX_TOTAL_LIBS>
where
    Isa: InstructionSet,
{
    const RUNTIME_MAX_TOTAL_LIBS: u16 = RUNTIME_MAX_TOTAL_LIBS;

    pub fn new(bin: DyBin) -> Self {
        Self { bin, libs: BTreeMap::new(), phantom: default!() }
    }

    #[inline]
    pub fn add_lib(&mut self, lib: DyLib) -> Result<bool, ProgError> {
        if self.lib_count() >= LIBS_MAX_TOTAL.min(Self::RUNTIME_MAX_TOTAL_LIBS)
        {
            return Err(ProgError::TooManyLibs);
        }
        for isa in &lib.as_static_lib().isae {
            if !Isa::is_supported(isa) {
                return Err(ProgError::IsaNotSupported(isa.to_owned()));
            }
        }
        Ok(self.libs.insert(lib.lib_id(), lib).is_none())
    }
}

impl<Isa, const RUNTIME_MAX_TOTAL_LIBS: u16> Program
    for DyProg<Isa, RUNTIME_MAX_TOTAL_LIBS>
where
    Isa: InstructionSet,
{
    type Isa = Isa;
    type Iter<'a> = std::iter::Chain<std::slice::Iter<'a, Lib>, std::iter::Map<btree_map::Values<'a, LibId, DyLib>, fn(&DyLib) -> &Lib>, > where Self: 'a;

    fn lib_count(&self) -> u16 { self.libs.len() as u16 + 1 }

    fn libs(&self) -> Self::Iter<'_> {
        std::slice::from_ref(self.bin.as_static_lib()).iter().chain(
            self.libs.values().map(DyLib::as_static_lib as fn(&DyLib) -> &Lib),
        )
    }

    fn lib(&self, id: LibId) -> Option<&Lib> {
        if id == self.bin.lib_id() {
            return Some(self.bin.as_static_lib());
        }
        self.libs.get(&id).map(DyLib::as_static_lib)
    }

    fn entrypoint(&self) -> LibSite {
        LibSite { lib: self.bin.lib_id(), pos: self.bin.entry_point }
    }
}
