#![allow(unused_imports)]
#![no_std]
#![no_main]

use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use stm32h7xx_hal::{prelude::*};
use caerus::{scheduler, device};

use panic_halt as _;
use cortex_m::peripheral::syst::SystClkSource;

#[entry]
fn main() -> ! {
    unsafe{crate::device::cortex_m7::PendSV2()};

    let p = cortex_m::Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    // this is configured for the LM3S6965 which has a default CPU clock of 12 MHz
    syst.set_reload(12_000_000);
    syst.clear_current();
    syst.enable_counter();
    syst.enable_interrupt();

    //NVIC::unmask(14);
    unsafe{crate::device::cortex_m7::PendSV2()};


    let dev = device::cortex_m7::CortexM7::new();
    let system = scheduler::new(dev);
    let mut system = system.into_preemption();

    let t1 = scheduler::tasks::new(&mut system, task_1, scheduler::PRIORITY_0);
    let t2 = scheduler::tasks::new(&mut system, task_2, scheduler::PRIORITY_1);

    let task_list = [t1, t2];

   // context_switch();

    system.start_sched(task_list);

    loop {}

    // Should never get here
}

#[no_mangle]
pub fn context_switch() {}

pub fn task_1() -> usize {
    loop {

    }
}

pub fn task_2() -> usize {
    loop {

    }
}