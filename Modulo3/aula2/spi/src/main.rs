//! embassy hello world
//!
//! This is an example of running the embassy executor with multiple tasks
//! concurrently.

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

mod lis2dh;

extern crate alloc;

use crate::lis2dh::Lis2dh;
use embassy_executor::Executor;
use embassy_executor::_export::StaticCell;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, embassy, gdma::*, peripherals::Peripherals, prelude::*, timer::TimerGroup,
    Rtc, IO,
};

macro_rules! singleton {
    ($val:expr) => {{
        type T = impl Sized;
        static STATIC_CELL: StaticCell<T> = StaticCell::new();
        let (x,) = STATIC_CELL.init(($val,));
        x
    }};
}

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

static EXECUTOR: StaticCell<Executor> = StaticCell::new();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;

    extern "C" {
        static mut _heap_start: u32;
    }

    unsafe {
        let heap_start = &_heap_start as *const _ as usize;
        ALLOCATOR.init(heap_start as *mut u8, HEAP_SIZE);
    }
}

#[embassy_executor::task]
async fn acc_task(lis2dh: &'static mut Lis2dh<'static>) {
    let who_i_am = lis2dh.who_i_am().await;
    println!("Who I am: {}", who_i_am);

    loop {
        let (x, y, z) = lis2dh.read_all().await;
        println!("Acc: <{:?}, {}, {}>", x, y, z);
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[entry]
fn main() -> ! {
    init_heap();
    println!("Init!");
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    // Disable watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    embassy::init(
        &clocks,
        hal::systimer::SystemTimer::new(peripherals.SYSTIMER),
    );

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let dma = Gdma::new(peripherals.DMA, &mut system.peripheral_clock_control);
    let tx_descriptors = singleton!([0; 8 * 3]);
    let rx_descriptors = singleton!([0; 8 * 3]);
    let lis2dh = singleton!(Lis2dh::new(
        io,
        dma,
        peripherals.SPI2,
        &mut system.peripheral_clock_control,
        &clocks,
        100,
        tx_descriptors,
        rx_descriptors,
    ));

    let executor = EXECUTOR.init(Executor::new());
    executor.run(|spawner| {
        // spawner.spawn(spi_task(spi)).ok();
        spawner.spawn(acc_task(lis2dh)).ok();
    })
}
