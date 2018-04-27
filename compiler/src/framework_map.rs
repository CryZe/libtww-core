use config::Config;
use linker::{LinkedSection, SectionKind};
use regex::Regex;
use rustc_demangle::demangle as demangle_rust;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, prelude::*};
use std::str;
use demangle::demangle as demangle_tww;

const FRAMEWORK_MAP: &str = include_str!("../resources/framework.map");
const HEADER: &str = r".text section layout
  Starting        Virtual
  address  Size   address
  -----------------------";

pub fn create(config: &Config, sections: &[LinkedSection]) {
    let mut file =
        BufWriter::new(File::create(&config.build.map).expect("Couldn't create the framework map"));

    writeln!(file, "{}", HEADER).unwrap();

    for section in sections {
        let mut section_name_buf;
        let section_name = section.section_name;
        let section_name = if section_name.starts_with(".text.")
            && section.kind == SectionKind::TextSection
        {
            section_name_buf = demangle_rust(&section_name[".text.".len()..]).to_string();
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

pub fn parse(buf: &[u8]) -> HashMap<String, u32> {
    let mut symbols = HashMap::new();
    let regex = Regex::new(r"\s{2}\w{8}\s\w{6}\s(\w{8}).{4}(.*)\s{2}").unwrap();
    let text = str::from_utf8(buf).unwrap();
    for line in text.lines().take_while(|l| !l.is_empty()) {
        if let Some(captures) = regex.captures(line) {
            let name = captures.get(2).unwrap().as_str();
            if name != ".text" {
                let address = u32::from_str_radix(captures.get(1).unwrap().as_str(), 16).unwrap();
                symbols.insert(demangle_tww(name).map(|n| n.into_owned()).unwrap_or_else(|_| name.to_owned()), address);
            }
        }
    }
    symbols
}
