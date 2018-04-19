use std::collections::BTreeMap;
use syn::{self, synom::ParseError};

pub struct Assembler<'a> {
    symbol_table: BTreeMap<&'a str, u32>,
    program_counter: u32,
}

pub struct Instruction {
    pub address: u32,
    pub data: u32,
}

impl<'a> Assembler<'a> {
    pub fn new(symbol_table: BTreeMap<&str, u32>) -> Assembler {
        Assembler {
            symbol_table,
            program_counter: 0,
        }
    }

    pub fn assemble_all_lines(&mut self, lines: &[&str]) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let filtered_lines = lines
            .iter()
            .map(|l| reduce_line_to_code(l))
            .filter(|l| !l.is_empty());

        for line in filtered_lines {
            if line.ends_with(':') {
                self.program_counter =
                    parse_program_counter_label(line).expect("Couldn't parse Address Label");
            } else {
                let instruction = self.parse_instruction(line);
                instructions.push(instruction);
                self.program_counter += 4;
            }
        }

        instructions
    }

    fn parse_instruction(&self, line: &str) -> Instruction {
        let data;

        if line.starts_with("bl ") {
            let operand = &line[3..];
            let destination = self.resolve_symbol(operand);
            data = build_branch_instruction(self.program_counter, destination, false, true);
        } else if line.starts_with("b ") {
            let operand = &line[2..];
            let destination = self.resolve_symbol(operand);
            data = build_branch_instruction(self.program_counter, destination, false, false);
        } else if line.starts_with("u32 ") {
            data = parse_u32_literal(&line[4..]).unwrap();
        } else if line == "nop" {
            data = 0x60000000;
        } else {
            panic!("Unknown instruction: {}", line);
        }

        Instruction {
            address: self.program_counter,
            data: data,
        }
    }

    fn resolve_symbol(&self, symbol: &str) -> u32 {
        parse_u32_literal(symbol).unwrap_or_else(|_| {
            *self.symbol_table
                .get(symbol)
                .unwrap_or_else(|| panic!("The symbol \"{}\" wasn't found", symbol))
        })
    }
}

fn reduce_line_to_code(line: &str) -> &str {
    let mut line = line;
    if let Some(index) = line.find(';') {
        line = &line[..index];
    }
    line.trim()
}

fn parse_i64_literal(literal: &str) -> Result<i64, ParseError> {
    let val: syn::LitInt = syn::parse_str(literal)?;
    Ok(val.value() as i64)
}

fn parse_u32_literal(literal: &str) -> Result<u32, ParseError> {
    parse_i64_literal(literal).map(|i| i as u32)
}

fn parse_program_counter_label(line: &str) -> Result<u32, ParseError> {
    let line = &line[..line.len() - 1];
    parse_u32_literal(line)
}

fn build_branch_instruction(address: u32, destination: u32, aa: bool, lk: bool) -> u32 {
    let bits_dest = if aa {
        destination
    } else {
        destination - address
    };
    let bits_aa = if aa { 1 } else { 0 };
    let bits_lk = if lk { 1 } else { 0 };

    (18 << 26) | (0x3FFFFFC & bits_dest) | (bits_aa << 1) | bits_lk
}
