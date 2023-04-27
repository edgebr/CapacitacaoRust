#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

#[rtic::app(device = stm32f4xx_hal::pac, dispatchers = [USART1])]
mod app {
    #[allow(unused_imports)]
    use rtic::mutex_prelude::TupleExt03;
    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::gpio::Edge;
    use stm32f4xx_hal::{
        gpio::{gpiob::PB0, gpioc::PC8, Input, Output, PushPull},
        pac,
        prelude::*,
        timer::MonoTimerUs,
    };

    // Shared resources go here
    #[shared]
    struct Shared {
        counter: u8,
        s1: u32,
        s2: u32,
        s3: u32,
        key: u8,
    }

    // Local resources go here
    #[local]
    struct Local {
        button: PC8<Input>,
        led: PB0<Output<PushPull>>,
    }

    #[monotonic(binds = TIM2, default = true)]
    type MicrosecMono = MonoTimerUs<pac::TIM2>;

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        let mut syscfg = ctx.device.SYSCFG.constrain();
        // clocks
        let rcc = ctx.device.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
        // gpio ports B and C
        let gpiob = ctx.device.GPIOB.split();
        let gpioc = ctx.device.GPIOC.split();
        // button
        let mut button = gpioc.pc8.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut ctx.device.EXTI);
        button.trigger_on_edge(&mut ctx.device.EXTI, Edge::Falling);
        // led
        let led = gpiob.pb0.into_push_pull_output();

        let mono = ctx.device.TIM2.monotonic_us(&clocks);
        display::spawn("Matheus").ok();
        (
            Shared {
                counter: 0u8,
                s1: 10,
                s2: 20,
                s3: 30,
                key: 7,
            },
            Local { button, led },
            init::Monotonics(mono),
        )
    }

    // Optional idle, can be removed if not needed.
    #[idle(shared = [s1, s2, s3])] // Prio 0
    fn idle(ctx: idle::Context) -> ! {
        rprintln!("Start of idle!");
        let s1 = ctx.shared.s1;
        let s2 = ctx.shared.s2;
        let s3 = ctx.shared.s3;

        (s1, s2, s3).lock(|s1, s2, s3| {
            rprintln!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
        });

        loop {
            continue;
        }
    }

    #[task(shared = [counter, &key], priority = 2)]
    fn display(mut ctx: display::Context, name: &'static str) {
        rprintln!("Hello, {}", name);
        let counter = ctx.shared.counter.lock(|counter| *counter);
        let key: &u8 = ctx.shared.key;
        rprintln!("Counter: {}, key: {}", counter, key);
        display::spawn_after(1.secs(), "Rodrigo").ok();
    }

    #[task(binds = EXTI9_5, shared = [counter], local = [button, led, a: u8 = 0], priority = 3)]
    fn button_click(mut ctx: button_click::Context) {
        ctx.local.button.clear_interrupt_pending_bit();
        ctx.local.led.toggle();
        ctx.shared.counter.lock(|counter| {
            *counter += 1;
        });
    }
}

//Task Priority
//   ┌────────────────────────────────────────────────────────┐
//   │                                                        │
//   │                                                        │
// 3 │                      Preempts                          │
// 2 │                    A─────────►                         │
// 1 │          B─────────► - - - - B────────►                │
// 0 │Idle┌─────►                   Resumes  ┌──────────►     │
//   ├────┴──────────────────────────────────┴────────────────┤
//   │                                                        │
//   └────────────────────────────────────────────────────────┘Time
