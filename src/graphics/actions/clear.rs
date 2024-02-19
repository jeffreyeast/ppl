//  Clears the graphics screen

use gtk::prelude::*;
use crate::graphics::gtk_context::GtkContext;

use super::GraphicsRequest;


pub struct Clear {}

impl GraphicsRequest for Clear {
    fn execute(&self, gtk_context: &mut GtkContext) -> Result<(),String> {
        gtk_context.window_contents.clear();
        gtk_context.drawing_area.as_ref().unwrap().queue_draw();
        Ok(())
    }

    fn execute2(&self) -> Result<(),String> {
        panic!("Not implemented");
    }
}

impl Clear {
    pub fn new() -> Self {
        Clear {}
    }
}