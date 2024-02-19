//  This module is the root of the graphics subsystem for PPL

use std::{thread::{JoinHandle, self}, sync::{mpsc, Arc, Mutex}};

use crate::utility::Event;

use self::{actions::GraphicsRequest, worker_thread::command_loop};

mod draw;
mod gtk_context;
mod actions;
mod messaging;
mod worker_thread;



#[derive(Debug,PartialEq)]
pub enum GraphicsStates {
    Idle,
    Ardmode,
    Ttymode,
}

#[derive(Debug)]
pub struct GraphicsContext {
    pub thread_handle: Mutex<Option<JoinHandle<()>>>,
    pub main_thread_request_channel: Mutex<mpsc::Sender<Box<dyn GraphicsRequest + Send>>>,
    pub graphics_thread_request_channel: Mutex<mpsc::Receiver<Box<dyn GraphicsRequest + Send>>>,
    pub graphics_state: Mutex<GraphicsStates>,
    pub request_complete: Arc<Event>,
}

impl GraphicsContext {
    pub fn is_in_graphics_mode(&self) -> bool {
        *self.graphics_state.lock().unwrap() == GraphicsStates::Ardmode
    }

    pub fn new() -> Arc<GraphicsContext> {
        let (sender,receiver) = std::sync::mpsc::channel();
        let graphics_context = Arc::new(GraphicsContext { 
            thread_handle: Mutex::new(None),
            main_thread_request_channel: Mutex::new(sender), 
            graphics_thread_request_channel: Mutex::new(receiver),
            graphics_state: Mutex::new(GraphicsStates::Idle),
            request_complete: Event::new() });
        let graphics_context_clone = graphics_context.clone();
        *graphics_context_clone.thread_handle.lock().unwrap() = Some(thread::spawn(move || command_loop(graphics_context.clone())));
        graphics_context_clone
    }
}