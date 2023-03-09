use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;

mod rust_interfaces {
    use super::*;

    mod no_trait {
        use super::*;

        struct SDCard {
            root: PathBuf,
            buffer: HashMap<PathBuf, String>,
        }

        impl SDCard {
            pub fn new(root: &'static str) -> Self {
                Self {
                    root: PathBuf::from_str(root).unwrap_or_default(),
                    buffer: HashMap::new(),
                }
            }

            pub fn write(&mut self, path: &Path, data: &str) {
                let mut path_buf = self.root.clone();
                path_buf.push(path);
                self.buffer.insert(path_buf, data.to_owned());
            }

            pub fn read(&self, path: &Path) -> Option<String> {
                let mut path_buf = self.root.clone();
                path_buf.push(path);

                if !self.buffer.contains_key(&path_buf) {
                    return None;
                }

                Some(self.buffer[&path_buf].to_owned())
            }
        }

        struct SPIFlash {
            speed: usize,
            buffer: HashMap<PathBuf, String>,
        }

        impl SPIFlash {
            pub fn new(speed: usize) -> Self {
                Self {
                    speed,
                    buffer: HashMap::new(),
                }
            }

            pub fn write(&mut self, path: &Path, data: &str) {
                let delay = (10.0 / self.speed as f32) as u64;
                std::thread::sleep(Duration::from_secs(delay));
                self.buffer.insert(PathBuf::from(path), data.to_owned());
            }

            pub fn read(&self, path: &Path) -> Option<String> {
                if !self.buffer.contains_key(path) {
                    return None;
                }

                let delay = (10.0 / self.speed as f32) as u64;
                std::thread::sleep(Duration::from_secs(delay));

                Some(self.buffer[path].to_owned())
            }
        }

        fn sd_card_routine(driver: &mut SDCard) -> bool {
            let file_name = Path::new("hello.txt");
            let content = "Hello, World!\n";

            driver.write(&file_name, content);

            let Some(read_content) = driver.read(&file_name) else {
                return false;
            };

            return &read_content == content;
        }

        fn spi_flash_routine(driver: &mut SPIFlash) -> bool {
            let file_name = Path::new("hello.txt");
            let content = "Hello, World!\n";

            driver.write(&file_name, content);

            let Some(read_content) = driver.read(&file_name) else {
                return false;
            };

            return &read_content == content;
        }

        #[test]
        fn test_drivers() {
            let mut sd_card = SDCard::new("sd://");
            let mut flash = SPIFlash::new(5);

            assert!(sd_card_routine(&mut sd_card));
            assert!(spi_flash_routine(&mut flash));
        }
    }

    mod with_trait {
        use super::*;

        pub trait Storage {
            fn write(&mut self, path: &Path, data: &str);
            fn read(&self, path: &Path) -> Option<String>;
        }

        mod sd_card {
            use super::*;

            pub struct SDCard {
                root: PathBuf,
                buffer: HashMap<PathBuf, String>,
            }

            impl SDCard {
                pub fn new(root: &'static str) -> Self {
                    Self {
                        root: PathBuf::from(root),
                        buffer: HashMap::new(),
                    }
                }
            }

            impl Storage for SDCard {
                fn write(&mut self, path: &Path, data: &str) {
                    let mut path_buf = self.root.clone();
                    path_buf.push(path);
                    self.buffer.insert(path_buf, data.to_owned());
                }

                fn read(&self, path: &Path) -> Option<String> {
                    let mut path_buf = self.root.clone();
                    path_buf.push(path);

                    if !self.buffer.contains_key(&path_buf) {
                        return None;
                    }

                    Some(self.buffer[&path_buf].to_owned())
                }
            }
        }

        mod spi_flash {
            use super::*;

            pub struct SPIFlash {
                speed: usize,
                buffer: HashMap<PathBuf, String>,
            }

            impl SPIFlash {
                pub fn new(speed: usize) -> Self {
                    Self {
                        speed,
                        buffer: HashMap::new(),
                    }
                }
            }

            impl Storage for SPIFlash {
                fn write(&mut self, path: &Path, data: &str) {
                    let delay = (10.0 / self.speed as f32) as u64;
                    std::thread::sleep(Duration::from_secs(delay));
                    self.buffer.insert(PathBuf::from(path), data.to_owned());
                }

                fn read(&self, path: &Path) -> Option<String> {
                    if !self.buffer.contains_key(path) {
                        return None;
                    }

                    let delay = (10.0 / self.speed as f32) as u64;
                    std::thread::sleep(Duration::from_secs(delay));

                    Some(self.buffer[path].to_owned())
                }
            }
        }

