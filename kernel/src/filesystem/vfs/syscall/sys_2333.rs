use system_error::SystemError;
use crate::arch::interrupt::TrapFrame;
use crate::syscall::table::{FormattedSyscallParam, Syscall};
use alloc::vec::Vec;

// 定义处理结构体 [cite: 15]
pub struct Sys2333Handle;

impl Syscall for Sys2333Handle {
    // 该系统调用不需要参数
    fn num_args(&self) -> usize {
        0
    }

    // 核心处理逻辑 [cite: 32, 37]
    fn handle(&self, _args: &[usize], _frame: &mut TrapFrame) -> Result<usize, SystemError> {
        // 在内核日志中打印信息
        log::info("syscall 2333 called");
        // 返回要求的值 6666
        Ok(6666)
    }

    fn entry_format(&self, _args: &[usize]) -> Vec<FormattedSyscallParam> {
        vec![]
    }
}

// 注册系统调用，手动指定调用号为 2333
syscall_table_macros::declare_syscall!(2333, Sys2333Handle);
