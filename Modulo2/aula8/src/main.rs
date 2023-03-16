mod storage;

mod default_trait {
    use super::*;
    use storage::SDCard;

    impl Default for SDCard {
        fn default() -> Self {
            SDCard::mount("sd://")
        }
    }

    #[test]
    fn test_sd_card_default() {
        let card1 = SDCard::mount("sd://");
        let card2 = SDCard::default();

        println!("Card1: {}", card1.root().name());
        println!("Card2: {}", card2.root().name());
    }
}

#[allow(unused)]
mod copy_and_clone {
    #[derive(Clone, Default)]
    struct CloneSmall {
        a: [u8; 10],
    }

    #[derive(Copy, Clone, Default)]
    struct CopySmall {
        a: [u8; 10],
    }

    fn move_small(_obj: CloneSmall) {}

    fn copy_small(_obj: CopySmall) {}

    #[test]
    fn test_small() {
        let _copy_non_default = CopySmall { a: [0u8; 10] };
        let obj_copy = CopySmall::default();
        let obj_clone = CloneSmall::default();

        // move_small(obj_clone.clone());
        move_small(obj_clone);
        copy_small(obj_copy);
    }

    #[derive(Clone)]
    /// [u8; 1000] não implementa Default!
    /// O máximo é [u8; 32]
    struct CloneLarge {
        a: [u8; 1000],
    }

    #[derive(Copy, Clone)]
    /// [u8; 1000] não implementa Default!
    /// O máximo é [u8; 32]
    struct CopyLarge {
        a: [u8; 1000],
    }

    impl Default for CloneLarge {
        fn default() -> Self {
            Self { a: [0u8; 1000] }
        }
    }

    impl Default for CopyLarge {
        fn default() -> Self {
            Self { a: [0u8; 1000] }
        }
    }

    fn move_large(_obj: CloneLarge) {}

    fn copy_large(_obj: CopyLarge) {}

    #[test]
    fn test_large() {
        let obj_copy = CopyLarge { a: [0u8; 1000] };
        let obj_clone = CloneLarge { a: [0u8; 1000] };

        move_large(obj_clone.clone());
        move_large(obj_clone);
        copy_large(obj_copy);
    }

    // !!! RECAP !!!
    // Estruturas por padrão usamos Clone. Podemos usar Copy se a estrutura for pequena e tivermos
    // uma restrição de algum trait.

    #[test]
    #[allow(unused)]
    fn implicity_copy() {
        let many_copies: [CopyLarge; 5];

        many_copies = [CopyLarge::default(); 5];

        let many_clones: [CloneLarge; 5];

        // many_clones = [CloneLarge::default(); 5];
    }
}

mod from_and_into_traits {
    use crate::storage::{Directory, FSEntry, File};

    #[test]
    fn test_dir_to_entry() {
        let dir = Directory::new_empty("hello/");
        let _dir_entry = FSEntry::from(dir);
        // let _dir_entry: FSEntry = dir.into();
    }

    #[test]
    fn test_file_to_entry() {
        let file = File::new("hello.txt");

        let _file_entry: FSEntry = file.into();
    }

    #[test]
    fn test_entry_to_dir() {
        let dir_entry = FSEntry::new_dir("hello/");
        let file_entry = FSEntry::new_file("hello.txt");

        // let right_dir: Result<Directory, _> = dir_entry.try_into();
        let right_dir = Directory::try_from(dir_entry);
        let wrong_dir = Directory::try_from(file_entry);

        println!("First conversion result was done? {}", right_dir.is_ok());
        println!("Second conversion result was done? {}", wrong_dir.is_ok());
    }

    #[test]
    fn test_entry_to_file() {
        let dir_entry = FSEntry::new_dir("hello/");
        let file_entry = FSEntry::new_file("hello.txt");

        let right_file: Result<File, _> = file_entry.try_into();
        let wrong_file: Result<File, _> = dir_entry.try_into();

        println!("First conversion result was done? {}", right_file.is_ok());
        println!("Second conversion result was done? {}", wrong_file.is_ok());
    }
}

mod display {
    use crate::storage::{Directory, FSEntry, File, SDCard};

    #[test]
    fn test_card_commit() {
        let mut card = SDCard::default();
        let mut dir = Directory::new_empty("hello/");
        let mut file = File::new("world.txt");

        card.root_mut().add_entry(FSEntry::new_file("empty.txt"));
        file.append("Hello, World!");
        dir.add_entry(file.into());
        card.root_mut().add_entry(dir.into());

        card.commit();
    }
}