        use sd_card::SDCard;
        use spi_flash::SPIFlash;

        fn driver_test_routine(driver: &mut impl Storage) -> bool {
            let file_name = Path::new("hello.txt");
            let content = "Hello, World!\n";

            driver.write(&file_name, content);

            let Some(read_content) = driver.read(&file_name) else {
                return false;
            };

            return &read_content == content;
        }

        #[test]
        fn test_drivers() {
            let mut sd_card = SDCard::new("sd://");
            let mut flash = SPIFlash::new(10);

            assert!(driver_test_routine(&mut sd_card));
            assert!(driver_test_routine(&mut flash));
        }

        fn driver_test_routine_ret(mut driver: impl Storage) -> (impl Storage, bool) {
            let file_name = Path::new("hello.txt");
            let content = "Hello, World!\n";

            driver.write(&file_name, content);

            let Some(read_content) = driver.read(&file_name) else {
                return (driver, false);
            };

            return (driver, &read_content == content);
        }

        #[test]
        fn test_drivers_ret() {
            let sd_card = SDCard::new("sd://");
            let flash = SPIFlash::new(10);

            let (_driver, res) = driver_test_routine_ret(sd_card);
            assert!(res);
            let (_driver, res) = driver_test_routine_ret(flash);
            assert!(res);
        }
    }
}

mod generic_bounds {
    use super::*;

    pub trait Storage {
        fn write(&mut self, path: &Path, data: &str);
        fn read(&self, path: &Path) -> Option<String>;
        fn append(&mut self, path: &Path, data: &str);
    }

    mod sd_card {
        use super::*;

        pub struct SDCard {
            root: PathBuf,
            buffer: HashMap<PathBuf, String>,
        }

        impl SDCard {
            pub fn new(root: &'static str) -> Self {
                Self {
                    root: PathBuf::from(root),
                    buffer: HashMap::new(),
                }
            }
        }

        impl Storage for SDCard {
            fn write(&mut self, path: &Path, data: &str) {
                let mut path_buf = self.root.clone();
                path_buf.push(path);
                self.buffer.insert(path_buf, data.to_owned());
            }

            fn read(&self, path: &Path) -> Option<String> {
                let mut path_buf = self.root.clone();
                path_buf.push(path);

                if !self.buffer.contains_key(&path_buf) {
                    return None;
                }

                Some(self.buffer[&path_buf].to_owned())
            }

            fn append(&mut self, path: &Path, data: &str) {
                todo!()
            }
        }
    }

    mod spi_flash {
        use super::*;

        pub struct SPIFlash {
            speed: usize,
            buffer: HashMap<PathBuf, String>,
        }

        impl SPIFlash {
            pub fn new(speed: usize) -> Self {
                Self {
                    speed,
                    buffer: HashMap::new(),
                }
            }

            pub fn speed(&self) -> usize {
                self.speed
            }
        }

        impl Storage for SPIFlash {
            fn write(&mut self, path: &Path, data: &str) {
                let delay = (10.0 / self.speed as f32) as u64;
                std::thread::sleep(Duration::from_secs(delay));
                self.buffer.insert(PathBuf::from(path), data.to_owned());
            }

            fn read(&self, path: &Path) -> Option<String> {
                if !self.buffer.contains_key(path) {
                    return None;
                }

                let delay = (10.0 / self.speed as f32) as u64;
                std::thread::sleep(Duration::from_secs(delay));

                Some(self.buffer[path].to_owned())
            }

            fn append(&mut self, path: &Path, data: &str) {
                let delay = (10.0 / self.speed as f32) as u64;
                std::thread::sleep(Duration::from_secs(delay));

                match self.read(path) {
                    None => self.write(path, data),
                    Some(read_content) => self.write(path, &format!("{read_content}{data}")),
                }
            }
        }
    }

    mod simple_bound {
        use super::*;

        #[derive(Copy, Clone)]
        enum LoggerLevel {
            Disabled,
            Error,
            Warning,
            Info,
            Debug,
        }

        struct Logger<T: Storage> {
            driver: T,
            level: LoggerLevel,
            file_name: Option<PathBuf>,
        }

        impl<T: Storage> Logger<T> {
            pub fn new(level: LoggerLevel, driver: T) -> Self {
                Self {
                    level,
                    driver,
                    file_name: None,
                }
            }

            pub fn new_log_file(&mut self, path: &Path) {
                self.file_name = Some(PathBuf::from(path));
            }

