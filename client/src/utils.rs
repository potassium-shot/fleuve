use dioxus::prelude::*;

#[derive(Clone, Copy)]
pub struct CallbackSignal(Signal<u8>);

impl CallbackSignal {
    pub fn get(&self) {
        self.0();
    }

    pub fn trigger(&mut self) {
        let mut w = self.0.write();
        *w = w.wrapping_add(1);
    }
}

pub fn use_callback_signal() -> CallbackSignal {
    CallbackSignal(use_signal(|| 0))
}
