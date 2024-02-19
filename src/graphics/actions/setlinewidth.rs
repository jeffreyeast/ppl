//  Sets the desired line width, in pixels

use crate::graphics::gtk_context::GtkContext;

use super::GraphicsRequest;


pub struct SetLineWidth {
    width: i32,
}

impl GraphicsRequest for SetLineWidth {
    fn execute(&self, gtk_context: &mut GtkContext) -> Result<(),String> {
        gtk_context.line_width = self.width;
        Ok(())
    }

    fn execute2(&self) -> Result<(),String> {
        panic!("Not implemented");
    }
}

impl SetLineWidth {
    pub fn new(width: i32) -> Self {
        SetLineWidth { width: width }
    }
}