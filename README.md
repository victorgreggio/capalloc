# Capital Allocation Optimizer

A sophisticated risk-based capital allocation optimizer demonstrating the `formcalc` formula engine's advanced capabilities with a terminal user interface (TUI) built with `ratatui`.

## Features

- **Advanced Formula Engine**: 13 interdependent formulas using complex mathematical functions
- **Financial Modeling**: Time value of money, ROI, payback period calculations
- **Multi-Criteria Optimization**: Priority scoring with weighted factors
- **Parallel Processing**: All asset alternatives processed simultaneously using Rayon
- **Interactive TUI**: Browse assets and view detailed risk analysis
- **Real-world Metrics**: PoF, CoF, safety risk levels, criticality scoring

## Risk Calculation Methodology

The optimizer uses **13 interdependent formulas** showcasing advanced mathematical operations:

### Core Metrics

1. **Baseline Risk** - Worst-case scenario with exponential scaling
2. **Safety Multiplier** - Dynamic multiplier based on asset classification
3. **Criticality Score** - Multi-factor scoring combining PoF and CoF
4. **Degradation Factor** - Time-based asset degradation modeling
5. **Post-Action Risk** - Actual risk after alternative implementation
6. **Risk Reduction** - Dollar value of eliminated risk

### Financial Analysis

7. **Implementation Complexity** - Difficulty scoring based on cost and criticality
8. **Time Value Adjustment** - Present value discount calculation
9. **Adjusted Cost** - Cost with complexity premium and time value
10. **ROI** - Risk reduction per adjusted dollar spent
11. **Payback Period** - Estimated months to recover investment

### Decision Support

12. **Cost Effectiveness** - Normalized score (0-100) combining ROI and criticality
13. **Priority Score** - Weighted multi-criteria ranking for portfolio optimization

### Advanced Formula Features

The formulas demonstrate formcalc's capabilities:

- ✅ **Mathematical Functions**: `max()`, `min()`, `exp()`, `ceil()`, `rnd()`, `^` (power)
- ✅ **Complex Conditionals**: Nested if-else statements with dynamic multipliers
- ✅ **Formula Dependencies**: Multi-level `get_output_from()` chains
- ✅ **Financial Calculations**: Time value of money, present value discounting
- ✅ **Weighted Scoring**: Multi-criteria decision analysis
- ✅ **Nested Expressions**: Complex arithmetic with multiple operations

See [FORMULAS.md](FORMULAS.md) for detailed formula documentation.

## Dataset

The `assets.csv` file contains asset maintenance alternatives with:

- **Asset Identification**: Asset ID, Alternative ID
- **Financial Data**: Investment cost, Consequence of Failure (CoF)
- **Risk Metrics**: Probability of Failure (PoF) after action
- **Safety Classification**: Negligible, Low, Medium, High, Critical

### CSV Schema

```
Asset_ID,Alternative_ID,Cost_USD,PoF_Post_Action,CoF_Total_USD,Safety_Risk_Level
PUMP_001,Do_Nothing,0,0.25,500000,High
PUMP_001,Refurbish,45000,0.05,500000,Low
PUMP_001,Replace,120000,0.01,500000,Negligible
```

## Running the Application

### Standard Mode (with TUI)

```bash
cd capalloc
cargo run --release
```

The application will:
1. Load asset alternatives from `assets.csv`
2. Calculate all risk metrics in parallel
3. Display performance metrics
4. Launch the interactive TUI

### Benchmark Mode (no TUI)

For performance testing without the UI:

```bash
cargo run --release -- --benchmark
# or
cargo run --release -- -b
```

This mode:
- Loads assets from CSV
- Calculates all risk metrics in parallel
- Displays timing statistics
- Exits without launching the TUI

## TUI Controls

- **↑/↓** or **j/k**: Navigate through alternatives
- **Page Up/Page Down**: Fast navigation
- **Enter** or **Space**: Toggle between summary and detailed view
- **q**: Quit the application

## TUI Interface

The interface shows:

### Header
- Total number of alternatives processed
- Total calculation time
- Average time per calculation

### Left Panel
- List of all asset alternatives with risk reduction values
- Color-coded by alternative type
- Highlighted selection

### Right Panel (Summary View)
- Asset ID and alternative description
- Investment cost
- Risk reduction
- ROI
- Safety risk level
- Calculation time

