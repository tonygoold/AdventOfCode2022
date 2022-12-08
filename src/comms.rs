use std::collections::VecDeque;

pub struct Datastream {
    stream: String,
}

impl Datastream {
    pub fn new(s: &str) -> Self {
        Self { stream: s.to_owned() }
    }

    pub fn start_packet_offset(&self) -> Option<usize> {
        let mut buffer = VecDeque::new();
        for (i, c) in self.stream.chars().enumerate() {
            buffer.push_back(c);
            if i < 4 {
                continue;
            }
            buffer.pop_front();
            if buffer[0] != buffer[1] && buffer[0] != buffer[2] && buffer[0] != buffer[3] &&
                buffer[1] != buffer[2] && buffer[1] != buffer[3] && buffer[2] != buffer[3] {
                return Some(i + 1);
            }
        }
        None
    }

    pub fn start_message_offset(&self) -> Option<usize> {
        let mut buffer = VecDeque::new();
        for (i, c) in self.stream.chars().enumerate() {
            buffer.push_back(c);
            if i < 14 {
                continue;
            }
            buffer.pop_front();
            let mut dup = false;
            for (i, c1) in buffer.iter().enumerate() {
                for c2 in buffer.iter().take(i) {
                    if c1 == c2 {
                        dup = true;
                        break;
                    }
                }
            }
            if !dup {
                return Some(i + 1);
            }
        }
        None
    }
}