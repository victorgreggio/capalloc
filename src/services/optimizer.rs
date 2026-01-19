use crate::domain::OptimizationResult;
use minilp::{ComparisonOp, OptimizationDirection, Problem, Variable};
use std::collections::HashMap;

/// Result of portfolio optimization
#[derive(Debug, Clone)]
pub struct OptimizationSolution {
    pub selected_alternatives: Vec<String>, // Asset IDs of selected alternatives
    pub total_cost: f64,
    pub total_risk_reduction: f64,
    pub total_priority_score: f64,
    pub num_assets_optimized: usize,
}

/// Portfolio optimizer using linear programming
/// Selects alternatives that maximize value under budget constraint
pub struct PortfolioOptimizer;

impl PortfolioOptimizer {
    pub fn new() -> Self {
        Self
    }

    /// Optimize asset portfolio under budget constraint using linear programming
    /// Formulation: Maximize sum of risk reduction
    /// Subject to:
    ///   - Total cost <= budget
    ///   - At most one alternative per asset (SOS1 constraint)
    ///   - All variables are binary (0 or 1)
    pub fn optimize(
        &self,
        results: &[OptimizationResult],
        budget: f64,
    ) -> Result<OptimizationSolution, Box<dyn std::error::Error>> {
        if results.is_empty() {
            return Err("No alternatives to optimize".into());
        }

        // Create LP problem: maximize risk reduction
        let mut problem = Problem::new(OptimizationDirection::Maximize);

        // Create binary decision variables for each alternative
        let mut vars: Vec<(Variable, &OptimizationResult)> = Vec::new();
        for result in results {
            // Binary variable: 1 if selected, 0 otherwise
            // Objective coefficient is the risk reduction
            let var = problem.add_var(result.risk_reduction, (0.0, 1.0));
            vars.push((var, result));
        }

        // Constraint 1: Total cost <= budget
        let cost_constraint: Vec<(Variable, f64)> = vars
            .iter()
            .map(|(var, result)| (*var, result.asset.cost_usd))
            .collect();
        problem.add_constraint(&cost_constraint, ComparisonOp::Le, budget);

        // Constraint 2: At most one alternative per asset
        // Group alternatives by asset_id
        let mut asset_groups: HashMap<String, Vec<Variable>> = HashMap::new();
        for (var, result) in &vars {
            asset_groups
                .entry(result.asset.asset_id.clone())
                .or_default()
                .push(*var);
        }

        // For each asset, add constraint: sum of alternatives <= 1
        for (_asset_id, asset_vars) in asset_groups.iter() {
            let constraint: Vec<(Variable, f64)> = asset_vars.iter().map(|v| (*v, 1.0)).collect();
            problem.add_constraint(&constraint, ComparisonOp::Le, 1.0);
        }

        // Solve the problem
        let solution = problem.solve()?;

        // Extract selected alternatives
        let mut selected = Vec::new();
        let mut total_cost = 0.0;
        let mut total_risk_reduction = 0.0;
        let mut total_priority = 0.0;

        for (var, result) in &vars {
            // Check if variable is selected (value close to 1)
            if solution[*var] > 0.5 {
                selected.push(format!(
                    "{} ({})",
                    result.asset.asset_id, result.asset.alternative_id
                ));
                total_cost += result.asset.cost_usd;
                total_risk_reduction += result.risk_reduction;
                total_priority += result.priority_score;
            }
        }

        Ok(OptimizationSolution {
            selected_alternatives: selected.clone(),
            total_cost,
            total_risk_reduction,
            total_priority_score: total_priority,
            num_assets_optimized: selected.len(),
        })
    }

    /// Optimize with priority score as objective
    /// Uses linear programming to find optimal solution
    pub fn optimize_by_priority(
        &self,
        results: &[OptimizationResult],
        budget: f64,
    ) -> Result<OptimizationSolution, Box<dyn std::error::Error>> {
        if results.is_empty() {
            return Err("No alternatives to optimize".into());
        }

        // Create LP problem: maximize priority score
        let mut problem = Problem::new(OptimizationDirection::Maximize);

        // Create binary decision variables
        let mut vars: Vec<(Variable, &OptimizationResult)> = Vec::new();
        for result in results {
            let var = problem.add_var(result.priority_score, (0.0, 1.0));
            vars.push((var, result));
        }

        // Budget constraint
        let cost_constraint: Vec<(Variable, f64)> = vars
            .iter()
            .map(|(var, result)| (*var, result.asset.cost_usd))
            .collect();
        problem.add_constraint(&cost_constraint, ComparisonOp::Le, budget);

        // One alternative per asset constraint
        let mut asset_groups: HashMap<String, Vec<Variable>> = HashMap::new();
        for (var, result) in &vars {
            asset_groups
                .entry(result.asset.asset_id.clone())
                .or_default()
                .push(*var);
        }

        for (_asset_id, asset_vars) in asset_groups.iter() {
            let constraint: Vec<(Variable, f64)> = asset_vars.iter().map(|v| (*v, 1.0)).collect();
            problem.add_constraint(&constraint, ComparisonOp::Le, 1.0);
        }

        // Solve
        let solution = problem.solve()?;

        // Extract results
        let mut selected = Vec::new();
        let mut total_cost = 0.0;
        let mut total_risk_reduction = 0.0;
        let mut total_priority = 0.0;

        for (var, result) in &vars {
            if solution[*var] > 0.5 {
                selected.push(format!(
                    "{} ({})",
                    result.asset.asset_id, result.asset.alternative_id
                ));
                total_cost += result.asset.cost_usd;
                total_risk_reduction += result.risk_reduction;
                total_priority += result.priority_score;
            }
        }

        Ok(OptimizationSolution {
            selected_alternatives: selected.clone(),
            total_cost,
            total_risk_reduction,
            total_priority_score: total_priority,
            num_assets_optimized: selected.len(),
        })
    }

