//  This module holds routines that interact between the main thread and the graphics thread

use super::{GraphicsContext, actions::{GraphicsRequest, clear::Clear, close::Close, setpoint::SetPoint, print::Print, vector::Vector, prepare::Prepare, setlinewidth::SetLineWidth}};



impl GraphicsContext {
    pub fn close(&self) -> Result<(),String> {
        self.send_request_to_worker(Box::new(Close::new()))
    }

    pub fn create_application(&self, window_width: i32, window_height: i32, title: &str) -> Result<(),String> {
        self.send_request_to_worker(Box::new(Prepare::new(window_width, window_height, title)))
    }

    pub fn clearscreen(&self) -> Result<(),String> {
        self.send_request_to_worker(Box::new(Clear::new()))
    }

    pub fn dottedvec(&self, dx: i32, dy: i32) -> Result<(),String> {
        self.send_request_to_worker(Box::new(Vector::as_dotted (dx, dy)))
    }

    pub fn print(&self, s: String) -> Result<(),String> {
        self.send_request_to_worker(Box::new(Print::new(s.as_str())))
    }

    fn send_request_to_worker(&self, request: Box<dyn GraphicsRequest + Send>) -> Result<(),String> {
        self.request_complete.reset();
        if self.main_thread_request_channel.lock().unwrap().send(request).is_ok() {
            self.request_complete.wait();
            Ok(())
        } else {
            Err(format!("window is closed"))
        }
    }

    pub fn setlinewidth(&self, width: i32) -> Result<(),String> {
        self.send_request_to_worker(Box::new(SetLineWidth::new(width)))
    }

    pub fn setpoint(&self, x: i32, y: i32) -> Result<(),String> {
        self.send_request_to_worker(Box::new(SetPoint::new(x, y)))
    }

    pub fn solidvec(&self, dx: i32, dy: i32) -> Result<(),String> {
        self.send_request_to_worker(Box::new(Vector::as_solid(dx, dy)))
    }

    pub fn worker_receive_request(&self) -> Result<Box<dyn GraphicsRequest + Send>,String> {
        self.graphics_thread_request_channel.lock().unwrap().recv().map_err(|e| e.to_string())
    }
}
