use core::arch::{asm, global_asm};
use cortex_m_rt::{exception};
use cortex_m::peripheral::scb;
use stm32h7xx_hal::pac::SCB;

// TODO: modify this assembly
global_asm! (r#"
    .globl PendSV2
    PendSV2:
    mrs r0, psp                      // psp was the stack pointer being used. Move it into r0
    isb                              // Flush instruction pipeline
    ldr r2, [r3]                     // r3 holds the pointer to the TCB. Load the first value of the current TCB into r2
    tst r14, #0x10                   // Checks if FPU was active prior to this
    it eq                            // If it was then execute the next instruction
    vstmdbeq r0!, {{s16-s31}}        // Push the FPU registers onto the stack
    stmdb r0!, {{r4-r11, r14}}       // Store core registers on the stack
    str r0, [r2]                     // Now update the first word in the current TCB with the location on the stack of the stored registers
    stmdb sp!, {{r0, r3}}            // Store the arguments for vTaskSwitchContext
    dsb                              // Data synchronization
    isb                              // Flush instruction pipeline
    mov r0, #0                       // Prepare to turn on all interrupts
    msr basepri, r0                  // Turn on all interrupts
    ldmia sp!, {{r0, r3}}            // Pop the argument registers off the stack
    ldr r1, [r3]                     // de-reference p_tcb again (but now p_tcb has been changed by the switch task function to point at the new TCB)
    ldr r0, [r1]                     // Store the first value (at the top of the stack) from the task we are switching into
    ldmia r0!, {{r4-r11, r14}}       // Now pop off all the registers from that location on the stack (the location where we previously stored the registers)
    tst r14, #0x10                   // Check if the FPU is active
    it eq                            // If it is run the next instruction
    vldmiaeq r0!, {{s16-s31}}        // Pop the FPU registers off
    msr psp, r0                      // Change the stack pointer to be where the new task is
    isb                              // Flush the instructions
    bx r14                           // Populate the link register with thread mode and branch to the new context
    .align 16
"#);

extern {
     pub fn PendSV2();
}

// TODO: Determine if this is necessary
pub struct CortexM7 {
}

impl CortexM7 {
    pub fn new() -> CortexM7 {
        CortexM7 {}
    }
}

// TODO: Determine how to build specific files depending on the target

/*#[allow(non_snake_case)]
#[exception]
//#[naked]
fn PendSV() {
    //let addr = 0x11111111;
    //let p_tcb: *const u32 = &addr;
    unsafe {asm!(
    "mrs r0, psp",                      // psp was the stack pointer being used. Move it into r0
    "isb",                              // Flush instruction pipeline
    "ldr r2, [r3]",                     // r3 holds the pointer to the TCB. Load the first value of the current TCB into r2
    "tst r14, #0x10",                   // Checks if FPU was active prior to this
    "it eq",                            // If it was then execute the next instruction
    "vstmdbeq r0!, {{s16-s31}}",        // Push the FPU registers onto the stack
    "stmdb r0!, {{r4-r11, r14}}",       // Store core registers on the stack
    "str r0, [r2]",                     // Now update the first word in the current TCB with the location on the stack of the stored registers
    "stmdb sp!, {{r0, r3}}",            // Store the arguments for vTaskSwitchContext
    //"mov r0, %0",                       // %0 is an input from extended ASM that has the interrupt priority for not running SysCalls
    //"msr basepri, r0",                  // Turn off all SysCall interrupts
    "dsb",                              // Data synchronization
    "isb",                              // Flush instruction pipeline
    //"bl vTaskSwitchContext",          // Call vTaskSwitchContext - this function changes the TCB from the current one to the one that is preempting it
    "mov r0, #0",                       // Prepare to turn on all interrupts
    "msr basepri, r0",                  // Turn on all interrupts
    "ldmia sp!, {{r0, r3}}",            // Pop the argument registers off the stack
    "ldr r1, [r3]",                     // de-reference p_tcb again (but now p_tcb has been changed by the switch task function to point at the new TCB)
    "ldr r0, [r1]",                     // Store the first value (at the top of the stack) from the task we are switching into
    "ldmia r0!, {{r4-r11, r14}}",       // Now pop off all the registers from that location on the stack (the location where we previously stored the registers)
    "tst r14, #0x10",                   // Check if the FPU is active
    "it eq",                            // If it is run the next instruction
    "vldmiaeq r0!, {{s16-s31}}",        // Pop the FPU registers off
    "msr psp, r0",                      // Change the stack pointer to be where the new task is
    "isb",                              // Flush the instructions
    "bx r14",                           // Populate the link register with thread mode and branch to the new context
    ".align 4",
    options(noreturn),
    //in("r3") p_tcb,
    )};

}*/

#[allow(non_snake_case)]
#[exception]
fn SysTick() {
    static mut COUNT: u32 = 0;
    *COUNT += 1;

    if *COUNT == 10 {
        // Activate the PendSV handler
        SCB::set_pendsv();
    }
    unsafe{PendSV2()};

}

#[allow(non_snake_case)]
#[exception]
fn PendSV() {
    unsafe {PendSV2()};
}

