#![allow(unused_imports)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32h7xx_hal::{prelude::*};
use caerus::{scheduler, device};

use panic_halt as _;

#[entry]
fn main() -> ! {
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