            fn core_log(&mut self, message: &str) {
                let file_name = self.file_name.as_ref().expect("File name is empty!");

                self.driver.append(&file_name, &format!("{message}\n"));
            }

            pub fn dbg(&mut self, message: &str) {
                if (self.level as i32) < (LoggerLevel::Debug as i32) {
                    return;
                }

                self.core_log(message);
            }

            pub fn inf(&mut self, message: &str) {
                if (self.level as i32) < (LoggerLevel::Info as i32) {
                    return;
                }

                self.core_log(message);
            }

            pub fn wrn(&mut self, message: &str) {
                if (self.level as i32) < (LoggerLevel::Warning as i32) {
                    return;
                }

                self.core_log(message);
            }

            pub fn err(&mut self, message: &str) {
                if (self.level as i32) < (LoggerLevel::Error as i32) {
                    return;
                }

                self.core_log(message);
            }

            pub fn print_current_log_file(&self) {
                let file_name = self.file_name.as_ref().expect("File name is empty!");

                match self.driver.read(&file_name) {
                    None => println!("<File is Empty>"),
                    Some(content) => println!("File content: {content}"),
                }
            }
        }

        #[test]
        fn test_logger() {
            let file_name = Path::new("090323.txt");
            let driver = sd_card::SDCard::new("sd://");
            // let driver = spi_flash::SPIFlash::new(20);

            let mut logger = Logger::new(LoggerLevel::Warning, driver);
            logger.new_log_file(&file_name);

            logger.wrn("Power is unstable");
            logger.err("Power state is CRITICAL!!!");
            logger.inf("Power state stable now.");

            logger.print_current_log_file();
        }
    }

    use crate::generic_bounds::spi_flash::SPIFlash;
    use std::fmt::{write, Debug, Formatter};

    impl Debug for SPIFlash {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "SPI Flash with speed {} kHz", self.speed())
        }
    }

    mod multiple_bounds {
        use super::*;
        use logger::*;

        mod logger {
            use super::*;

            #[derive(Copy, Clone)]
            pub enum LoggerLevel {
                Disabled,
                Error,
                Warning,
                Info,
                Debug,
            }

            pub struct Logger<T: Storage> {
                driver: T,
                level: LoggerLevel,
                file_name: Option<PathBuf>,
            }

            impl<T: Storage> Logger<T> {
                pub fn level(&self) -> LoggerLevel {
                    self.level
                }

                pub fn driver(&self) -> &T {
                    &self.driver
                }

                pub fn new(level: LoggerLevel, driver: T) -> Self {
                    Self {
                        level,
                        driver,
                        file_name: None,
                    }
                }

                pub fn new_log_file(&mut self, path: &Path) {
                    self.file_name = Some(PathBuf::from(path));
                }

                fn core_log(&mut self, message: &str) {
                    let file_name = self.file_name.as_ref().expect("File name is empty!");

                    self.driver.append(&file_name, &format!("{message}\n"));
                }

                pub fn dbg(&mut self, message: &str) {
                    if (self.level as i32) < (LoggerLevel::Debug as i32) {
                        return;
                    }

                    self.core_log(message);
                }

                pub fn inf(&mut self, message: &str) {
                    if (self.level as i32) < (LoggerLevel::Info as i32) {
                        return;
                    }

                    self.core_log(message);
                }

                pub fn wrn(&mut self, message: &str) {
                    if (self.level as i32) < (LoggerLevel::Warning as i32) {
                        return;
                    }

                    self.core_log(message);
                }

                pub fn err(&mut self, message: &str) {
                    if (self.level as i32) < (LoggerLevel::Error as i32) {
                        return;
                    }

                    self.core_log(message);
                }

                pub fn print_current_log_file(&self) {
                    let file_name = self.file_name.as_ref().expect("File name is empty!");

                    match self.driver.read(&file_name) {
                        None => println!("<File is Empty>"),
                        Some(content) => println!("File content: {content}"),
                    }
                }
            }
        }

        fn print_logger_info<T: Storage>(logger: &Logger<T>) {
            // format!("Logger <lvl: {:?}, driver: {:?}>", logger.level(), logger.driver());
        }

        #[test]
        fn test_print_logger_info() {
            let logger_sd = Logger::new(LoggerLevel::Info, sd_card::SDCard::new("sd://"));
            let logger_spi = Logger::new(LoggerLevel::Warning, SPIFlash::new(20));

            print_logger_info(&logger_sd);
            print_logger_info(&logger_spi);
        }
    }

    mod where_clause {
        use super::*;
        use logger::*;

        mod logger {
            use super::*;

            #[derive(Debug, Copy, Clone)]
            pub enum LoggerLevel {
                Disabled,
                Error,
                Warning,
                Info,
                Debug,
            }

            pub struct Logger<T: Storage> {
                driver: T,
                level: LoggerLevel,
                file_name: Option<PathBuf>,
            }

            impl<T: Storage> Logger<T> {
                pub fn level(&self) -> LoggerLevel {
                    self.level
                }

                pub fn driver(&self) -> &T {
                    &self.driver
                }

                pub fn new(level: LoggerLevel, driver: T) -> Self {
                    Self {
                        level,
                        driver,
                        file_name: None,
                    }
                }

                pub fn new_log_file(&mut self, path: &Path) {
                    self.file_name = Some(PathBuf::from(path));
                }

                fn core_log(&mut self, message: &str) {
                    let file_name = self.file_name.as_ref().expect("File name is empty!");

                    self.driver.append(&file_name, &format!("{message}\n"));
                }

                pub fn dbg(&mut self, message: &str) {
                    if (self.level as i32) < (LoggerLevel::Debug as i32) {
                        return;
                    }

                    self.core_log(message);
                }

                pub fn inf(&mut self, message: &str) {
                    if (self.level as i32) < (LoggerLevel::Info as i32) {
                        return;
                    }

                    self.core_log(message);
                }

                pub fn wrn(&mut self, message: &str) {
                    if (self.level as i32) < (LoggerLevel::Warning as i32) {
                        return;
                    }

                    self.core_log(message);
                }

                pub fn err(&mut self, message: &str) {
                    if (self.level as i32) < (LoggerLevel::Error as i32) {
                        return;
                    }

                    self.core_log(message);
                }

                pub fn print_current_log_file(&self) {
                    let file_name = self.file_name.as_ref().expect("File name is empty!");

                    match self.driver.read(&file_name) {
                        None => println!("<File is Empty>"),
                        Some(content) => println!("File content: {content}"),
                    }
                }
            }
        }

        fn print_logger_info<T>(logger: &Logger<T>)
        where
            T: Storage + Debug,
        {
            format!(
                "Logger <lvl: {:?}, driver: {:?}>",
                logger.level(),
                logger.driver()
            );
        }

        fn print_logger_info2<T>(logger: &Logger<T>)
        where
            T: Storage,
            T: Debug,
        {
            format!(
                "Logger <lvl: {:?}, driver: {:?}>",
                logger.level(),
                logger.driver()
            );
        }
    }
}

