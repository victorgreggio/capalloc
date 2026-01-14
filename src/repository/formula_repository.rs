use formcalc::Formula;
use std::error::Error;

/// Repository abstraction for loading capital allocation formulas
pub trait FormulaRepository: Send + Sync {
    fn load_all(&self) -> Result<Vec<Formula>, Box<dyn Error>>;
}

/// In-memory formula repository that loads formulas as if from a data source
pub struct InMemoryFormulaRepository;

impl InMemoryFormulaRepository {
    pub fn new() -> Self {
        Self
    }
}

impl Default for InMemoryFormulaRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl FormulaRepository for InMemoryFormulaRepository {
    fn load_all(&self) -> Result<Vec<Formula>, Box<dyn Error>> {
        Ok(vec![
            self.load_baseline_risk_formula(),
            self.load_safety_multiplier_formula(),
            self.load_criticality_score_formula(),
            self.load_degradation_factor_formula(),
            self.load_post_action_risk_formula(),
            self.load_risk_reduction_formula(),
            self.load_implementation_complexity_formula(),
            self.load_time_value_adjustment_formula(),
            self.load_adjusted_cost_formula(),
            self.load_roi_formula(),
            self.load_cost_effectiveness_formula(),
            self.load_priority_score_formula(),
            self.load_payback_period_formula(),
        ])
    }
}

impl InMemoryFormulaRepository {
    fn load_baseline_risk_formula(&self) -> Formula {
        Formula::new(
            "baseline_risk",
            r#"
            // Baseline risk assumes worst-case scenario (PoF = 1.0 for do-nothing)
            // Uses exponential scaling for high consequence assets
            if (cof_total > 1000000) then
                return rnd(exp(0.5) * cof_total, 2)
            else
                return rnd(1.0 * cof_total, 2)
            end
        "#,
        )
    }

    fn load_safety_multiplier_formula(&self) -> Formula {
        Formula::new(
            "safety_multiplier",
            r#"
            // Critical assets have higher consequence weight
            // Uses nested conditionals and arithmetic combinations
            if (is_critical) then
                return 1.5 + (pof_post_action * 0.2)
            else if (is_high_risk) then
                return 1.25 + max(0, pof_post_action - 0.1) * 0.15
            else
                return 1.0
            end
        "#,
        )
    }

    fn load_criticality_score_formula(&self) -> Formula {
        Formula::new(
            "criticality_score",
            r#"
            // Calculate asset criticality using multiple factors
            // Combines PoF, CoF, and safety classification
            if (is_critical) then
                return rnd(((pof_post_action * 10) + (cof_total / 500000)) * 1.5, 2)
            else if (is_high_risk) then
                return rnd(((pof_post_action * 10) + (cof_total / 500000)) * 1.25, 2)
            else
                return rnd((pof_post_action * 10) + (cof_total / 500000), 2)
            end
        "#,
        )
    }

    fn load_degradation_factor_formula(&self) -> Formula {
        Formula::new(
            "degradation_factor",
            r#"
            // Models asset degradation over time
            // Higher PoF indicates more degradation
            return rnd(1.0 - min(pof_post_action * 2, 0.95), 4)
        "#,
        )
    }

    fn load_post_action_risk_formula(&self) -> Formula {
        Formula::new(
            "post_action_risk",
            r#"
            // Risk = Probability × Consequence × Safety Multiplier × Degradation
            // Apply degradation factor (lower degradation = lower risk)
            return rnd(pof_post_action * cof_total * get_output_from('safety_multiplier') * max(get_output_from('degradation_factor'), 0.5), 2)
        "#,
        )
    }

    fn load_risk_reduction_formula(&self) -> Formula {
        Formula::new(
            "risk_reduction",
            r#"
            // Risk reduction = baseline risk - post action risk
            // Ensure non-negative reduction
            return rnd(max(get_output_from('baseline_risk') - get_output_from('post_action_risk'), 0), 2)
        "#,
        )
    }

