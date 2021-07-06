# AluVM Runtime Environment

Provides ALURE ISA estensions to the main ALU instruction set and implements
execution environment for AluRE binaries.

## Components

- AluRE rust library, used by package manager ([Alux]) and assembler ([AluAsm])
- AluREX executable able to run binary Alu programs

## Usage

```
alurex prog-id [--defaults | --dyn-data file] [--prog-dir dir]

-D, --defaults:
    Use default input data from dynamic data segment.
-d, --dyn-data:
    Use input data stored in YAML, JSON or TOML file
-p, --prog-dir:
    Directory where Alu binary programs and libraries are stored.
    Defaults for `~/.alure/lib`

By default, `alurex` looks for ALu executables (files with `.rex` extension)
and checks whether any of those has a given program id. If the program is not 
found, then the `--prog-dir` is checked.
```

[Alux]: https://github.com/pandoracore/alux
[AluAsm]: https://github.com/pandoracore/aluasm
