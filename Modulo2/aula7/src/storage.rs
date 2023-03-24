use std::collections::HashMap;
use std::path::{Path, PathBuf};
// &str, String
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug)]
pub struct SDCard {
    root: PathBuf,
    buffer: HashMap<PathBuf, String>,
}

pub struct SPIFlash {
    speed: usize,
    buffer: HashMap<PathBuf, String>,
}

pub mod v1 {
    use super::*;

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
}

pub mod v2 {
    use super::*;

    pub trait Storage {
        fn write(&mut self, path: &Path, data: &str);
        fn read(&self, path: &Path) -> Option<String>;
        fn flush(&mut self) {
            println!("Flush default impl");
        }
    }

    impl Storage for SDCard {
        fn write(&mut self, path: &Path, data: &str) {
            let mut path_buf = self.root.clone();
            path_buf.push(path);
            self.buffer.insert(path_buf, data.to_owned());
            self.flush();
        }

        fn read(&self, path: &Path) -> Option<String> {
            let mut path_buf = self.root.clone();
            path_buf.push(path);

            if !self.buffer.contains_key(&path_buf) {
                return None;
            }

            Some(self.buffer[&path_buf].to_owned())
        }

        fn flush(&mut self) {
            println!("SD Card flushing...");
        }
    }

    impl Storage for SPIFlash {
        fn write(&mut self, path: &Path, data: &str) {
            let delay = (10.0 / self.speed as f32) as u64;
            std::thread::sleep(Duration::from_secs(delay));
            self.buffer.insert(PathBuf::from(path), data.to_owned());
            self.flush();
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

pub mod v3 {
    use super::*;
    use std::fmt::{Debug, Formatter};

    #[derive(Debug, Copy, Clone)]
    #[allow(dead_code)]
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
        pub fn new(level: LoggerLevel, driver: T) -> Self {
            Self {
                level,
                driver,
                file_name: None,
            }
        }

        pub fn level(&self) -> LoggerLevel {
            self.level
        }

        pub fn driver(&self) -> &T {
            &self.driver
        }

        pub fn new_log_file(&mut self, path: &Path) {
            self.file_name = Some(PathBuf::from(path));
        }

        fn core_log(&mut self, message: &str, ref_level: LoggerLevel) {
            if (self.level as i32) < (ref_level as i32) {
                return;
            }

            let file_name = self.file_name.as_ref().expect("File name is empty!");

            self.driver.append(&file_name, &format!("{message}\n"));
        }

        #[allow(dead_code)]
        pub fn dbg(&mut self, message: &str) {
            self.core_log(message, LoggerLevel::Debug);
        }

        pub fn inf(&mut self, message: &str) {
            self.core_log(message, LoggerLevel::Info);
        }

        pub fn wrn(&mut self, message: &str) {
            self.core_log(message, LoggerLevel::Warning);
        }

        pub fn err(&mut self, message: &str) {
            self.core_log(message, LoggerLevel::Error);
        }

        pub fn print_current_log_file(&self) {
            let file_name = self.file_name.as_ref().expect("File name is empty!");

            match self.driver.read(&file_name) {
                None => println!("<File is Empty>"),
                Some(content) => println!("File content: {content}"),
            }
        }
    }

    pub trait Storage {
        fn write(&mut self, path: &Path, data: &str);
        fn read(&self, path: &Path) -> Option<String>;
        fn append(&mut self, path: &Path, data: &str);
        fn flush(&mut self) {
            println!("Flush default");
        }
    }

    impl Storage for SDCard {
        fn write(&mut self, path: &Path, data: &str) {
            let mut path_buf = self.root.clone();
            path_buf.push(path);
            self.buffer.insert(path_buf, data.to_owned());
            self.flush();
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
            match self.read(path) {
                None => self.write(path, data),
                Some(read_content) => {
                    let content = format!("{}{}", read_content, data);
                    self.write(path, &content);
                }
            }
            self.flush();
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

    impl Debug for SPIFlash {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "SPI Flash with speed {} kHz", self.speed())
        }
    }
}
