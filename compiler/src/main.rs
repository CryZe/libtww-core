extern crate byteorder;
extern crate goblin;
extern crate rustc_demangle;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate syn;
extern crate toml;
#[macro_use]
extern crate structopt;

mod assembler;
mod dol;
// mod elf;
mod config;
mod static_lib;

use assembler::Assembler;
use assembler::Instruction;
use config::Config;
use dol::DolFile;
use rustc_demangle::demangle;
use static_lib::SectionKind;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::process::Command;
use structopt::StructOpt;

const FRAMEWORK_MAP: &'static str = include_str!("../resources/framework.map");
const HEADER: &'static str = r".text section layout
  Starting        Virtual
  address  Size   address
  -----------------------";

fn create_framework_map(config: &Config, sections: &[static_lib::LinkedSection]) {
    let mut file =
        BufWriter::new(File::create(&config.out.map).expect("Couldn't create the framework map"));

    writeln!(file, "{}", HEADER).unwrap();

    for section in sections {
        let mut section_name_buf;
        let section_name = section.section_name;
        let section_name = if section_name.starts_with(".text.")
            && section.kind == SectionKind::TextSection
        {
            section_name_buf = demangle(&section_name[".text.".len()..]).to_string();
            section_name_buf = section_name_buf
                .replace(' ', "_")
                .replace("()", "Void")
                .replace("(", "Tuple<")
                .replace(")", ">");
            let mut section_name: &str = &section_name_buf;
            if section_name.len() >= 19 && &section_name[section_name.len() - 19..][..3] == "::h" {
                section_name = &section_name[..section_name.len() - 19];
            }
            section_name
        } else {
            section_name
        };

        writeln!(
            file,
            "  00000000 {:06x} {:08x}  4 {} \t{}",
            section.len - section.sym_offset,
            section.address + section.sym_offset,
            section_name,
            section.member_name
        ).unwrap();
    }

    write!(file, "{}", FRAMEWORK_MAP).unwrap();
}

fn main() {
    let opt = config::Opt::from_args();

    let mut toml_buf = String::new();
    File::open("RomHack.toml")
        .expect(
            "Couldn't find \"RomHack.toml\". Did you build the project correctly \
             using \"make\"?",
        )
        .read_to_string(&mut toml_buf)
        .expect("Failed to read \"RomHack.toml\".");

    let config: Config = toml::from_str(&toml_buf).expect("Can't parse RomHack.toml");
    let base_address: syn::LitInt =
        syn::parse_str(&config.link.base).expect("Invalid Base Address");

    println!("Compiling...");

    let exit_code = Command::new("cargo")
        .args(&[
            "build",
            "--release",
            "--target",
            "powerpc-unknown-linux-gnu",
        ])
        .env(
            "RUSTFLAGS",
            "-C target-feature=+msync,+fres,+frsqrte -C opt-level=s",
        )
        .current_dir(&config.src.src)
        .spawn()
        .expect("Couldn't build the project")
        .wait()
        .unwrap();

    assert!(exit_code.success(), "Couldn't build the project");

    println!("Linking...");

    let mut intermediate = Vec::new();
    let _ = File::open(&config.src.link[0])
        .unwrap_or_else(|_| {
            panic!(
                "Couldn't find \"{}\". Did you build the project correctly?",
                config.src.link[0].display()
            )
        })
        .read_to_end(&mut intermediate);

    let linked = static_lib::link(
        &intermediate,
        base_address.value() as u32,
        config.link.entries.iter().map(|x| x as &str).collect(),
    );

    println!("Creating map...");

    create_framework_map(&config, &linked.sections);

    println!("Parsing patch...");

    let mut asm = String::new();
    File::open(&config.src.patch)
        .unwrap_or_else(|_| {
            panic!(
                "Couldn't find \"{}\". If you don't need to patch the dol, just create an empty file.",
                config.src.patch.display()
            )
        })
        .read_to_string(&mut asm)
        .expect("Couldn't read the patch file");

    let lines = &asm.lines().collect::<Vec<_>>();

    let mut assembler = Assembler::new(linked.symbol_table);
    let instructions = &assembler.assemble_all_lines(lines);

    println!("Loading original game...");

    // if let Some("cheat") = args().skip(1).next().as_ref().map(|x| x as &str) {
    //     write_cheat(linked.dol, instructions);
    // } else {
    let mut original = Vec::new();
    let _ = File::open(&config.src.dol)
        .unwrap_or_else(|_| panic!("Couldn't find \"{}\".", config.src.dol.display()))
        .read_to_end(&mut original);

    println!("Patching game...");

    let original = DolFile::parse(&original);
    patch_game(original, linked.dol, &config, instructions);

    if opt.iso {
        println!("Building ISO...");

        let iso_path = config
            .out
            .iso
            .expect("Expected iso path in `RomHack.toml` at key `out.iso`");
        let pack_path = config.out.dol.parent().unwrap().parent().unwrap();

        let exit_code = Command::new("gcit")
            .arg(pack_path)
            .arg("-q")
            .arg("-flush")
            .arg("-d")
            .arg(&iso_path)
            .spawn()
            .expect("Couldn't build the ISO")
            .wait()
            .unwrap();

        assert!(exit_code.success(), "Couldn't build the ISO");
    }
    // }
}

