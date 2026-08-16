#![allow(unused_imports)]
use covopt_macro::covopt_param;
pub use sgl_macros::{
    sgl_cmd, sgl_combine_handlers, sgl_compile, sgl_hardware_call, sgl_package, sgl_syscall,
};
use std::io::Write;

/// Combine multiple SGL VM hardware or syscall handlers into a single combined handler closure.
#[macro_export]
macro_rules! sgl_combine_handlers_rules {
    ($($handler:path),* $(,)?) => {
        |vm: &mut $crate::sgl::vm::ScriptVm, a: usize, b: usize, c: usize| {
            let initial_dest = vm.registers[a % 256];
            $(
                $handler(vm, a, b, c);
                if vm.registers[a % 256] != initial_dest {
                    return;
                }
            )*
        }
    };
}
