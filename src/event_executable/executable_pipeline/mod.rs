use std::collections::VecDeque;

pub struct ExecutablePipeline {
    executables: VecDeque<Option<String>>,
    current_index: usize,
}

impl ExecutablePipeline {
    pub fn pop_front(&mut self) -> Option<&Option<String>> {
        let next = self.executables.get(self.current_index);
        self.current_index += 1;

        next
    }
}