mod storage;

mod rust_interfaces {
    mod no_trait {
        use crate::storage::{SDCard, SPIFlash};
        use std::path::Path;

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
        use crate::storage::v2::Storage;
        use crate::storage::{SDCard, SPIFlash};
        use std::path::Path;

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

            // let mut a = 0i32;
            // driver_test_routine(&mut a);

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
    mod simple_bound {
        use crate::storage::v3::{Logger, LoggerLevel};
        #[allow(unused_imports)]
        use crate::storage::{SDCard, SPIFlash};
        use std::path::Path;

        #[test]
        fn test_logger() {
            let file_name = Path::new("090323.txt");
            let driver = SDCard::new("sd://");
            let driver = SPIFlash::new(20);

            let mut logger = Logger::new(LoggerLevel::Info, driver);
            logger.new_log_file(&file_name);

            logger.wrn("Power is unstable");
            logger.err("Power state is CRITICAL!!!");
            logger.inf("Power state stable now.");

            logger.print_current_log_file();
        }
    }

    mod multiple_bounds {
        use crate::storage::v3::{Logger, LoggerLevel, Storage};
        use crate::storage::{SDCard, SPIFlash};
        use std::fmt::Debug;

        fn print_logger_info<T: Storage + Debug>(logger: &Logger<T>) {
            println!(
                "Logger <lvl: {:?}, driver: {:?}>",
                logger.level(),
                logger.driver()
            );
        }

        fn foo<T: PartialEq>(a: &[T], b: &[T]) -> bool {
            todo!()
        }

        #[test]
        fn test_print_logger_info() {
            let logger_sd = Logger::new(LoggerLevel::Info, SDCard::new("sd://"));
            let logger_spi = Logger::new(LoggerLevel::Warning, SPIFlash::new(20));

            print_logger_info(&logger_sd);
            print_logger_info(&logger_spi);
        }
    }

    mod where_clause {
        use crate::storage::v3::{Logger, Storage};
        use std::fmt::Debug;

        #[allow(unused)]
        fn print_logger_info<T>(logger: &Logger<T>) -> ()
        where
            T: Storage + Debug,
        {
            format!(
                "Logger <lvl: {:?}, driver: {:?}>",
                logger.level(),
                logger.driver()
            );
        }

        #[allow(unused)]
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

mod associate_types {
    pub trait ProtoIterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    pub struct RingPointer<'a> {
        index: usize,
        coll: &'a Vec<u32>,
    }

    impl<'a> RingPointer<'a> {
        pub fn new(coll: &'a Vec<u32>) -> Self {
            Self { index: 0, coll }
        }
    }

    impl<'a> ProtoIterator for RingPointer<'a> {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.coll.is_empty() {
                return None;
            }

            let el = Some(self.coll[self.index]);

            self.index += 1;
            if self.index == self.coll.len() {
                self.index = 0;
            }

            el
        }
    }

    #[test]
    fn ring_pointer_empty() {
        let coll = vec![];
        let mut ring_ptr = RingPointer::new(&coll);

        assert_eq!(ring_ptr.next(), None);
    }

    #[test]
    fn ring_pointer_one_element() {
        let coll = vec![1];
        let mut ring_ptr = RingPointer::new(&coll);

        assert_eq!(ring_ptr.next(), Some(1));
        assert_eq!(ring_ptr.next(), Some(1));
    }

    #[test]
    fn ring_pointer_two_elements() {
        let coll = vec![1, 2];
        let mut ring_pointer = RingPointer::new(&coll);

        assert_eq!(ring_pointer.next(), Some(1));
        assert_eq!(ring_pointer.next(), Some(2));
        assert_eq!(ring_pointer.next(), Some(1));
    }
}

mod generics_vs_associate_tyes {
    pub trait ProtoIterator<I> {
        fn next(&mut self) -> Option<I>;
    }

    pub struct RingPointer<'a> {
        index: usize,
        coll: &'a Vec<u32>,
    }

    impl<'a> RingPointer<'a> {
        #[allow(unused)]
        pub fn new(coll: &'a Vec<u32>) -> Self {
            Self { index: 0, coll }
        }
    }

    impl<'a> ProtoIterator<u32> for RingPointer<'a> {
        fn next(&mut self) -> Option<u32> {
            if self.coll.is_empty() {
                return None;
            }

            let el = Some(self.coll[self.index]);

            self.index += 1;
            if self.index == self.coll.len() {
                self.index = 0;
            }

            el
        }
    }

    impl<'a> ProtoIterator<String> for RingPointer<'a> {
        fn next(&mut self) -> Option<String> {
            todo!()
        }
    }
}

mod associate_types_and_generics {
    use std::fmt::Debug;

