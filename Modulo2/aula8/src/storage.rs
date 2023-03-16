use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

pub struct SDCard {
    root: Directory,
}

impl SDCard {
    pub fn mount(root: &'static str) -> Self {
        Self {
            root: Directory::new_empty(root),
        }
    }

    pub fn root(&self) -> &Directory {
        &self.root
    }

    pub fn root_mut(&mut self) -> &mut Directory {
        &mut self.root
    }

    pub fn commit(&mut self) {
        println!("Writing bellow tree into SDCard...\n{}", self.root);
    }
}

pub struct Directory {
    level: usize,
    name: String,
    children: Vec<FSEntry>,
}

pub struct File {
    level: usize,
    name: String,
    content: String,
}

pub enum FSEntry {
    Directory(Directory),
    File(File),
}

impl Directory {
    pub fn new_empty(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            children: vec![],
            level: 0,
        }
    }

    #[allow(unused)]
    pub fn with_entries(mut self, mut entries: Vec<FSEntry>) -> Self {
        for entry in entries.iter_mut() {
            entry.set_level(self.level + 1);
        }
        self.children = entries;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_entry(&mut self, mut entry: FSEntry) {
        let None = self
            .children
            .iter()
            .find(|x| entry.name() == x.name()) else
        {
            println!("Entry already exist");
            return;
        };

        entry.set_level(self.level + 1);
        self.children.push(entry);
    }

    pub fn set_level(&mut self, level: usize) {
        self.level = level;
        for child in self.children.iter_mut() {
            child.set_level(self.level + 1);
        }
    }
}

impl File {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            content: String::new(),
            level: 0,
        }
    }

    #[allow(unused)]
    pub fn with_content(mut self, content: String) -> Self {
        self.content = content;
        self
    }

    pub fn set_level(&mut self, level: usize) {
        self.level = level;
    }

    pub fn append(&mut self, content: &str) {
        self.content += content;
    }
}

impl FSEntry {
    pub fn name(&self) -> &str {
        match self {
            FSEntry::Directory(Directory { name, .. }) => name,
            FSEntry::File(File { name, .. }) => name,
        }
    }

    pub fn new_dir(name: &str) -> FSEntry {
        FSEntry::Directory(Directory::new_empty(name))
    }

    pub fn new_file(name: &str) -> FSEntry {
        FSEntry::File(File::new(name))
    }

    pub fn set_level(&mut self, level: usize) {
        match self {
            FSEntry::Directory(dir) => dir.set_level(level),
            FSEntry::File(f) => f.set_level(level),
        }
    }

    pub fn level(&self) -> usize {
        match self {
            FSEntry::Directory(dir) => dir.level,
            FSEntry::File(f) => f.level,
        }
    }
}

mod from_into {
    use super::*;

    impl From<Directory> for FSEntry {
        fn from(value: Directory) -> Self {
            FSEntry::Directory(value)
        }
    }

    impl From<File> for FSEntry {
        fn from(value: File) -> Self {
            FSEntry::File(value)
        }
    }

    impl TryFrom<FSEntry> for Directory {
        type Error = String;

        fn try_from(value: FSEntry) -> Result<Self, Self::Error> {
            match value {
                FSEntry::Directory(dir) => Ok(dir),
                FSEntry::File(_) => Err("Cannot convert file into directory".to_owned()),
            }
        }
    }

    impl TryFrom<FSEntry> for File {
        type Error = String;

        fn try_from(value: FSEntry) -> Result<Self, Self::Error> {
            match value {
                FSEntry::Directory(_) => Err("Cannot convert directory into file".to_owned()),
                FSEntry::File(file) => Ok(file),
            }
        }
    }
}

mod display {
    use super::*;

    impl Display for Directory {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut children = String::new();

            for child in self.children.iter() {
                children += &format!("{}{}\n", "\t".repeat(child.level()), child);
            }

            let new_line = if children.is_empty() { "" } else { "\n" };

            write!(f, "{}{}{}", self.name, new_line, children)
        }
    }

    impl Display for File {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} -> {{{}}}", self.name, self.content)
        }
    }

    impl Display for FSEntry {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                FSEntry::Directory(dir) => write!(f, "{}", dir),
                FSEntry::File(file) => write!(f, "{}", file),
            }
        }
    }
}

mod operator_overload {
    use super::*;

    impl Add<File> for Directory {
        type Output = Directory;

        fn add(mut self, rhs: File) -> Self::Output {
            self.add_entry(FSEntry::File(rhs));
            self
        }
    }

    impl AddAssign<File> for Directory {
        fn add_assign(&mut self, rhs: File) {
            self.add_entry(FSEntry::File(rhs));
        }
    }

    impl AddAssign<Directory> for Directory {
        fn add_assign(&mut self, rhs: Directory) {
            self.add_entry(FSEntry::Directory(rhs));
        }
    }

    impl<'a> AddAssign<&'a str> for File {
        fn add_assign(&mut self, rhs: &'a str) {
            self.append(rhs);
        }
    }

    impl AddAssign<FSEntry> for SDCard {
        fn add_assign(&mut self, rhs: FSEntry) {
            self.root.add_entry(rhs);
        }
    }

    // !!! Lista de operadores pode ser consultada em 'std::ops'
}

mod iterator {
    use super::*;

    pub struct FSEntryIter<'a> {
        root: &'a Directory,
        index: usize,
    }

    impl<'a> FSEntryIter<'a> {
        pub fn new(dir: &'a Directory) -> Self {
            Self {
                root: dir,
                index: 0,
            }
        }
    }

    impl<'a> Iterator for FSEntryIter<'a> {
        type Item = &'a FSEntry;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index == self.root.children.len() {
                return None;
            }

            let el = &self.root.children[self.index];
            self.index += 1;

            Some(el)
        }
    }

    impl Directory {
        pub fn iter(&self) -> FSEntryIter {
            FSEntryIter::new(self)
        }
    }
}
