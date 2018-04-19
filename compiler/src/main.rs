extern crate byteorder;
extern crate goblin;
extern crate rustc_demangle;
#[macro_use]
extern crate serde_derive;
extern crate encoding_rs;
extern crate image;
extern crate regex;
extern crate serde;
extern crate syn;
extern crate toml;

mod assembler;
mod banner;
mod config;
mod dol;
mod framework_map;
mod demangle;
mod iso;
mod linker;

use assembler::Assembler;
use assembler::Instruction;
use banner::Banner;
use config::Config;
use dol::DolFile;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, prelude::*};
use std::process::Command;

<<<<<<< HEAD
||||||| merged common ancestors
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

=======
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

>>>>>>> WIP
fn main() {
    let mut toml_buf = String::new();
    File::open("RomHack.toml")
        .expect("Couldn't find \"RomHack.toml\".")
        .read_to_string(&mut toml_buf)
        .expect("Failed to read \"RomHack.toml\".");

    let mut config: Config = toml::from_str(&toml_buf).expect("Can't parse RomHack.toml");
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

    println!("Loading original game...");

    let buf = iso::reader::load_iso_buf(&config.src.iso)
        .unwrap_or_else(|_| panic!("Couldn't find \"{}\".", config.src.iso.display()));

    let (new_dol_data, new_banner_data);
    let mut iso = iso::reader::load_iso(&buf);

    let mut original_symbols = HashMap::new();
    if let Some(framework_map) = iso.framework_map() {
        println!("Parsing game's framework.map...");
        original_symbols = framework_map::parse(&framework_map.data);
    }

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
        &original_symbols,
    );

    println!("Creating map...");

    framework_map::create(&config, &linked.sections);

    let mut instructions = Vec::new();
    if let Some(patch) = config.src.patch.take() {
        println!("Parsing patch...");

        let mut asm = String::new();
        File::open(&patch)
            .unwrap_or_else(|_| panic!("Couldn't find \"{}\".", patch.display()))
            .read_to_string(&mut asm)
            .expect("Couldn't read the patch file");

        let lines = &asm.lines().collect::<Vec<_>>();

        let mut assembler = Assembler::new(linked.symbol_table);
        instructions = assembler.assemble_all_lines(lines);
    }

    {
        println!("Patching game...");

        let main_dol = iso.main_dol_mut().expect("Dol file not found");
        let dol_data: Vec<u8> = main_dol.data.to_owned();

        let original = DolFile::parse(&dol_data);
        new_dol_data = patch_game(original, linked.dol, &instructions);
        main_dol.data = &new_dol_data;
    }
    {
        println!("Patching banner...");

        if let Some(banner_file) = iso.banner_mut() {
            // TODO Not always true
            let is_japanese = true;
            let mut banner = Banner::parse(is_japanese, &banner_file.data);
            if let Some(game_name) = config.info.game_name.take() {
                banner.game_name = game_name;
            }
            if let Some(developer_name) = config.info.developer_name.take() {
                banner.developer_name = developer_name;
            }
            if let Some(full_game_name) = config.info.full_game_name.take() {
                banner.full_game_name = full_game_name;
            }
            if let Some(full_developer_name) = config.info.full_developer_name.take() {
                banner.full_developer_name = full_developer_name;
            }
            if let Some(game_description) = config.info.description.take() {
                banner.game_description = game_description;
            }
            if let Some(image_path) = config.info.image.take() {
                let image = image::open(image_path)
                    .expect("Couldn't open banner image")
                    .to_rgba();
                banner.image.copy_from_slice(&image);
            }
            new_banner_data = banner.to_bytes(is_japanese);
            banner_file.data = &new_banner_data;
        } else {
            println!("No banner to patch.");
        }
    }

    println!("Building ISO...");
    let iso_path = &config.build.iso;
    iso::writer::write_iso(
        BufWriter::with_capacity(4 << 20, File::create(iso_path).unwrap()),
        &iso,
    ).unwrap();
}

fn patch_game(
    mut original: DolFile,
    intermediate: DolFile,
    instructions: &[Instruction],
) -> Box<[u8]> {
    original.append(intermediate);
    original.patch(instructions);

    original.to_bytes()
}
