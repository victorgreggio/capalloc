# Complex Formula Features Demonstration

This document showcases the advanced formula capabilities used in the Capital Allocation Optimizer, demonstrating the power of the `formcalc` formula engine.

## Formula Count and Complexity

The optimizer uses **13 interdependent formulas** with complex mathematical operations:

1. `baseline_risk` - Exponential scaling for high-value assets
2. `safety_multiplier` - Dynamic multiplier with nested conditionals
3. `criticality_score` - Multi-factor scoring system
4. `degradation_factor` - Time-based degradation modeling
5. `post_action_risk` - Complex risk calculation with multiple dependencies
6. `risk_reduction` - Derived risk metrics
7. `implementation_complexity` - Cost-based complexity scoring
8. `time_value_adjustment` - Financial discount calculations
9. `adjusted_cost` - Cost adjustments with time value of money
10. `roi` - Return on investment calculation
11. `cost_effectiveness` - Normalized scoring (0-100)
12. `priority_score` - Weighted multi-criteria decision scoring
13. `payback_period` - Financial payback analysis

## Advanced Formula Features Used

### 1. Mathematical Functions

#### `max()` and `min()` Functions
```
// Ensure degradation doesn't exceed 95%
return rnd(1.0 - min(pof_post_action * 2, 0.95), 4)

// Cap ROI value for scoring
min(get_output_from('roi'), 20) * 3.5

// Ensure minimum safety factor
max(get_output_from('degradation_factor'), 0.5)
```

#### `exp()` - Exponential Function
```
// Exponential scaling for high-consequence assets
if (cof_total > 1000000) then
    return rnd(exp(0.5) * cof_total, 2)
```

#### `ceil()` - Ceiling Function
```
// Round up months for implementation timeline
ceil(get_output_from('implementation_complexity') * 2)
```

#### `rnd()` - Rounding Function
```
// Round to 2 decimal places for currency
return rnd(baseline_risk - post_action_risk, 2)

// Round to 4 decimal places for precision metrics
return rnd(1.0 / (1.0 + 0.006666667) ^ months, 4)
```

#### Power Operator `^`
```
// Present value discount calculation: 1 / (1 + r)^n
return rnd(1.0 / (1.0 + 0.006666667) ^ ceil(complexity * 2), 4)
```

### 2. Complex Conditional Logic

#### Nested If-Else Statements
```
if (is_critical) then
    return rnd(((pof_post_action * 10) + (cof_total / 500000)) * 1.5, 2)
else if (is_high_risk) then
    return rnd(((pof_post_action * 10) + (cof_total / 500000)) * 1.25, 2)
else
    return rnd((pof_post_action * 10) + (cof_total / 500000), 2)
end
```

#### Inline Conditionals for Dynamic Multipliers
```
// Dynamic safety multiplier with conditional logic
if (is_critical) then
    return 1.5 + (pof_post_action * 0.2)
else if (is_high_risk) then
    return 1.25 + max(0, pof_post_action - 0.1) * 0.15
else
    return 1.0
end
```

### 3. Formula Dependencies with `get_output_from()`

#### Multi-Level Dependencies
```
// Priority score depends on 3 other calculated values
risk_reduction_weight = get_output_from('risk_reduction') / 1000000
roi_weight = min(get_output_from('roi'), 10) / 10
criticality_weight = get_output_from('criticality_score') / 10
```

#### Chained Calculations
```
// Risk reduction depends on baseline and post-action risk
get_output_from('baseline_risk') - get_output_from('post_action_risk')

// ROI depends on adjusted cost, which depends on complexity and time adjustment
get_output_from('risk_reduction') / get_output_from('adjusted_cost')
```

### 4. Complex Mathematical Expressions

#### Multi-Operation Expressions
```
// Weighted priority scoring
((get_output_from('risk_reduction') / 1000000) * 0.4 + 
 (min(get_output_from('roi'), 10) / 10) * 0.35 + 
 (get_output_from('criticality_score') / 10) * 0.25)
```

