#[derive(Debug)]
pub enum Node<'a> {
    Directory(Box<Directory<'a>>),
    File(File<'a>),
}

impl<'a> Node<'a> {
    pub fn as_directory(&self) -> Option<&Directory<'a>> {
        if let &Node::Directory(ref dir) = self {
            Some(dir)
        } else {
            None
        }
    }

    pub fn as_directory_mut(&mut self) -> Option<&mut Directory<'a>> {
        if let &mut Node::Directory(ref mut dir) = self {
            Some(dir)
        } else {
            None
        }
    }

    pub fn as_file(&self) -> Option<&File<'a>> {
        if let &Node::File(ref file) = self {
            Some(file)
        } else {
            None
        }
    }

    pub fn as_file_mut(&mut self) -> Option<&mut File<'a>> {
        if let &mut Node::File(ref mut file) = self {
            Some(file)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Directory<'a> {
    pub id: u16,
    pub name: &'a str,
    pub children: Vec<Node<'a>>,
}

impl<'a> Directory<'a> {
    pub fn new(name: &'a str) -> Directory<'a> {
        Self {
            id: 0,
            name,
            children: Vec::new(),
        }
    }

    pub fn main_dol_mut(&mut self) -> Option<&mut File<'a>> {
        let sys_dir = self.children
            .iter_mut()
            .filter_map(|c| c.as_directory_mut())
            .find(|d| d.name == "&&systemdata")?;
        let dol = sys_dir
            .children
            .iter_mut()
            .filter_map(|c| c.as_file_mut())
            .find(|f| f.name.ends_with(".dol"))?;
        Some(dol)
    }

    pub fn banner_mut(&mut self) -> Option<&mut File<'a>> {
        let banner = self.children
            .iter_mut()
            .filter_map(|c| c.as_file_mut())
            .find(|f| f.name == "opening.bnr")?;
        Some(banner)
    }

    pub fn resolve_path(&self, path: &str) -> Option<&File<'a>> {
        let mut dir = self;
        let mut segments = path.split('/').peekable();

        while let Some(segment) = segments.next() {
            if segments.peek().is_some() {
                // Must be a folder
                dir = dir.children
                    .iter()
                    .filter_map(|c| c.as_directory())
                    .find(|d| d.name == segment)?;
            } else {
                return dir
                    .children
                    .iter()
                    .filter_map(|c| c.as_file())
                    .find(|f| f.name == segment);
            }
        }
        None
    }
}

pub struct File<'a> {
    pub id: u16,
    pub name: &'a str,
    pub data: &'a [u8],
}

impl<'a> File<'a> {
    pub fn new(name: &'a str, data: &'a [u8]) -> File<'a> {
        Self { id: 0, name, data }
    }
}

use std::fmt;

impl<'a> fmt::Debug for File<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

// use std::path::Path;
// use std::io::{Read, Result};
// use std::fs::{read_dir, self};

// pub fn import_from_disk<P: AsRef<Path>>(path: P) -> Result<Directory> {
//     let path = path.as_ref();
//     let name = path.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();

//     let mut directory = Directory {
//         id: 0,
//         name,
//         children: Vec::new(),
//     };

//     for entry in read_dir(path)? {
//         let entry = entry?;
//         let file_type = entry.file_type()?;
//         if file_type.is_dir() {
//             let child = import_from_disk(entry.path())?;
//             directory.children.push(Node::Directory(Box::new(child)));
//         } else {
//             let name = entry.file_name().to_string_lossy().into_owned();

//             let mut file = fs::File::open(entry.path())?;
//             let len = file.metadata()?.len();
//             let mut data = Vec::with_capacity(len as usize + 1);
//             file.read_to_end(&mut data)?;

//             directory.children.push(Node::File(File {
//                 id: 0,
//                 name,
//                 data,
//             }));
//         }
//     }

//     Ok(directory)
// }
