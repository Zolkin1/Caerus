use crate::scheduler::Sys;

// TODO: Allow the tasks to take a generic input

pub struct Task<STATE> {
    // Hold Task Control Block
    tcb: TaskControlBlock,

    // Holds current state (consider making this a type defined state)
    state: STATE,

    // Holds the function pointer that is the task function
    task_fn: fn () -> usize,

    // Priority
    prio: u8,
}

pub struct NotInit;
pub struct Running;
pub struct Ready;
pub struct Suspended;

//#[derive(Clone)]
//#[derive(Copy)]
struct TaskControlBlock {
    tid: usize,
    sp: usize,
}

pub fn new<S, D>(s: &mut Sys<S, D>, a_fcn: fn() -> usize, a_prio: u8) -> Task<NotInit> {
    Task {
        state: NotInit,
        tcb: TaskControlBlock {
            tid: s.get_next_tid(),
            sp: s.get_next_sp(),
        },
        task_fn: a_fcn,
        prio: a_prio,
    }
}

impl<STATE> Task<STATE> {
    pub fn into_running(self) -> Task<Running> {
        Task {
            state: Running,
            tcb: self.tcb,
            task_fn: self.task_fn,
            prio: self.prio,
        }
    }

    pub fn into_ready(self) -> Task<Ready> {        // TODO: Do i need any stack preparation?
        Task {
            state: Ready,
            tcb: self.tcb,
            task_fn: self.task_fn,
            prio: self.prio,
        }
    }

    pub fn into_suspended(self) -> Task<Suspended> {
        Task {
            state: Suspended,
            tcb: self.tcb,
            task_fn: self.task_fn,
            prio: self.prio,
        }
    }
}
/*
impl TaskControlBlock {
    pub fn new(a_tid: usize, a_sp: usize) -> TaskControlBlock {
        TaskControlBlock {
            tid: a_tid,
            sp: a_sp,
        }
    }
}*/
