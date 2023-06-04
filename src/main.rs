// AluVM Runtime Environment
// To find more on AluVM please check <https://www.aluvm.org>
//
// Designed & written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
// for Pandora Core AG

extern crate alloc;
#[macro_use]
extern crate amplify;

pub mod dyn_data;
pub mod files;
pub mod isa;

use std::fs;
use std::path::PathBuf;

use aluasm::module::Module;
use aluasm::product::{DyBin, DyLib, Product};
use alure::program::DyProg;
use aluvm::data::encoding::Decode;
use aluvm::isa::Instr;
use clap::Parser;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Parser)]
#[clap(
    name = "alurex",
    bin_name = "alurex",
    author,
    version,
    about = "Executes AluVM binary programs"
)]
pub struct Args {
    /// Use default input data from dynamic data segment.
    #[clap(short = 'D', long)]
    pub defaults: bool,

    #[clap(short, long, default_value = "~/.alure")]
    pub prog_dir: PathBuf,

    /// Use input data stored in YAML, JSON or TOML file
    #[clap(short, long, conflicts_with = "defaults")]
    pub dyn_data: Option<PathBuf>,

    /// Either AluRE prog id (Bech32 string starting with `alurex1...` or
    /// a file name containing binary AluRE program (usually has extension
    /// `*.rex`).
    ///
    /// If a prog id is given, by default, `alurex` looks for ALu executables
    /// (files with `.rex` extensions and checks whether any of those has the
    /// given program id. If the program is not found, then the `prog-dir`
    /// is checked (see `--prog-dir` option).
    pub exec: String,
}

fn main() {
    let args = Args::parse();

    eprintln!("Executing {}", args.exec);
    let exec = fs::File::open(args.exec).expect("unable to open the file");
    let bin = DyBin::decode(exec).expect("unable to decode the executable");
    let mut prog = DyProg::<Instr>::new(bin);
    for entry in fs::read_dir(args.prog_dir)
        .expect("unable to read the program directory")
    {
        let entry = entry.expect("unable to read the program directory entry");
        if entry.path().extension() != Some("ald".as_ref()) {
            continue;
        }
        let lib = DyLib::decode(
            fs::File::open(entry.path()).expect("unable to open the library"),
        )
        .expect("unable to decode the library");
        prog.add_lib(lib);
    }
    let mut vm = aluvm::Vm::<Instr>::new();
    vm.run(&prog, &());
}