mod operator_overload {
    use super::storage::SDCard;
    use crate::storage::{Directory, FSEntry, File};

    #[test]
    fn test_dir_add() {
        let mut card = SDCard::default();
        let dir = Directory::new_empty("hello/");
        let mut file = File::new("world.txt");

        card.root_mut().add_entry(FSEntry::new_file("empty.txt"));
        file.append("Hello, World!");
        let dir = dir + file;
        card.root_mut().add_entry(dir.into());

        card.commit();
    }

    #[test]
    fn test_dir_add_assign() {
        let mut card = SDCard::default();
        let mut dir = Directory::new_empty("hello/");
        let mut file = File::new("world.txt");

        card.root_mut().add_entry(FSEntry::new_file("empty.txt"));
        file.append("Hello, World!");
        dir += file;
        card.root_mut().add_entry(dir.into());

        card.commit();
    }

    #[test]
    fn test_file_add_assign() {
        let mut card = SDCard::default();
        let mut dir = Directory::new_empty("hello/");
        let mut file = File::new("world.txt");

        card.root_mut().add_entry(FSEntry::new_file("empty.txt"));
        file += "Hello, World!";
        dir += file;
        card.root_mut().add_entry(dir.into());

        card.commit();
    }

    #[test]
    fn test_sd_card_add_assign() {
        let mut card = SDCard::default();
        let mut dir = Directory::new_empty("hello/");
        let mut file = File::new("world.txt");

        card += File::new("empty.txt").into();
        file += "Hello, World!";
        dir += file;
        card += dir.into();

        card.commit();
    }
}

mod iterator_trait {
    use crate::storage::{Directory, File, SDCard};

    #[test]
    fn test_iter_only_files() {
        let mut card = SDCard::default();
        for i in 0..3 {
            card += File::new(&format!("empty{}.txt", i)).into();
        }

        card.commit();

        for (i, entry) in card.root().iter().enumerate() {
            println!("{}: {}", i, entry);
        }
    }

    #[test]
    fn test_iter_dir_and_files() {
        let mut card = SDCard::default();
        for i in 0..3 {
            card += File::new(&format!("empty{}.txt", i)).into();
            card += Directory::new_empty(&format!("hello{}/", i)).into();
        }

        card.commit();

        for (i, entry) in card.root().iter().enumerate() {
            println!("{}: {}", i, entry);
        }
    }
}

mod drop_trait {
    use crate::storage::{Directory, File, SDCard};

    // impl Drop for SDCard {
    //     fn drop(&mut self) {
    //         self.commit();
    //     }
    // }

    #[test]
    fn test_drop() {
        let mut card = SDCard::default();
        for i in 0..3 {
            card += File::new(&format!("empty{}.txt", i)).into();
            card += Directory::new_empty(&format!("hello{}/", i)).into();
        }
    }

    #[test]
    fn test_drop_with_scope() {
        {
            let mut card = SDCard::default();
            for i in 0..3 {
                card += File::new(&format!("empty{}.txt", i)).into();
                card += Directory::new_empty(&format!("hello{}/", i)).into();
            }

            println!("Before end of scope");
        }
        println!("End of function");
    }
}

mod super_traits {
    use std::io::{Read, Write};

    pub trait Storage {}

    pub trait StorageExt: Read + Write {}

    // !!! Copy é um super trait que depende de Clone. Desta forma, quando implementamos o Copy
    //     para alguma struct, devemos também implementar Clone para essa mesma struct.

    // !!! Para usar o dynamic dispatch com mais de 1 trait usamos super trait
    #[allow(unused)]
    struct Hello {
        a: Box<dyn Storage>,
        // b: Box<dyn Storage + Read + Write>,
        c: Box<dyn StorageExt>,
    }
}

// TODO Próximas aulas
mod deref_and_deref_mut {}

mod dynamic_dispatch2 {
    // TODO Próximas aulas
    mod object_safety {}
}

mod exercicio {
    #[derive(Debug, PartialEq, Eq)]
    #[allow(unused)]
    pub enum Comparison {
        Equal,
        Sublist,
        Superlist,
        Unequal,
    }

    #[allow(unused)]
    pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
        todo!()
    }

    #[test]
    fn test_exercicio() {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