fn patch_game(
    original: DolFile,
    intermediate: DolFile,
    config: &Config,
    instructions: &[Instruction],
) {
    let mut original = original;

    original.append(intermediate);
    original.patch(instructions);

    let data = original.to_bytes();
    let mut file = File::create(&config.out.dol).unwrap_or_else(|_| {
        panic!(
            "Couldn't create \"{}\". You might need to provide higher \
             privileges.",
            config.out.dol.display()
        )
    });

    file.write(&data).expect("Couldn't write the main.dol");
}

// fn write_cheat(intermediate: DolFile, instructions: &[Instruction]) {
//     let mut file = File::create("../../cheat.txt").expect(
//         "Couldn't create \"cheat.txt\". You might need to provide higher \
//          privileges.",
//     );

//     writeln!(file, "A8000000 00000001").unwrap();

//     for instruction in instructions {
//         writeln!(
//             file,
//             "{:08X} {:08X}",
//             (instruction.address & 0x01FFFFFF) | 0x04000000,
//             instruction.data
//         ).unwrap();
//     }

//     for section in intermediate
//         .text_sections
//         .iter()
//         .chain(intermediate.data_sections.iter())
//     {
//         writeln!(
//             file,
//             "{:08X} {:08X}",
//             (section.address & 0x01FFFFFF) | 0x06000000,
//             section.data.len()
//         ).unwrap();
//         let line_ender = if section.data.len() % 8 > 0 {
//             8 - (section.data.len() % 8)
//         } else {
//             0
//         };
//         for (i, byte) in section
//             .data
//             .iter()
//             .chain(std::iter::repeat(&0).take(line_ender))
//             .enumerate()
//         {
//             if i % 8 == 4 {
//                 write!(file, " ").unwrap();
//             }

//             write!(file, "{:02X}", byte).unwrap();

//             if i % 8 == 7 {
//                 writeln!(file, "").unwrap();
//             }
//         }
//     }

//     // for section in intermediate.text_sections.iter().chain(intermediate.data_sections.iter()) {
//     //     let mut address = section.address;

//     //     let line_ender = if section.data.len() % 4 > 0 {
//     //         4 - (section.data.len() % 4)
//     //     } else {
//     //         0
//     //     };

//     //     for (i, byte) in section.data.iter().chain(std::iter::repeat(&0).take(line_ender)).enumerate() {
//     //         if i % 4 == 0 {
//     //             write!(file, "{:08X} ", (address & 0x01FFFFFF) | 0x04000000).unwrap();
//     //         }

//     //         write!(file, "{:02X}", byte).unwrap();

//     //         if i % 4 == 3 {
//     //             writeln!(file, "").unwrap();
//     //         }

//     //         address += 1;
//     //     }
//     // }
// }
