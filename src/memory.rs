use std::time::Duration;

use crossterm::event::{self, Event};

// Constants
pub const MEMORY_MAX: usize = 1 << 16;
const MR_KBSR: usize = 0xFE00; // keyboard status
const MR_KBDR: usize = 0xFE02; // keyboard data
pub struct Memory {
    mem: [u16; MEMORY_MAX],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            mem: [0; MEMORY_MAX],
        }
    }

    pub fn get(&mut self, addr: usize) -> u16 {
        if addr == MR_KBSR {
            if let Ok(true) = event::poll(Duration::from_millis(10)) {
                let char_pressed = if let Ok(Event::Key(key_event)) = event::read() {
                    key_event.code.as_char()
                } else {
                    None
                };
                if let Some(c) = char_pressed {
                    self.mem[MR_KBSR] = 0xFFFF;
                    self.mem[MR_KBDR] = c as u16;
                }
            } else {
                self.mem[MR_KBSR] = 0;
            }
        };
        self.mem[addr]
    }

    pub fn set(&mut self, addr: usize, value: u16) {
        self.mem[addr] = value;
    }

    #[cfg(test)]
    pub fn get_range(&self, range: std::ops::Range<usize>) -> &[u16] {
        &self.mem[range]
    }
}