    pub trait ProtoIterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    pub struct RingPointer<'a, T: Debug> {
        index: usize,
        coll: &'a Vec<T>,
    }

    impl<'a, T: Debug> RingPointer<'a, T> {
        pub fn new(coll: &'a Vec<T>) -> Self {
            Self { index: 0, coll }
        }
    }

    impl<'a, T: Debug> ProtoIterator for RingPointer<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.coll.is_empty() {
                return None;
            }

            let el = Some(&self.coll[self.index]);
            let el = dbg!(el);

            self.index += 1;
            if self.index == self.coll.len() {
                self.index = 0;
            }

            el
        }
    }

    #[test]
    fn ring_pointer_empty() {
        let coll = Vec::<u32>::new();
        let mut ring_ptr = RingPointer::new(&coll);

        assert_eq!(ring_ptr.next(), None);
    }

    #[test]
    fn ring_pointer_one_element() {
        let coll = vec![1];
        let mut ring_ptr = RingPointer::new(&coll);

        assert_eq!(ring_ptr.next(), Some(&1));
        assert_eq!(ring_ptr.next(), Some(&1));
    }

    #[test]
    fn ring_pointer_two_elements() {
        let coll = vec![1, 2];
        let mut ring_pointer = RingPointer::new(&coll);

        assert_eq!(ring_pointer.next(), Some(&1));
        assert_eq!(ring_pointer.next(), Some(&2));
        assert_eq!(ring_pointer.next(), Some(&1));
    }
}

// TODO Aula 7 acabou aqui

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

        #[allow(unused)]
        fn do_something_u8(x: u8) {
            x.method();
        }

        #[allow(unused)]
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

    /// # Por que _trait objects_ usando indireção (& ou Box<T>)?
    /// É necessário utilizar uma indireção (referências ou Box<T>), pois em rust todos os tipos
    /// devem possuir um tamanho conhecido em tempo de compilação. No exemplo acima, temos que o
    /// tipo do _trait object_ `Foo`, pode possuir o conteúdo de uma `String` (24 bytes), ou um
    /// `u8` (1 byte), ou qualquer outro tipo que implementa o trait `Foo`. Desta forma, para
    /// facilitar a utilização e diminuição de tamanho, é sempre utilizado uma indireção nos
    /// _trait objects_.
    fn do_something_dyn_box(x: Box<dyn Foo>) {
        x.method();
    }

    #[test]
    fn test_dyn_box() {
        let x = Box::new("Hello".to_string());
        // let x = Box::new(7u8);
        do_something_dyn_box(x);
    }
}

#[allow(unused)]
mod impl_vs_dyn {
    trait Foo {
        fn method(&self) -> String;
    }

    fn do_something_impl(foo: impl Foo) -> impl Foo {
        foo
    }

    fn do_something_dyn_ref(foo: &dyn Foo) -> &dyn Foo {
        foo
    }

    fn do_something_dyn_box(foo: Box<dyn Foo>) -> Box<dyn Foo> {
        foo
    }

    struct OnlyDynTrait {
        foo: Vec<Box<dyn Foo>>,
    }

    struct StaticDispatch<T: Foo> {
        foo: T,
    }
}

mod struct_state_with_traits {
    pub trait State {
        fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
        fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State>;
    }

    pub struct StoppedState;

    pub struct PausedState;

    pub struct PlayingState;

    impl State for StoppedState {
        fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            println!("{} | Stopped -> Playing", player.music());
            Box::new(PlayingState)
        }

        fn stop(self: Box<Self>, _player: &mut Player) -> Box<dyn State> {
            println!("Already stopped");
            self
        }
    }

    impl State for PausedState {
        fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            println!("{} | Paused -> Playing", player.music());
            Box::new(PlayingState)
        }

        fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            println!("{} | Paused -> Stopped", player.music());
            Box::new(StoppedState)
        }
    }

    impl State for PlayingState {
        fn play(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            println!("{} | Playing -> Paused", player.music());
            Box::new(PausedState)
        }

        fn stop(self: Box<Self>, player: &mut Player) -> Box<dyn State> {
            println!("{} | Playing -> Stopped", player.music());
            Box::new(StoppedState)
        }
    }

    pub struct Player {
        music: String,
    }

    impl Player {
        pub fn new(music: &str) -> Self {
            Self {
                music: music.to_string(),
            }
        }

        pub fn music(&self) -> &str {
            &self.music
        }
    }

    #[test]
    fn test_player_and_state() {
        let mut player = Player::new("Track1.mp3");
        let state = Box::new(StoppedState {});
        let state = state.play(&mut player);
        let state = state.play(&mut player);
        let _state = state.play(&mut player);
    }
}

mod struct_context_with_traits {
    pub trait Interface {
        type Driver: Driver;

        fn open(self) -> Self::Driver;
    }

    pub trait Driver {
        fn write(&mut self, content: Vec<u8>);
        fn read(&self) -> Vec<u8>;
    }

    pub struct SpiIterface;

    impl Interface for SpiIterface {
        type Driver = SpiDriver;

        fn open(self) -> Self::Driver {
            println!("Opening SPI...");
            SpiDriver { content: vec![] }
        }
    }

    pub struct SpiDriver {
        content: Vec<u8>,
    }

    impl Driver for SpiDriver {
        fn write(&mut self, content: Vec<u8>) {
            println!("Writing {content:?} to SPI...");
            self.content.extend(content.iter());
        }

        fn read(&self) -> Vec<u8> {
            self.content.clone()
        }
    }

    #[test]
    fn test_spi() {
        let spi = SpiIterface {};
        let mut driver = spi.open();

        driver_test_routine(&mut driver);
    }

    fn driver_test_routine(driver: &mut impl Driver) {
        driver.write(vec![1, 2, 3]);
        let result = driver.read();

        assert_eq!(result, vec![1, 2, 3]);
    }
}

fn main() {
    println!("Hello, world!");
}
