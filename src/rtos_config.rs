use crate::scheduler;

/// Declare all the compile time config constants
pub const NUM_TASKS: usize = 2;
pub const TASK_STACK_SIZE_KB: usize = 100;
pub const START_TASK_STACK_SPACE: usize = 100;
//pub const SCHED_TYPE: u8 = scheduler::TIME_SLICE;