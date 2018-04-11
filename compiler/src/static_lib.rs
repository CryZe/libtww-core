use byteorder::{ByteOrder, BE};
use dol::{DolFile, Section};
use goblin::archive::Archive;
use goblin::elf::{section_header, sym, Elf, Reloc};
use std::collections::{BTreeMap, HashMap, HashSet};

fn symbols_referenced_in_section<F>(section_index: usize, elf: &Elf, mut f: F)
where
    F: FnMut(usize),
{
    if let Some(reloc_table) = reloc_table_for_section(section_index, elf) {
        for relocation in reloc_table {
            let symbol_index = relocation.r_sym as usize;
            f(symbol_index);
        }
    }
}

fn reloc_table_for_section<'a>(section_index: usize, elf: &'a Elf) -> Option<&'a [Reloc]> {
    elf.shdr_relocs
        .iter()
        .find(|&&(reloc_index, _)| {
            section_index == elf.section_headers[reloc_index as usize].sh_info as usize
        })
        .map(|&(_, ref r)| &**r)
}

fn function_symbols_for_section<'a>(
    section_index: usize,
    elf: &'a Elf,
) -> Box<Iterator<Item = sym::Sym> + 'a> {
    Box::new(
        elf.syms
            .iter()
            .filter(move |sym| section_index == sym.st_shndx as usize && sym.is_function()),
    )
}

#[derive(Copy, Clone, PartialOrd, Ord, Hash, PartialEq, Eq, Debug)]
pub enum SectionKind {
    TextSection,
    DataSection,
    BlockStartedBySymbol,
}

#[derive(PartialOrd, Ord, Hash, PartialEq, Eq, Debug)]
struct SectionInfo<'a> {
    kind: SectionKind,
    member_name: &'a str,
    section_index: usize,
}

#[derive(PartialOrd, Ord, Hash, PartialEq, Eq)]
struct LookupKey<'a> {
    member_name: &'a str,
    section_index: usize,
}

struct LocatedSection<'a> {
    address: u32,
    padding: u32,
    len: u32,
    section_info: SectionInfo<'a>,
}

pub struct Linked<'a> {
    pub dol: DolFile,
    pub symbol_table: BTreeMap<&'a str, u32>,
    pub sections: Vec<LinkedSection<'a>>,
}

pub struct LinkedSection<'a> {
    pub address: u32,
    pub len: u32,
    pub member_name: &'a str,
    pub section_name: &'a str,
    pub sym_offset: u32,
    pub kind: SectionKind,
}