// TODO
mod associate_types {}

/// # Polimosfismo
/// + A possibilidade de alterar a implementação de método, dependendo do objeto utilizado.
/// + O mecanismo para definir qual implementação será utilizada é denominado _dispatch_.
/// + O _dispatch_ pode ser **estático** (decidido em tempo de compilação)
/// ou **dinâmico** (decidido em tempo de execução).
/// + Rust sempre prioriza o **_dispatch_ estático**, porém suporta o **_dispatch_ dinâmico**,
/// através dos **_trait objects_**
mod dynamic_dispatch {
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String {
            format!("u8: {}", self)
        }
    }

    impl Foo for String {
        fn method(&self) -> String {
            format!("string: {}", self)
        }
    }

    mod monomorphization {
        use super::*;

        fn do_something<T: Foo>(x: T) {
            x.method();
        }

        #[test]
        fn test_monomorphization() {
            let x = 7u8;
            let y = "Hello".to_string();

            do_something(x);
            do_something(y);
        }

        fn do_something_u8(x: u8) {
            x.method();
        }

        fn do_something_string(x: String) {
            x.method();
        }
    }

    fn do_something_dyn(x: &dyn Foo) {
        x.method();
    }

    #[test]
    fn test_dyn_cast() {
        let x = 7u8;
        do_something_dyn(&x as &dyn Foo);
    }

    #[test]
    fn test_dyn_coercing() {
        let x = "Hello".to_string();
        // let x = 7u8;
        do_something_dyn(&x);
    }

    /// # Por que _trait objects_ usando ponteiros?
    ///
    fn do_something_dyn_box(x: Box<dyn Foo>) {
        x.method();
    }

    #[test]
    fn test_dyn_box() {
        let x = Box::new("Hello".to_string());
        // let x = Box::new(7u8);
        do_something_dyn_box(x);
    }

    // TODO Pesquisar!!
    mod object_safety {}
}

mod impl_vs_dyn {}

mod struct_states_with_traits {}

mod iterator_trait {}

mod from_and_into_traits {}

mod read_and_write_traits {}

mod operator_overload {}

mod default_trait {}

mod drop_trait {}

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
