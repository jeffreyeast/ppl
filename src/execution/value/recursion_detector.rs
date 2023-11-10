// This module holds code used to detect recursion and prevent in fmt::Display.

use std::cell::RefCell;

thread_local!{
    static CYCLE: Cycle = Cycle { pass: RefCell::new(1) };
}


#[derive(Debug, Clone)]
pub struct Cycle {
    pass:       RefCell<u32>,
}

impl Cycle {
    pub fn has_not_been_processed(&self) -> bool {
        CYCLE.with(|cycle| { 
            let pass = &mut *self.pass.borrow_mut();
            if *pass != *cycle.pass.borrow() {
                *pass = *cycle.pass.borrow();
                true
            } else {
                false
            }
        })
    }

    pub fn new() -> Cycle {
        CYCLE.with(|cycle| { Cycle { pass: RefCell::new(*cycle.pass.borrow() - 1) } } )
    }

    pub fn start() {
        CYCLE.with(|cycle| { *cycle.pass.borrow_mut() += 1; } )
    }
}