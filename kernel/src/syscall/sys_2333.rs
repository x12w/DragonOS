use system_error::SystemError;
use crate::arch::interrupt::TrapFrame;
use crate::syscall::table::{FormattedSyscallParam, Syscall};
use alloc::vec::Vec;
use log::info;

// 1. 先定义一个标识符（常量名）
const SYS_CUSTOM_2333: usize = 2333;

pub struct Sys2333Handle;

impl Syscall for Sys2333Handle {
    fn num_args(&self) -> usize {
        0
    }

    fn handle(&self, _args: &[usize], _frame: &mut TrapFrame) -> Result<usize, SystemError> {
        info!("syscall 2333 called"); // 任务要求 [cite: 32]
        Ok(6666) // 任务返回要求 [cite: 32]
    }

    fn entry_format(&self, _args: &[usize]) -> Vec<FormattedSyscallParam> {
        vec![]
    }
}

// 2. 传入标识符常量，而不是直接传数字
syscall_table_macros::declare_syscall!(SYS_CUSTOM_2333, Sys2333Handle);
