//  This module holds code to capture the current stack pointer. Credit to 
//  "Lambda Fairy" of StackOverflow, dated Aug 15, 2016

use std::cell::Cell;
use std::usize;

// This global variable tracks the highest point of the stack
thread_local!(static STACK_END: Cell<usize> = Cell::new(usize::MAX));

#[macro_export]
macro_rules! stack_ptr {
    () => ({
        // Grab a copy of the stack pointer
        let x: usize;
        unsafe {
            asm!("mov {},rsp",
                 out(reg) x,);
        }
        x
    })
}

/// Saves the current position of the stack. Any function
/// being profiled must call this macro.
#[macro_export]
macro_rules! tick {
    () => ({
        // Save the current stack pointer in STACK_END
        let stack_end = stack_ptr!();
        STACK_END.with(|c| {
            // Since the stack grows down, the "tallest"
            // stack must have the least pointer value
            let best = cmp::min(c.get(), stack_end);
            c.set(best);
        });
    })
}