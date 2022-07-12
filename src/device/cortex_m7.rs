use core::arch::asm;
use crate::device::ContextSwitch;

pub struct CortexM7 {
}

impl CortexM7 {
    pub fn new() -> CortexM7 {
        CortexM7 {}
    }
}

// TODO: Go through this code
impl ContextSwitch for CortexM7 {
    fn save_registers() {
        // Does not need to be in a critical section (?)
        let addr = 0x11111111;
        let p_tcb: *const u32 = &addr;
        unsafe {asm!(
        "mrs r0, psp",
        "isb",
        "ldr r2, [r3]",
        "tst r14, #0x10",
        "it eq",
        "vstmdbeq r0!, {{s16-s31}}",
        "stmdb r0!, {{r4-r11, r14}}",   // Store core registers
        "str r0, [r2]",
        "stmdb sp!, {{r0, r3}}",
        "mov r0, #0",
        "msr basepri, r0",
        "dsb",
        "isb",
        //"bl vTaskSwitchContext",
        "mov r0, #0",
        "msr basepri, r0",
        "ldmia sp!, {{r0, r3}}",
        "ldr r1, [r3]",
        "ldr r0, [r1]",
        "ldmia r0!, {{r4-r11, r14}}",
        "tst r14, #0x10",
        "it eq",
        "vldmiaeq r0!, {{s16-s31}}",
        "msr psp, r0",
        "isb",
        "bx r14",
        in("r3") p_tcb,
        )};

    }

    fn restore_registers() {
        todo!()
    }
}