    fn load_implementation_complexity_formula(&self) -> Formula {
        Formula::new(
            "implementation_complexity",
            r#"
            // Calculate implementation difficulty based on cost and asset type
            // Higher cost and critical assets = higher complexity
            if (is_critical) then
                return rnd(min((cost / 100000) * 2.0, 10), 2)
            else if (is_high_risk) then
                return rnd(min((cost / 100000) * 1.5, 10), 2)
            else
                return rnd(min(cost / 100000, 10), 2)
            end
        "#,
        )
    }

    fn load_time_value_adjustment_formula(&self) -> Formula {
        Formula::new(
            "time_value_adjustment",
            r#"
            // Discount factor for time value of money
            // More complex projects take longer, reducing present value
            // Present value discount: 1 / (1 + r)^n
            return rnd(1.0 / (1.0 + 0.006666667) ^ ceil(get_output_from('implementation_complexity') * 2), 4)
        "#,
        )
    }

    fn load_adjusted_cost_formula(&self) -> Formula {
        Formula::new(
            "adjusted_cost",
            r#"
            // Adjust cost for time value and complexity
            return rnd(cost * (1 + get_output_from('implementation_complexity') * 0.05) * get_output_from('time_value_adjustment'), 2)
        "#,
        )
    }

    fn load_roi_formula(&self) -> Formula {
        Formula::new(
            "roi",
            r#"
            // ROI = Risk Reduction / Adjusted Cost
            if (get_output_from('adjusted_cost') > 0) then
                return rnd(get_output_from('risk_reduction') / get_output_from('adjusted_cost'), 4)
            else
                return 999.9999
            end
        "#,
        )
    }

    fn load_cost_effectiveness_formula(&self) -> Formula {
        Formula::new(
            "cost_effectiveness",
            r#"
            // Normalized cost effectiveness score (0-100)
            // Considers both ROI and criticality
            // Weighted combination: 70% ROI, 30% criticality
            return rnd(min((min(get_output_from('roi'), 20) * 3.5) + (min(get_output_from('criticality_score'), 10) * 3), 100), 2)
        "#,
        )
    }

    fn load_priority_score_formula(&self) -> Formula {
        Formula::new(
            "priority_score",
            r#"
            // Overall priority score combining multiple factors
            // Uses weighted formula with exponential scaling for critical assets
            if (is_critical) then
                return rnd(((get_output_from('risk_reduction') / 1000000) * 0.4 + (min(get_output_from('roi'), 10) / 10) * 0.35 + (get_output_from('criticality_score') / 10) * 0.25) * 1.3, 4)
            else
                return rnd((get_output_from('risk_reduction') / 1000000) * 0.4 + (min(get_output_from('roi'), 10) / 10) * 0.35 + (get_output_from('criticality_score') / 10) * 0.25, 4)
            end
        "#,
        )
    }

    fn load_payback_period_formula(&self) -> Formula {
        Formula::new(
            "payback_period",
            r#"
            // Estimated payback period in months
            // Based on risk reduction as annual savings
            if (get_output_from('risk_reduction') > 0) then
                return rnd((get_output_from('adjusted_cost') / get_output_from('risk_reduction')) * 12, 1)
            else
                return 999.9
            end
        "#,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use formcalc::FormulaT;

    #[test]
    fn test_load_all_formulas() {
        let repo = InMemoryFormulaRepository::new();
        let formulas = repo.load_all().unwrap();
        assert_eq!(formulas.len(), 13);
    }

    #[test]
    fn test_formula_names() {
        let repo = InMemoryFormulaRepository::new();
        let formulas = repo.load_all().unwrap();

        let names: Vec<&str> = formulas.iter().map(|f| f.name()).collect();
        assert!(names.contains(&"baseline_risk"));
        assert!(names.contains(&"post_action_risk"));
        assert!(names.contains(&"risk_reduction"));
        assert!(names.contains(&"roi"));
        assert!(names.contains(&"cost_effectiveness"));
        assert!(names.contains(&"safety_multiplier"));
        assert!(names.contains(&"criticality_score"));
        assert!(names.contains(&"priority_score"));
    }
}
