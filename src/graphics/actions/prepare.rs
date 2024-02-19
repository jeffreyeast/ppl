//  Creates the GTK application and applicaztion window objects

use crate::graphics::{worker_thread::create_application, gtk_context::GtkContext};

use super::GraphicsRequest;


pub struct Prepare {
    window_width: i32,
    window_height: i32,
    title: String,
}

impl GraphicsRequest for Prepare {
    fn execute(&self, _gtk_context: &mut GtkContext) -> Result<(),String> {
        panic!("Not implemented");
    }

    fn execute2(&self) -> Result<(),String> {
        create_application(self.window_width, self.window_height, self.title.as_str()).expect("Worker thread create application failed");
        Ok(())
    }
}

impl Prepare {
    pub fn new(window_width: i32, window_height: i32, title: &str) -> Self {
        Prepare { window_height: window_height, window_width: window_width, title: String::from(title) }
    }
}