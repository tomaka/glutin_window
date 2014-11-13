#![deny(missing_docs)]

//! A Piston window back-end using the Glutin library.

extern crate glutin;
extern crate event;
extern crate input;
extern crate current;

use current::{ Get };
use event::window::{ PollEvent, ShouldClose, Size, SwapBuffers };
use input::InputEvent;

/// Wraps the window back-end.
pub struct GlutinWindow<'a> {
    /// The Glutin window.
    pub window: glutin::Window,
    /// Events received from window,
    /// stored temporarily to not loose them.
    events: glutin::PollEventsIterator<'a>,
}

impl<'a> GlutinWindow<'a> {
    /// Creates a new `GlutinWindow`.
    pub fn new(window: glutin::Window) -> GlutinWindow<'a> {
        let events = window.poll_events();
        GlutinWindow {
            window: window,
            events: events
        }
    }
}

impl<'a> Get<ShouldClose> for GlutinWindow<'a> {
    fn get(&self) -> ShouldClose {
        ShouldClose(self.window.is_closed())
    }
}

impl<'a> Get<Size> for GlutinWindow<'a> {
    fn get(&self) -> Size {
        let (w, h) = self.window.get_inner_size().unwrap();
        Size([w as u32, h as u32])
    }
}

impl<'a> SwapBuffers for GlutinWindow<'a> {
    fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }
}

fn map_input_event(ev: glutin::Event) -> Option<InputEvent> {
    match ev {
        _ => None
    }
}

impl<'a> PollEvent<InputEvent> for GlutinWindow<'a> {
    fn poll_event(&mut self) -> Option<InputEvent> {
        // Get next event.
        let next_event = self.events.next();
        // If there are no remaining events, get new ones.
        let next_event = match next_event {
            None => {
                self.events = self.window.poll_events();
                self.events.next()
            }
            x => x
        };
        // Map to input event structure.
        match next_event {
            None => None,
            Some(ev) => map_input_event(ev)
        }
    }
}
