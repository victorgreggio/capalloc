use crate::domain::{Asset, OptimizationResult};
use crate::repository::{AssetRepository, FormulaRepository};
use crate::services::RiskCalculationService;
use rayon::prelude::*;
use std::error::Error;
use std::time::{Duration, Instant};

/// Application service orchestrating the capital allocation workflow
pub struct CapitalAllocationApp {
    repository: Box<dyn AssetRepository + Send + Sync>,
    calculator: RiskCalculationService,
}

impl CapitalAllocationApp {
    pub fn new(
        repository: Box<dyn AssetRepository + Send + Sync>,
        formula_repository: Box<dyn FormulaRepository>,
    ) -> Self {
        Self {
            repository,
            calculator: RiskCalculationService::new(formula_repository),
        }
    }

    /// Load assets from repository
    pub fn load_assets(&self) -> Result<Vec<Asset>, Box<dyn Error>> {
        self.repository.load_all()
    }

    /// Calculate risk metrics for all assets in parallel
    pub fn calculate_all_risks(
        &self,
        assets: Vec<Asset>,
    ) -> (Vec<OptimizationResult>, Duration) {
        let start = Instant::now();

        let results: Vec<OptimizationResult> = assets
            .par_iter()
            .filter_map(|asset| self.calculator.calculate(asset).ok())
            .collect();

        let duration = start.elapsed();
        (results, duration)
    }

    /// Calculate risk metrics for a single asset
    #[allow(dead_code)]
    pub fn calculate_risk(&self, asset: &Asset) -> Result<OptimizationResult, Box<dyn Error>> {
        self.calculator.calculate(asset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Asset;
    use crate::repository::InMemoryFormulaRepository;
    use std::error::Error;

    struct MockRepository {
        assets: Vec<Asset>,
    }

    impl MockRepository {
        fn new(assets: Vec<Asset>) -> Self {
            Self { assets }
        }
    }

    impl AssetRepository for MockRepository {
        fn load_all(&self) -> Result<Vec<Asset>, Box<dyn Error>> {
            Ok(self.assets.clone())
        }
    }

    fn create_test_asset(id: &str, alternative: &str, cost: f64) -> Asset {
        Asset {
            asset_id: id.to_string(),
            alternative_id: alternative.to_string(),
            cost_usd: cost,
            pof_post_action: 0.05,
            cof_total_usd: 500000.0,
            safety_risk_level: "Low".to_string(),
        }
    }

    fn create_app(assets: Vec<Asset>) -> CapitalAllocationApp {
        let repository = Box::new(MockRepository::new(assets));
        let formula_repository = Box::new(InMemoryFormulaRepository::new());
        CapitalAllocationApp::new(repository, formula_repository)
    }

    #[test]
    fn test_load_assets() {
        let assets = vec![
            create_test_asset("PUMP_001", "Refurbish", 45000.0),
            create_test_asset("PUMP_001", "Replace", 120000.0),
        ];
        let app = create_app(assets.clone());

        let loaded = app.load_assets().unwrap();
        assert_eq!(loaded.len(), 2);
    }

    #[test]
    fn test_calculate_risk_single() {
        let assets = vec![create_test_asset("PUMP_001", "Refurbish", 45000.0)];
        let app = create_app(assets.clone());

        let result = app.calculate_risk(&assets[0]).unwrap();
        assert!(result.risk_reduction > 0.0);
    }

    #[test]
    fn test_calculate_all_risks() {
        let assets = vec![
            create_test_asset("PUMP_001", "Refurbish", 45000.0),
            create_test_asset("VALVE_002", "Repair", 15000.0),
            create_test_asset("TANK_003", "Replace", 200000.0),
        ];
        let app = create_app(assets.clone());

        let (results, duration) = app.calculate_all_risks(assets);

        assert_eq!(results.len(), 3);
        assert!(duration.as_nanos() > 0);
    }

    #[test]
    fn test_parallel_calculation_performance() {
        let assets: Vec<Asset> = (1..=100)
            .map(|id| create_test_asset(&format!("ASSET_{:03}", id), "Optimize", 10000.0))
            .collect();

        let app = create_app(assets.clone());

        let (results, _duration) = app.calculate_all_risks(assets);
        assert_eq!(results.len(), 100);
    }
}
