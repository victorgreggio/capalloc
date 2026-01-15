use crate::domain::RiskCalculationResult;
use std::collections::HashSet;
use std::time::Duration;

pub struct AppState {
    pub results: Vec<RiskCalculationResult>,
    pub display_order: Vec<usize>, // Indices into results for display order
    pub total_time: Duration,
    pub selected: usize, // Index into display_order
    pub expanded: bool,
    pub optimization_budget: Option<f64>,
    // Three strategy results
    pub risk_selected: HashSet<String>, // Asset_ID (Alternative_ID)
    pub priority_selected: HashSet<String>,
    pub combined_selected: HashSet<String>,
}

impl AppState {
    pub fn new(results: Vec<RiskCalculationResult>, total_time: Duration) -> Self {
        let display_order: Vec<usize> = (0..results.len()).collect();
        Self {
            results,
            display_order,
            total_time,
            selected: 0,
            expanded: false,
            optimization_budget: None,
            risk_selected: HashSet::new(),
            priority_selected: HashSet::new(),
            combined_selected: HashSet::new(),
        }
    }

    pub fn with_optimization(
        results: Vec<RiskCalculationResult>,
        total_time: Duration,
        risk_alternatives: Vec<String>,
        priority_alternatives: Vec<String>,
        combined_alternatives: Vec<String>,
        budget: f64,
    ) -> Self {
        let risk_set: HashSet<String> = risk_alternatives.into_iter().collect();
        let priority_set: HashSet<String> = priority_alternatives.into_iter().collect();
        let combined_set: HashSet<String> = combined_alternatives.into_iter().collect();

        // Sort display order: alternatives selected by any strategy first
        let mut display_order: Vec<usize> = (0..results.len()).collect();
        display_order.sort_by(|&a, &b| {
            let key_a = format!(
                "{} ({})",
                results[a].asset.asset_id, results[a].asset.alternative_id
            );
            let key_b = format!(
                "{} ({})",
                results[b].asset.asset_id, results[b].asset.alternative_id
            );
            let a_selected = risk_set.contains(&key_a)
                || priority_set.contains(&key_a)
                || combined_set.contains(&key_a);
            let b_selected = risk_set.contains(&key_b)
                || priority_set.contains(&key_b)
                || combined_set.contains(&key_b);

            match (a_selected, b_selected) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(&b),
            }
        });

        Self {
            results,
            display_order,
            total_time,
            selected: 0,
            expanded: false,
            optimization_budget: Some(budget),
            risk_selected: risk_set,
            priority_selected: priority_set,
            combined_selected: combined_set,
        }
    }

    pub fn is_selected_by_risk(&self, result: &RiskCalculationResult) -> bool {
        let key = format!(
            "{} ({})",
            result.asset.asset_id, result.asset.alternative_id
        );
        self.risk_selected.contains(&key)
    }

    pub fn is_selected_by_priority(&self, result: &RiskCalculationResult) -> bool {
        let key = format!(
            "{} ({})",
            result.asset.asset_id, result.asset.alternative_id
        );
        self.priority_selected.contains(&key)
    }

    pub fn is_selected_by_combined(&self, result: &RiskCalculationResult) -> bool {
        let key = format!(
            "{} ({})",
            result.asset.asset_id, result.asset.alternative_id
        );
        self.combined_selected.contains(&key)
    }

    #[allow(dead_code)]
    pub fn is_selected_by_any(&self, result: &RiskCalculationResult) -> bool {
        self.is_selected_by_risk(result)
            || self.is_selected_by_priority(result)
            || self.is_selected_by_combined(result)
    }

    pub fn select_next(&mut self) {
        if self.selected < self.display_order.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn page_down(&mut self, page_size: usize) {
        self.selected = (self.selected + page_size).min(self.display_order.len().saturating_sub(1));
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.selected = self.selected.saturating_sub(page_size);
    }

    pub fn toggle_expand(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn get_selected(&self) -> Option<&RiskCalculationResult> {
        self.display_order
            .get(self.selected)
            .and_then(|&idx| self.results.get(idx))
    }
}
