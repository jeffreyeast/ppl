//  Closes the graphics application window

use gtk::prelude::GtkWindowExt;

use crate::graphics::gtk_context::GtkContext;

use super::GraphicsRequest;


pub struct Close {}

impl GraphicsRequest for Close {
    fn execute(&self, gtk_context: &mut GtkContext) -> Result<(),String> {
        gtk_context.main_window.as_ref().unwrap().close();
        Ok(())
    }

    fn execute2(&self) -> Result<(),String> {
        panic!("Not implemented");
    }
}

impl Close {
    pub fn new() -> Self {
        Close {}
    }
}