use textplots::{Chart, Plot, Shape};
use std::collections::VecDeque;
use std::cmp;

pub struct StreamPlotter {
    pub history: VecDeque<f32>,
    pub history_max_len: usize,
}

impl StreamPlotter {
    pub fn add(&mut self, value: f32) {
        self.history.push_back(value);
        if self.history.len() > self.history_max_len {
            self.history.pop_front();
        }
        self.plot_history()
    }

    pub fn plot_history(&mut self) {
        println!("Plot history");
        let cur_offset = -(self.history.len() as f32);
        let max_offset = -(self.history_max_len as f32);
        let resolution = cmp::max(self.history_max_len * 3, 32) as u32;

        let data = &self.history
            .iter()
            .enumerate()
            .map(|(i, &x)| (i as f32 + cur_offset, x))
            .collect::<Vec<(f32, f32)>>()[..];
        Chart::new(resolution, 60, max_offset, 0.0)
            .lineplot(&Shape::Steps(data))
            .nice();
    }

    pub fn new() -> Self {
        StreamPlotter{
            history: VecDeque::new(),
            history_max_len: 50,
        }
    }
}

