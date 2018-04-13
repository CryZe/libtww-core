extern crate byteorder;
extern crate goblin;
extern crate rustc_demangle;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate syn;
extern crate toml;

mod assembler;
mod config;
mod dol;
mod iso;
mod linker;

use assembler::Assembler;
use assembler::Instruction;
use config::Config;
use dol::DolFile;
use linker::SectionKind;
use rustc_demangle::demangle;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use std::process::Command;

const FRAMEWORK_MAP: &str = include_str!("../resources/framework.map");
const HEADER: &str = r".text section layout
  Starting        Virtual
  address  Size   address
  -----------------------";

fn create_framework_map(config: &Config, sections: &[linker::LinkedSection]) {
    let mut file =
        BufWriter::new(File::create(&config.build.map).expect("Couldn't create the framework map"));

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

    let mut libs_to_link = Vec::with_capacity(config.src.link.len() + 1);
    for lib_path in &config.src.link {
        let mut file_buf = Vec::new();
        File::open(&config.src.link[0])
            .unwrap_or_else(|_| {
                panic!(
                    "Couldn't find \"{}\". Did you build the project correctly?",
                    lib_path.display()
                )
            })
            .read_to_end(&mut file_buf)
            .unwrap();
        libs_to_link.push(file_buf);
    }
    libs_to_link.push(linker::BASIC_LIB.to_owned());

    let linked = linker::link(
        &libs_to_link,
        base_address.value() as u32,
        config.link.entries.clone(),
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

    let buf = iso::reader::load_iso_buf(&config.src.iso)
        .unwrap_or_else(|_| panic!("Couldn't find \"{}\".", config.src.iso.display()));

    let new_dol_data;
    let mut iso = iso::reader::load_iso(&buf);
    {
        let main_dol = iso.main_dol_mut().unwrap();
        let dol_data: Vec<u8> = main_dol.data.to_owned();

        println!("Patching game...");

        let original = DolFile::parse(&dol_data);
        new_dol_data = patch_game(original, linked.dol, instructions);
        main_dol.data = &new_dol_data;
    }

    println!("Building ISO...");
    let iso_path = &config.build.iso;
    iso::writer::write_iso(BufWriter::new(File::create(iso_path).unwrap()), &iso).unwrap();
}

fn patch_game(original: DolFile, intermediate: DolFile, instructions: &[Instruction]) -> Box<[u8]> {
    let mut original = original;

    original.append(intermediate);
    original.patch(instructions);

    original.to_bytes()
}
