use serde::Deserialize;

/// Domain model representing a capital investment alternative
#[derive(Debug, Clone, Deserialize)]
pub struct Asset {
    #[serde(rename = "Asset_ID")]
    pub asset_id: String,
    #[serde(rename = "Alternative_ID")]
    pub alternative_id: String,
    #[serde(rename = "Cost_USD")]
    pub cost_usd: f64,
    #[serde(rename = "PoF_Post_Action")]
    pub pof_post_action: f64,
    #[serde(rename = "CoF_Total_USD")]
    pub cof_total_usd: f64,
    #[serde(rename = "Safety_Risk_Level")]
    pub safety_risk_level: String,
}

impl Asset {
    pub fn is_high_risk(&self) -> bool {
        self.safety_risk_level == "High" || self.safety_risk_level == "Critical"
    }

    pub fn is_critical(&self) -> bool {
        self.safety_risk_level == "Critical"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_asset() -> Asset {
        Asset {
            asset_id: "IT_SYSTEM_001".to_string(),
            alternative_id: "Pilot_Program".to_string(),
            cost_usd: 45000.0,
            pof_post_action: 0.05,
            cof_total_usd: 500000.0,
            safety_risk_level: "Low".to_string(),
        }
    }

    #[test]
    fn test_is_high_risk_false() {
        let asset = create_test_asset();
        assert!(!asset.is_high_risk());
    }

    #[test]
    fn test_is_high_risk_true() {
        let mut asset = create_test_asset();
        asset.safety_risk_level = "High".to_string();
        assert!(asset.is_high_risk());
    }

    #[test]
    fn test_is_critical() {
        let mut asset = create_test_asset();
        asset.safety_risk_level = "Critical".to_string();
        assert!(asset.is_critical());
    }

    #[test]
    fn test_optimization_result_creation() {
        let asset = create_test_asset();
        let result = RiskCalculationResult::new(
            asset.clone(),
            25000.0,
            250000.0,
            225000.0,
            0.9,
            5.5,
            0.75,
            85.0,
            24.0,
            0.5,
        );

        assert_eq!(result.baseline_risk, 25000.0);
        assert_eq!(result.post_action_risk, 250000.0);
        assert_eq!(result.risk_reduction, 225000.0);
        assert_eq!(result.roi, 0.9);
        assert_eq!(result.criticality_score, 5.5);
        assert_eq!(result.priority_score, 0.75);
        assert_eq!(result.calculation_time_ms, 0.5);
    }
}

/// Value object representing the optimization result for a capital investment alternative
#[derive(Debug, Clone)]
pub struct RiskCalculationResult {
    pub asset: Asset,
    pub baseline_risk: f64,
    pub post_action_risk: f64,
    pub risk_reduction: f64,
    pub roi: f64,
    pub criticality_score: f64,
    pub priority_score: f64,
    pub cost_effectiveness: f64,
    pub payback_period: f64,
    pub calculation_time_ms: f64,
}

impl RiskCalculationResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        asset: Asset,
        baseline_risk: f64,
        post_action_risk: f64,
        risk_reduction: f64,
        roi: f64,
        criticality_score: f64,
        priority_score: f64,
        cost_effectiveness: f64,
        payback_period: f64,
        calculation_time_ms: f64,
    ) -> Self {
        Self {
            asset,
            baseline_risk,
            post_action_risk,
            risk_reduction,
            roi,
            criticality_score,
            priority_score,
            cost_effectiveness,
            payback_period,
            calculation_time_ms,
        }
    }

    #[allow(dead_code)]
    pub fn cost_benefit_ratio(&self) -> f64 {
        if self.asset.cost_usd > 0.0 {
            self.risk_reduction / self.asset.cost_usd
        } else {
            0.0
        }
    }
}
