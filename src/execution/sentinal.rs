//  This module holds the implementation of the execution sentinal, used for interrupting execution

use std::sync::{Arc, atomic::AtomicBool};

#[derive(Debug)]
pub struct ExecutionSentinal {
    stop_requested: Arc<AtomicBool>,
}

impl ExecutionSentinal {
    pub fn as_atomicbool(&self) -> Arc<AtomicBool> {
        self.stop_requested.clone()
    }

    pub fn clear_stop_requested(&mut self) {
        self.stop_requested.store(false, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn is_stop_requested(&self) -> bool {
        self.stop_requested.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn new() -> ExecutionSentinal {
        ExecutionSentinal { stop_requested: Arc::new(AtomicBool::new(false)) }
    }

}