### Right Panel (Expanded View)
- Complete asset information
- Financial analysis (cost, CoF)
- Risk metrics (PoF, baseline risk, post-action risk, risk reduction)
- Optimization metrics (ROI, cost/benefit ratio)
- Performance statistics

## Performance

The formcalc engine demonstrates:
- **Automatic Dependency Resolution**: Formulas executed in correct order
- **Parallel Execution**: Independent formulas run in parallel
- **Multi-asset Parallelism**: All assets processed simultaneously using Rayon

### Example Performance (25 alternatives)

```
Loaded 25 asset alternatives
Calculating risk metrics in parallel...
Calculated risk metrics for 25 alternatives in 45.90ms
Average time per calculation: 1.84ms
```

With **13 formulas per calculation**, the engine efficiently processes complex interdependent calculations.

## Architecture

### Repository Pattern

The application uses the Repository pattern for both data and formulas:

**Asset Repository:**
- `AssetRepository` trait - abstraction for data access
- `CsvAssetRepository` - loads assets from CSV files
- Easily extensible to database, API, or other sources

**Formula Repository:**
- `FormulaRepository` trait - abstraction for formula loading
- `InMemoryFormulaRepository` - loads formulas as if from a data source
- Formulas treated as data, enabling dynamic updates without recompilation

### Domain-Driven Design

The codebase follows DDD and SOLID principles:
- **Domain Layer**: Core business entities (`Asset`, `OptimizationResult`)
- **Service Layer**: Business logic (`RiskCalculationService`)
- **Repository Layer**: Data access abstractions
- **Application Layer**: Use case orchestration (`CapitalAllocationApp`)
- **UI Layer**: Presentation (`AppState`, rendering)

## Formula Dependencies

The calculator demonstrates formcalc's sophisticated dependency management with **5 execution layers**:

```
Layer 1 (Independent - executed in parallel):
├─ baseline_risk
├─ safety_multiplier
├─ criticality_score
├─ degradation_factor
└─ implementation_complexity

Layer 2 (depends on Layer 1):
├─ post_action_risk → depends on (safety_multiplier, degradation_factor)
└─ time_value_adjustment → depends on (implementation_complexity)

Layer 3 (depends on Layer 2):
├─ risk_reduction → depends on (baseline_risk, post_action_risk)
└─ adjusted_cost → depends on (implementation_complexity, time_value_adjustment)

Layer 4 (depends on Layer 3):
├─ roi → depends on (risk_reduction, adjusted_cost)
└─ payback_period → depends on (risk_reduction, adjusted_cost)

Layer 5 (depends on Layer 4):
├─ cost_effectiveness → depends on (roi, criticality_score)
└─ priority_score → depends on (risk_reduction, roi, criticality_score)
```

The engine automatically:
1. Analyzes formula dependencies
2. Groups formulas into execution layers
3. Executes independent formulas in parallel within each layer
4. Waits for dependencies before proceeding to next layer
5. Computes all 13 metrics correctly regardless of formula order

## Testing

The project includes comprehensive unit tests covering:

- **Domain Layer** (4 tests): Entity behavior, value objects
- **Service Layer** (5 tests): Complex formula calculations
- **Application Layer** (4 tests): Use case orchestration, parallel processing
- **Repository Layer** (2 tests): Formula loading with 13 formulas

Run tests with:
```bash
cargo test
```

All tests pass with zero warnings:
```
running 15 tests
test result: ok. 15 passed; 0 failed; 0 ignored
```

## Use Cases

This optimization approach is applicable to:

- **Asset Maintenance Planning**: Prioritize repairs/replacements based on ROI
- **Risk-Based Inspection (RBI)**: Optimize inspection schedules
- **Budget Allocation**: Maximize risk reduction under budget constraints
- **Portfolio Optimization**: Select best alternatives for multiple assets
- **Safety Investment**: Prioritize investments with highest safety impact

## Example Decision Support

For PUMP_001:
- **Do Nothing**: $0 cost, $125,000 risk → Baseline
- **Refurbish**: $45,000 cost, $475,000 risk reduction → ROI: 10.56
- **Replace**: $120,000 cost, $495,000 risk reduction → ROI: 4.13

**Decision**: Refurbish offers the best ROI (10.56 vs 4.13)

## Future Enhancements

Potential additions:
- Constraint optimization (budget limits, resource constraints)
- Multi-objective optimization (cost, safety, reliability)
- Scenario analysis and sensitivity testing
- Export optimization results
- Integration with maintenance planning systems
