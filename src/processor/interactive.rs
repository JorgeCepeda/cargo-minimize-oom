use winput::message_loop;
use winput::{Vk, Action};

use std::cell::LazyCell;
use winput::message_loop::EventReceiver;

thread_local! {
    static RECEIVER: LazyCell<EventReceiver> = LazyCell::new(|| {
        winput::message_loop::start().unwrap()
    });
}

/// To be run in a terminal, sends UpArrow + Enter
pub fn launch_repro() {
    winput::send(Vk::UpArrow);
    winput::send(Vk::Enter);
}

/// LeftArrow means false, RightArrow means true
pub fn repro_by_keystrokes() -> bool {
    RECEIVER.with(|r| r.clear());
    loop {
        let event = RECEIVER.with(|r| r.next_event());
        match event {
             message_loop::Event::Keyboard { vk, action, .. } => {
                 if vk == Vk::RightArrow && action == Action::Press {
                     return true;
                 } else if vk == Vk::LeftArrow && action == Action::Press {
                     return false;
                 }
            },
            _ => ()
        }
    }
}
