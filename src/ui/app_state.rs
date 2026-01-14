use crate::domain::OptimizationResult;
use std::time::Duration;

pub struct AppState {
    pub results: Vec<OptimizationResult>,
    pub total_time: Duration,
    pub selected: usize,
    pub expanded: bool,
}

impl AppState {
    pub fn new(results: Vec<OptimizationResult>, total_time: Duration) -> Self {
        Self {
            results,
            total_time,
            selected: 0,
            expanded: false,
        }
    }

    pub fn select_next(&mut self) {
        if self.selected < self.results.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn page_down(&mut self, page_size: usize) {
        self.selected = (self.selected + page_size).min(self.results.len().saturating_sub(1));
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.selected = self.selected.saturating_sub(page_size);
    }

    pub fn toggle_expand(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn get_selected(&self) -> Option<&OptimizationResult> {
        self.results.get(self.selected)
    }
}
