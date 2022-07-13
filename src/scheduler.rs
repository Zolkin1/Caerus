pub mod tasks;
use crate::rtos_config::{NUM_TASKS, START_TASK_STACK_SPACE, TASK_STACK_SIZE_KB};

pub const PRIORITY_0: u8 = 0;
pub const PRIORITY_1: u8 = 0;
pub const PRIORITY_2: u8 = 0;
pub const PRIORITY_3: u8 = 0;

pub struct RoundRobin;
pub struct TimeSlice;
pub struct Preemption;

pub struct Sys <S, D> {
    // Scheduling type
    sched: S,

    // Time slice
    ts: Option<usize>,

    // Device
    device: D,

    // Holds the running task

    // Holds the next stack location

    // Intermediate data
    current_tid: usize,
    current_stack_location: usize,
}

pub fn new<D>(a_device: D) -> Sys<RoundRobin, D> {
    Sys {
        sched: RoundRobin,
        ts: None,
        device: a_device,
        current_tid: 0,
        current_stack_location: START_TASK_STACK_SPACE,
    }
}

impl<S, D> Sys<S, D>
{
    pub fn start_sched(&self, t: [tasks::Task<tasks::NotInit>; NUM_TASKS]) {
        // schedule the tasks
        // Sort the tasks by priority into a new structure
        // Set the tasks to ready
        // Get the highest priority task and join that function
        loop {}
    }
}
impl<S, D> Sys<S, D> {
    pub fn into_time_slice(self, a_ts: usize) -> Sys<TimeSlice, D> {
        Sys {
            sched: TimeSlice,
            ts: Some(a_ts),
            device: self.device,
            current_tid: self.current_tid,
            current_stack_location: self.current_stack_location,
        }
    }

    pub fn into_preemption(self) -> Sys<Preemption, D> {
        Sys {
            sched: Preemption,
            ts: None,
            device: self.device,
            current_tid: self.current_tid,
            current_stack_location: self.current_stack_location,
        }
    }

    fn get_next_tid(&mut self) -> usize {
        self.current_tid += 1;
        self.current_tid
    }

    fn get_next_sp(&mut self) -> usize {
        self.current_stack_location += TASK_STACK_SIZE_KB;
        self.current_stack_location
    }
}

impl <D> Sys<Preemption, D> {

}

impl <D> Sys<RoundRobin, D> {

}

impl<D> Sys<TimeSlice, D> {
    // Sort the tasks by priority into a new structure
    // Set the tasks to ready
    // Get the highest priority task and join that function
    // Start a timer - how to do this without polling the timer?
    // Prepare the stack
    // Join the function at that location in the stack

    // Later: Store the context
    // Jump to the next stack
}