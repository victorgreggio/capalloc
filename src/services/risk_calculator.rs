use crate::domain::{Asset, OptimizationResult};
use crate::repository::FormulaRepository;
use formcalc::{Engine, Value};
use std::error::Error;
use std::time::Instant;

/// Service responsible for calculating risk metrics and ROI
pub struct RiskCalculationService {
    formula_repository: Box<dyn FormulaRepository>,
}

impl RiskCalculationService {
    pub fn new(formula_repository: Box<dyn FormulaRepository>) -> Self {
        Self { formula_repository }
    }

    /// Calculate risk metrics for a single asset alternative
    pub fn calculate(&self, asset: &Asset) -> Result<OptimizationResult, Box<dyn Error>> {
        let start = Instant::now();

        let mut engine = Engine::new();
        self.set_variables(&mut engine, asset);

        let formulas = self.formula_repository.load_all()?;
        engine.execute(formulas)?;

        let result = self.extract_results(&engine)?;
        let calculation_time_ms = start.elapsed().as_secs_f64() * 1000.0;

        Ok(OptimizationResult::new(
            asset.clone(),
            result.baseline_risk,
            result.post_action_risk,
            result.risk_reduction,
            result.roi,
            result.criticality_score,
            result.priority_score,
            result.cost_effectiveness,
            result.payback_period,
            calculation_time_ms,
        ))
    }

    /// Set asset data as variables in the formula engine
    fn set_variables(&self, engine: &mut Engine, asset: &Asset) {
        engine.set_variable("cost".to_string(), Value::Number(asset.cost_usd));
        engine.set_variable(
            "pof_post_action".to_string(),
            Value::Number(asset.pof_post_action),
        );
        engine.set_variable(
            "cof_total".to_string(),
            Value::Number(asset.cof_total_usd),
        );
        engine.set_variable("is_critical".to_string(), Value::Bool(asset.is_critical()));
        engine.set_variable("is_high_risk".to_string(), Value::Bool(asset.is_high_risk()));
    }

    /// Extract calculation results from the engine
    fn extract_results(&self, engine: &Engine) -> Result<CalculationResults, Box<dyn Error>> {
        Ok(CalculationResults {
            baseline_risk: self.extract_number(engine, "baseline_risk")?,
            post_action_risk: self.extract_number(engine, "post_action_risk")?,
            risk_reduction: self.extract_number(engine, "risk_reduction")?,
            roi: self.extract_number(engine, "roi")?,
            criticality_score: self.extract_number(engine, "criticality_score")?,
            priority_score: self.extract_number(engine, "priority_score")?,
            cost_effectiveness: self.extract_number(engine, "cost_effectiveness")?,
            payback_period: self.extract_number(engine, "payback_period")?,
        })
    }

    fn extract_number(&self, engine: &Engine, name: &str) -> Result<f64, Box<dyn Error>> {
        match engine
            .get_result(name)
            .ok_or(format!("{} not found", name))?
        {
            Value::Number(n) => Ok(n),
            _ => Err(format!("{} is not a number", name).into()),
        }
    }
}

/// Internal struct for holding calculation results
struct CalculationResults {
    baseline_risk: f64,
    post_action_risk: f64,
    risk_reduction: f64,
    roi: f64,
    criticality_score: f64,
    priority_score: f64,
    cost_effectiveness: f64,
    payback_period: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::InMemoryFormulaRepository;

    fn create_test_asset() -> Asset {
        Asset {
            asset_id: "PUMP_001".to_string(),
            alternative_id: "Refurbish".to_string(),
            cost_usd: 45000.0,
            pof_post_action: 0.05,
            cof_total_usd: 500000.0,
            safety_risk_level: "Low".to_string(),
        }
    }

    #[test]
    fn test_calculate_risk_metrics() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = RiskCalculationService::new(formula_repo);
        let asset = create_test_asset();

        let result = service.calculate(&asset);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.baseline_risk > 0.0);
        assert!(result.post_action_risk > 0.0);
        assert!(result.risk_reduction > 0.0);
        assert!(result.calculation_time_ms >= 0.0);
    }

    #[test]
    fn test_risk_reduction_calculation() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = RiskCalculationService::new(formula_repo);
        let asset = create_test_asset();

        let result = service.calculate(&asset).unwrap();
        
        // Risk reduction should equal baseline - post action
        let expected = result.baseline_risk - result.post_action_risk;
        assert!((result.risk_reduction - expected).abs() < 0.01);
    }

    #[test]
    fn test_critical_asset_multiplier() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = RiskCalculationService::new(formula_repo);
        
        let mut normal_asset = create_test_asset();
        normal_asset.safety_risk_level = "Low".to_string();
        let normal_result = service.calculate(&normal_asset).unwrap();

        let mut critical_asset = create_test_asset();
        critical_asset.safety_risk_level = "Critical".to_string();
        let critical_result = service.calculate(&critical_asset).unwrap();

        // Critical assets should have higher post-action risk due to multiplier
        assert!(critical_result.post_action_risk > normal_result.post_action_risk);
    }

    #[test]
    fn test_roi_calculation() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = RiskCalculationService::new(formula_repo);
        let asset = create_test_asset();

        let result = service.calculate(&asset).unwrap();
        
        // ROI should be risk_reduction / adjusted_cost (which includes time value and complexity adjustments)
        // Just verify it's a positive number and reasonable
        assert!(result.roi > 0.0);
        assert!(result.roi < 1000.0); // Sanity check
    }

    #[test]
    fn test_low_pof_reduces_risk() {
        let formula_repo = Box::new(InMemoryFormulaRepository::new());
        let service = RiskCalculationService::new(formula_repo);

        let mut high_pof = create_test_asset();
        high_pof.pof_post_action = 0.25;
        let high_result = service.calculate(&high_pof).unwrap();

        let mut low_pof = create_test_asset();
        low_pof.pof_post_action = 0.01;
        let low_result = service.calculate(&low_pof).unwrap();

        assert!(low_result.post_action_risk < high_result.post_action_risk);
        assert!(low_result.risk_reduction > high_result.risk_reduction);
    }
}
