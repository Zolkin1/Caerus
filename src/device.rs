pub mod cortex_m7;

pub trait ContextSwitch {
    fn save_registers();
    fn restore_registers();
}