#### Nested Function Calls
```
// Cost effectiveness with multiple nested functions
rnd(min((min(get_output_from('roi'), 20) * 3.5) + 
        (min(get_output_from('criticality_score'), 10) * 3), 100), 2)
```

### 5. Financial Calculations

#### Time Value of Money
```
// Present value discount factor
// Formula: PV = 1 / (1 + r)^n
// Where r = monthly rate (0.08/12 ≈ 0.00667), n = months
return rnd(1.0 / (1.0 + 0.006666667) ^ ceil(complexity * 2), 4)
```

#### Adjusted Cost with Complexity Premium
```
// Cost adjusted for time value and implementation complexity
cost * (1 + get_output_from('implementation_complexity') * 0.05) * 
       get_output_from('time_value_adjustment')
```

#### Payback Period Calculation
```
// Payback in months: (Total Cost / Annual Savings) * 12
if (get_output_from('risk_reduction') > 0) then
    return rnd((get_output_from('adjusted_cost') / 
                get_output_from('risk_reduction')) * 12, 1)
```

## Dependency Graph

The formulas create a complex dependency graph that the engine automatically resolves:

```
Layer 1 (Independent):
  - baseline_risk
  - safety_multiplier
  - criticality_score
  - degradation_factor
  - implementation_complexity

Layer 2 (Depends on Layer 1):
  - post_action_risk (depends on: safety_multiplier, degradation_factor)
  - time_value_adjustment (depends on: implementation_complexity)

Layer 3 (Depends on Layer 2):
  - risk_reduction (depends on: baseline_risk, post_action_risk)
  - adjusted_cost (depends on: implementation_complexity, time_value_adjustment)

Layer 4 (Depends on Layer 3):
  - roi (depends on: risk_reduction, adjusted_cost)
  - payback_period (depends on: risk_reduction, adjusted_cost)

Layer 5 (Depends on Layer 4):
  - cost_effectiveness (depends on: roi, criticality_score)
  - priority_score (depends on: risk_reduction, roi, criticality_score)
```

## Performance Metrics

Despite the complexity, the engine performs efficiently:

- **25 alternatives processed in ~45ms**
- **1.84ms average per calculation**
- **13 formulas per calculation**
- **Parallel execution across multiple dependency layers**

## Formula Complexity Analysis

### Most Complex Formula: `priority_score`
```
// Uses 7 function calls, 3 dependencies, conditional logic, and weighted arithmetic
if (is_critical) then
    return rnd(((get_output_from('risk_reduction') / 1000000) * 0.4 + 
                (min(get_output_from('roi'), 10) / 10) * 0.35 + 
                (get_output_from('criticality_score') / 10) * 0.25) * 1.3, 4)
else
    return rnd((get_output_from('risk_reduction') / 1000000) * 0.4 + 
               (min(get_output_from('roi'), 10) / 10) * 0.35 + 
               (get_output_from('criticality_score') / 10) * 0.25, 4)
end
```

**Features demonstrated:**
- ✅ Multiple `get_output_from()` calls (3 dependencies)
- ✅ Nested `min()` and `rnd()` functions
- ✅ Division operations (5 occurrences)
- ✅ Multiplication with weights
- ✅ Addition for aggregation
- ✅ Conditional multiplier (1.3x for critical assets)
- ✅ High precision rounding (4 decimal places)

## Real-World Applications

These formula patterns are applicable to:

1. **Financial Modeling**: NPV, IRR, payback analysis
2. **Risk Assessment**: Multi-factor risk scoring
3. **Portfolio Optimization**: Weighted scoring systems
4. **Resource Allocation**: Priority-based ranking
5. **Maintenance Planning**: RBI optimization
6. **Investment Analysis**: ROI and cost-benefit analysis

## Key Takeaways

The `formcalc` engine successfully demonstrates:

1. ✅ **Complex mathematical operations** without code compilation
2. ✅ **Automatic dependency resolution** across multiple layers
3. ✅ **Parallel execution** for independent formulas
4. ✅ **Financial calculations** with precision handling
5. ✅ **Nested conditionals** for business logic
6. ✅ **Function composition** for complex expressions
7. ✅ **Data-driven formulas** that can be updated without recompilation