pub fn link<'a>(
    buf: &'a [u8],
    base_address: u32,
    mut archive_symbols_to_visit: Vec<&'a str>,
) -> Linked<'a> {
    let archive = Archive::parse(buf).unwrap();

    // println!("{:#?}", archive);

    // TODO Handle "weak" and "merge" symbols
    // TODO Handle bss

    let mut symbols_to_visit = Vec::new();
    let mut visited_sections = HashSet::new();
    let mut parsed_elfs = BTreeMap::new();

    while let Some(archive_symbol_name) = archive_symbols_to_visit.pop() {
        let member_name = archive
            .member_of_symbol(archive_symbol_name)
            .unwrap_or_else(|| panic!("Unresolved symbol {}", archive_symbol_name));

        // println!();
        // println!(
        //     "Visiting file {} to find symbol {}",
        //     member_name, archive_symbol_name
        // );

        let member = archive.get(member_name).unwrap();
        let elf_buf = &buf[member.offset as usize..][..member.header.size as usize];
        let elf = parsed_elfs
            .entry(member_name)
            .or_insert_with(|| Elf::parse(elf_buf).unwrap());

        for (symbol_index, symbol) in elf.syms.iter().enumerate() {
            let name_index = symbol.st_name;
            let name = elf.strtab.get(name_index).unwrap().unwrap();
            if name == archive_symbol_name {
                symbols_to_visit.push(symbol_index);
            }
        }

        while let Some(symbol_index) = symbols_to_visit.pop() {
            let symbol = elf.syms.get(symbol_index).unwrap();
            let section_index = symbol.st_shndx as usize;
            let name_index = symbol.st_name;
            let name = elf.strtab.get(name_index).unwrap().unwrap();

            let section = &elf.section_headers[section_index];
            // let section_name = elf.shdr_strtab
            //     .get(section.sh_name as usize)
            //     .unwrap()
            //     .unwrap();

            if symbol.is_import() && section.sh_type == section_header::SHT_NULL {
                archive_symbols_to_visit.push(name);
            // println!(
            //     "    |-> Visiting imported symbol {} (#{})",
            //     name, symbol_index
            // );
            } else {
                // println!("    |");
                // println!(
                //     "    |-> Visiting symbol {} (#{}) in section {} (#{})",
                //     name, symbol_index, section_name, section_index
                // );
                // let bind = symbol.st_bind();
                // let is_global = bind == sym::STB_GLOBAL || bind == sym::STB_WEAK;
                if visited_sections.insert(SectionInfo {
                    member_name,
                    section_index,
                    kind: if section.is_executable() {
                        SectionKind::TextSection
                    } else if section.sh_type == section_header::SHT_NOBITS {
                        SectionKind::BlockStartedBySymbol
                    } else {
                        SectionKind::DataSection
                    },
                    // symbol: if is_global {
                    //     Some((name, symbol.st_value as u32))
                    // } else {
                    //     None
                    // },
                }) {
                    symbols_referenced_in_section(section_index, &elf, |symbol_index| {
                        // let symbol = elf.syms.get(symbol_index).unwrap();
                        // let section_index = symbol.st_shndx as usize;

                        // let section = &elf.section_headers[section_index];
                        // let name_index = symbol.st_name;
                        // let name = elf.strtab.get(name_index).unwrap().unwrap();
                        // let section_name = elf.shdr_strtab
                        //     .get(section.sh_name as usize)
                        //     .unwrap()
                        //     .unwrap();
                        // println!(
                        //     "        |-> References symbol {} (#{}) in section {} (#{})",
                        //     name, symbol_index, section_name, section_index
                        // );
                        symbols_to_visit.push(symbol_index);
                    });
                }
            }
        }
    }

    // println!();
    // println!("Layouting...");

    let mut data_section_address = None;
    let mut address = base_address;
    let mut symbol_table = BTreeMap::new();

    let mut visited_sections = visited_sections.into_iter().collect::<Vec<_>>();
    visited_sections.sort_unstable();

    // println!("{:#?}", visited_sections);

    let mut lookup_located_sections = HashMap::with_capacity(visited_sections.len());

    let located_sections = visited_sections
        .into_iter()
        .enumerate()
        .map(|(index, section_info)| {
            let elf = &parsed_elfs[&section_info.member_name];
            let section = &elf.section_headers[section_info.section_index];
            let align = section.sh_addralign as u32;
            let rem = address % align;
            let padding = if rem != 0 {
                let padding = align - rem;
                address += padding;
                padding
            } else {
                0
            };

            if data_section_address.is_none() && section_info.kind != SectionKind::TextSection {
                data_section_address = Some(address);
            }

            for symbol in function_symbols_for_section(section_info.section_index, elf) {
                let bind = symbol.st_bind();
                let is_global = bind == sym::STB_GLOBAL || bind == sym::STB_WEAK;
                if is_global {
                    let name_index = symbol.st_name;
                    let name = elf.strtab.get(name_index).unwrap().unwrap();
                    symbol_table.insert(name, address + symbol.st_value as u32);
                }
            }

            lookup_located_sections.insert(
                LookupKey {
                    member_name: section_info.member_name,
                    section_index: section_info.section_index,
                },
                index,
            );
            let val = LocatedSection {
                padding,
                address,
                len: section.sh_size as u32,
                section_info,
            };

            address += section.sh_size as u32;

            val
        })
        .collect::<Vec<_>>();

    // println!("Relocating...");
    // println!();

    let (mut text_section, mut data_section) = (Vec::<u8>::new(), Vec::<u8>::new());

    for &LocatedSection {
        section_info:
            SectionInfo {
                member_name,
                section_index,
                kind: section_kind,
                ..
            },
        address: located_section_address,
        padding: located_section_padding,
        ..
    } in &located_sections
    {
        let member = archive.get(member_name).unwrap();
        let elf_buf = &buf[member.offset as usize..][..member.header.size as usize];

        let elf = &parsed_elfs[&member_name];
        let section = &elf.section_headers[section_index];
        let mut section_buf;
        let mut section_slice = if section_kind != SectionKind::BlockStartedBySymbol {
            &elf_buf[section.sh_offset as usize..][..section.sh_size as usize]
        } else {
            &[]
        };

        if let Some(reloc_table) = reloc_table_for_section(section_index, &elf) {
            // let section = &elf.section_headers[section_index];
            // let section_name = elf.shdr_strtab
            //     .get(section.sh_name as usize)
            //     .unwrap()
            //     .unwrap();
            // println!("Relocating section {} (#{})", section_name, section_index);
            section_buf = section_slice.to_owned();

            for reloc in reloc_table {
                // println!("    |-> {:?}", reloc);
                let instruction = &mut section_buf[reloc.r_offset as usize..][..4];
                let symbol_index = reloc.r_sym as usize;
                let symbol = elf.syms.get(symbol_index).unwrap();
                let symbol_section_index = symbol.st_shndx as usize;
                // println!("Looking for {}, {}", member_name, symbol_section_index);
                let section_address = lookup_located_sections
                    .get(&LookupKey {
                        member_name,
                        section_index: symbol_section_index,
                    })
                    .map(|&index| located_sections[index].address)
                    .unwrap_or_else(|| {
                        let name_index = symbol.st_name;
                        let archive_symbol_name = elf.strtab.get(name_index).unwrap().unwrap();
                        let member_name = archive.member_of_symbol(archive_symbol_name).unwrap();
                        let elf = &parsed_elfs[&member_name];

                        for (symbol_index, symbol) in elf.syms.iter().enumerate() {
                            let name_index = symbol.st_name;
                            let name = elf.strtab.get(name_index).unwrap().unwrap();
                            if name == archive_symbol_name {
                                let symbol = elf.syms.get(symbol_index).unwrap();
                                let section_index = symbol.st_shndx as usize;
                                let key = &LookupKey {
                                    member_name,
                                    section_index,
                                };
                                let located_section_index = lookup_located_sections[key];
                                return located_sections[located_section_index].address;
                            }
                        }
                        unreachable!()
                    });

                // Hopefully based on:
                // https://github.com/llvm-mirror/lld/blob/0e7ca58c010ce93e66ce716923b0570c91248b7e/ELF/InputSection.cpp#L641

                // S -> Sym.getVA(0)
                let symbol_address = section_address.wrapping_add(symbol.st_value as u32);
                // A -> Addend
                let a = reloc.r_addend as u32;
                // P -> getVA(Rel.Offset)
                // getVa(Offset) => (Out ? Out->Addr : 0) + getOffset(Offset)
                let p = (located_section_address as u32).wrapping_add(reloc.r_offset as u32);

                // The enum can be found here:
                // https://github.com/vocho/openqnx/blob/master/trunk/lib/elf/public/sys/elf_ppc.h#L50
                const R_PPC_ADDR32: u32 = 1;
                const R_PPC_REL24: u32 = 10;
                const R_PPC_PLTREL24: u32 = 18;
                const R_PPC_REL32: u32 = 26;

                let value = match reloc.r_type {
                    R_PPC_ADDR32 => {
                        // R_ABS -> S + A -> Sym.getVA(A)
                        symbol_address.wrapping_add(a)
                    }
                    R_PPC_REL24 | R_PPC_REL32 => {
                        // R_PC -> S + A - P -> Sym.getVA(A) - P
                        symbol_address.wrapping_add(a).wrapping_sub(p)
                    }
                    R_PPC_PLTREL24 => {
                        // R_PLT_PC -> L + A - P -> Sym.getPltVA() + A - P
                        // TODO getPltVA()

                        // For now just lower this as S + A -P
                        // That should honestly be fine
                        symbol_address.wrapping_add(a).wrapping_sub(p)
                    }
                    t => panic!("Unknown reloc type {}", t),
                };

                assert_ne!(
                    reloc.r_offset as i32, -1,
                    "Should be end of section. Can't handle this yet"
                );

                // Based on LLD:
                // https://github.com/llvm-mirror/lld/blob/6d2b0b2fa1005a104120a93bad32f487377e989b/ELF/Arch/PPC.cpp#L49
                match reloc.r_type {
                    R_PPC_ADDR32 | R_PPC_REL32 => BE::write_u32(instruction, value),
                    R_PPC_PLTREL24 | R_PPC_REL24 => {
                        let val = BE::read_u32(instruction) | (value & 0x3FFFFFC);
                        BE::write_u32(instruction, val);
                    }
                    t => panic!("Unknown reloc type {}", t),
                }
            }

            section_slice = &section_buf;
        }

        if section_kind == SectionKind::TextSection {
            for _ in 0..located_section_padding {
                text_section.push(0);
            }
            // println!(
            //     "At text section offset: {:x}, should be at address {:x}, padding: {:x}",
            //     text_section.len(),
            //     located_section_address,
            //     located_section_padding,
            // );
            text_section.extend(section_slice);
        } else {
            for _ in 0..located_section_padding {
                data_section.push(0);
            }
            if section_kind == SectionKind::DataSection {
                data_section.extend(section_slice);
            } else {
                for _ in 0..section.sh_size {
                    data_section.push(0);
                }
            }
        }
    }

    let dol = DolFile {
        text_sections: vec![
            Section {
                address: base_address,
                data: text_section.into_boxed_slice(),
            },
        ],
        data_sections: vec![
            Section {
                address: data_section_address.unwrap_or(base_address),
                data: data_section.into_boxed_slice(),
            },
        ],
        bss_address: 0,
        bss_size: 0,
        entry_point: 0,
    };

    // println!("{:#?}", dol);

    Linked {
        dol,
        symbol_table,
        sections: located_sections
            .into_iter()
            .map(|s| {
                let section_index = s.section_info.section_index;
                let elf = &parsed_elfs[&s.section_info.member_name];
                let section = &elf.section_headers[section_index];
                let section_name = elf.shdr_strtab
                    .get(section.sh_name as usize)
                    .unwrap()
                    .unwrap();

                let sym_offset =
                    if let Some(sym) = function_symbols_for_section(section_index, elf).next() {
                        // println!("Offset {} for {}", sym.st_value, section_name);
                        sym.st_value as u32
                    } else {
                        0
                    };

                LinkedSection {
                    address: s.address,
                    len: s.len,
                    member_name: s.section_info.member_name,
                    section_name: section_name,
                    kind: s.section_info.kind,
                    sym_offset,
                }
            })
            .collect(),
    }
}
