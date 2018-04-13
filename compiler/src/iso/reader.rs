use super::virtual_file_system::{Directory, File, Node};
use super::{FstEntry, FstNodeType, consts::*};
use byteorder::{ByteOrder, BE};
use std::fs;
use std::io::{Read, Result};
use std::path::Path;
use std::str;

pub fn load_iso_buf<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let len = file.metadata()?.len();
    let mut buf = Vec::with_capacity(len as usize + 1);
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn load_iso<'a>(buf: &'a [u8]) -> Directory<'a> {
    let fst_offset = BE::read_u32(&buf[OFFSET_FST_OFFSET..]) as usize;
    let mut pos = fst_offset;
    let num_entries = BE::read_u32(&buf[fst_offset + 8..]) as usize;
    let string_table_offset = num_entries * 0xC;

    let mut fst_entries = Vec::with_capacity(num_entries);
    for _ in 0..num_entries {
        let kind = if buf[pos] == 0 {
            FstNodeType::File
        } else {
            FstNodeType::Directory
        };
        pos += 2;

        let cur_pos = pos;
        let string_offset = BE::read_u16(&buf[pos..]) as usize;

        pos = string_offset + string_table_offset + fst_offset;
        let mut end = pos;
        while buf[end] != 0 {
            end += 1;
        }
        let relative_file_name = str::from_utf8(&buf[pos..end]).unwrap();

        pos = cur_pos + 2;
        let file_offset_parent_dir = BE::read_u32(&buf[pos..]) as usize;
        let file_size_next_dir_index = BE::read_u32(&buf[pos + 4..]) as usize;
        pos += 8;

        fst_entries.push(FstEntry {
            kind,
            relative_file_name,
            file_offset_parent_dir,
            file_size_next_dir_index,
            file_name_offset: 0,
        });
    }

    let mut root_dir = Directory::new("root");
    let mut sys_data = Directory::new("&&systemdata");

    sys_data
        .children
        .push(Node::File(File::new("iso.hdr", &buf[..HEADER_LENGTH])));

    let dol_offset = BE::read_u32(&buf[OFFSET_DOL_OFFSET..]) as usize;

    sys_data.children.push(Node::File(File::new(
        "AppLoader.ldr",
        &buf[HEADER_LENGTH..dol_offset],
    )));

    sys_data.children.push(Node::File(File::new(
        "Start.dol",
        &buf[dol_offset..fst_offset],
    )));

    let fst_size = BE::read_u32(&buf[OFFSET_FST_SIZE..]) as usize;
    sys_data.children.push(Node::File(File::new(
        "Game.toc",
        &buf[fst_offset..][..fst_size],
    )));

    root_dir.children.push(Node::Directory(Box::new(sys_data)));

    let mut count = 1;

    while count < num_entries {
        let entry = &fst_entries[count];
        if fst_entries[count].kind == FstNodeType::Directory {
            let mut dir = Directory::new(entry.relative_file_name);

            while count < entry.file_size_next_dir_index - 1 {
                count = get_dir_structure_recursive(count + 1, &fst_entries, &mut dir, buf);
            }

            root_dir.children.push(Node::Directory(Box::new(dir)));
        } else {
            let file = get_file_data(&fst_entries[count], buf);
            root_dir.children.push(Node::File(file));
        }
        count += 1;
    }

    root_dir
}

fn get_dir_structure_recursive<'a>(
    mut cur_index: usize,
    fst: &[FstEntry<'a>],
    parent_dir: &mut Directory<'a>,
    buf: &'a [u8],
) -> usize {
    let entry = &fst[cur_index];

    if entry.kind == FstNodeType::Directory {
        let mut dir = Directory::new(entry.relative_file_name);

        while cur_index < entry.file_size_next_dir_index - 1 {
            cur_index = get_dir_structure_recursive(cur_index + 1, fst, &mut dir, buf);
        }

        parent_dir.children.push(Node::Directory(Box::new(dir)));
    } else {
        let file = get_file_data(entry, buf);
        parent_dir.children.push(Node::File(file));
    }

    cur_index
}

fn get_file_data<'a>(fst_data: &FstEntry<'a>, buf: &'a [u8]) -> File<'a> {
    let data = &buf[fst_data.file_offset_parent_dir..][..fst_data.file_size_next_dir_index];
    File::new(fst_data.relative_file_name, data)
}