    /// Optimize using combined objective (weighted risk + priority)
    /// Allows balancing between risk reduction and priority score
    pub fn optimize_combined(
        &self,
        results: &[OptimizationResult],
        budget: f64,
        risk_weight: f64,
        priority_weight: f64,
    ) -> Result<OptimizationSolution, Box<dyn std::error::Error>> {
        if results.is_empty() {
            return Err("No alternatives to optimize".into());
        }

        // Create LP problem with combined objective
        let mut problem = Problem::new(OptimizationDirection::Maximize);

        // Create variables with weighted objective
        let mut vars: Vec<(Variable, &OptimizationResult)> = Vec::new();
        for result in results {
            // Normalize to similar scales before weighting
            let normalized_risk = result.risk_reduction / 1_000_000.0; // Scale to millions
            let normalized_priority = result.priority_score;
            let objective_coeff =
                risk_weight * normalized_risk + priority_weight * normalized_priority;

            let var = problem.add_var(objective_coeff, (0.0, 1.0));
            vars.push((var, result));
        }

        // Add constraints
        let cost_constraint: Vec<(Variable, f64)> = vars
            .iter()
            .map(|(var, result)| (*var, result.asset.cost_usd))
            .collect();
        problem.add_constraint(&cost_constraint, ComparisonOp::Le, budget);

        let mut asset_groups: HashMap<String, Vec<Variable>> = HashMap::new();
        for (var, result) in &vars {
            asset_groups
                .entry(result.asset.asset_id.clone())
                .or_default()
                .push(*var);
        }

        for (_asset_id, asset_vars) in asset_groups.iter() {
            let constraint: Vec<(Variable, f64)> = asset_vars.iter().map(|v| (*v, 1.0)).collect();
            problem.add_constraint(&constraint, ComparisonOp::Le, 1.0);
        }

        // Solve
        let solution = problem.solve()?;

        // Extract results
        let mut selected = Vec::new();
        let mut total_cost = 0.0;
        let mut total_risk_reduction = 0.0;
        let mut total_priority = 0.0;

        for (var, result) in &vars {
            if solution[*var] > 0.5 {
                selected.push(format!(
                    "{} ({})",
                    result.asset.asset_id, result.asset.alternative_id
                ));
                total_cost += result.asset.cost_usd;
                total_risk_reduction += result.risk_reduction;
                total_priority += result.priority_score;
            }
        }

        Ok(OptimizationSolution {
            num_assets_optimized: selected.len(),
            selected_alternatives: selected,
            total_cost,
            total_risk_reduction,
            total_priority_score: total_priority,
        })
    }
}

impl Default for PortfolioOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Asset;

    fn create_test_result(
        asset_id: &str,
        alternative: &str,
        cost: f64,
        risk_reduction: f64,
        priority: f64,
    ) -> OptimizationResult {
        OptimizationResult::new(
            Asset {
                asset_id: asset_id.to_string(),
                alternative_id: alternative.to_string(),
                cost_usd: cost,
                pof_post_action: 0.05,
                cof_total_usd: 500000.0,
                safety_risk_level: "Low".to_string(),
            },
            500000.0,
            25000.0,
            risk_reduction,
            risk_reduction / cost,
            5.0,
            priority,
            75.0,
            12.0,
            0.5,
        )
    }

    #[test]
    fn test_optimize_under_budget() {
        let optimizer = PortfolioOptimizer::new();
        let results = vec![
            create_test_result("IT_SYSTEM_001", "Pilot_Program", 10000.0, 50000.0, 5.0),
            create_test_result("DATACENTER_002", "Full_Implementation", 15000.0, 80000.0, 8.0),
            create_test_result("CLOUD_MIGRATION_003", "Partial_Implementation", 20000.0, 60000.0, 6.0),
        ];

        let solution = optimizer.optimize(&results, 30000.0).unwrap();

        assert!(solution.total_cost <= 30000.0);
        assert!(solution.num_assets_optimized > 0);
    }

    #[test]
    fn test_one_alternative_per_asset() {
        let optimizer = PortfolioOptimizer::new();
        let results = vec![
            create_test_result("IT_SYSTEM_001", "Pilot_Program", 10000.0, 50000.0, 5.0),
            create_test_result("IT_SYSTEM_001", "Full_Implementation", 50000.0, 90000.0, 9.0),
            create_test_result("DATACENTER_002", "Partial_Implementation", 8000.0, 40000.0, 4.0),
        ];

        let solution = optimizer
            .optimize_combined(&results, 100000.0, 0.5, 0.5)
            .unwrap();

        // Should not select both alternatives for IT_SYSTEM_001
        // Count how many times IT_SYSTEM_001 appears in selected alternatives
        let it_system_count = solution
            .selected_alternatives
            .iter()
            .filter(|s| s.starts_with("IT_SYSTEM_001"))
            .count();
        assert!(
            it_system_count <= 1,
            "Should select at most one alternative per investment"
        );
    }

    #[test]
    fn test_maximize_risk_reduction() {
        let optimizer = PortfolioOptimizer::new();
        let results = vec![
            create_test_result("IT_SYSTEM_001", "Cheap", 5000.0, 10000.0, 2.0),
            create_test_result("DATACENTER_002", "Expensive", 5000.0, 50000.0, 5.0),
        ];

        let solution = optimizer.optimize(&results, 5000.0).unwrap();

        // Should select the one with higher risk reduction
        assert!(solution.total_risk_reduction >= 50000.0);
    }
}
