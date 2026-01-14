# Contributing to Capital Allocation Optimizer

Thank you for your interest in contributing! This project demonstrates the `formcalc` formula engine with a full-featured capital allocation optimization application.

## Development Setup

### Prerequisites

- Rust 1.70+ (stable)
- Cargo

### Building

```bash
# Clone the repository
git clone <repository-url>
cd capalloc

# Build the project
cargo build

# Run tests
cargo test

# Build release
cargo build --release
```

## Code Standards

### Formatting

Format your code before committing:

```bash
cargo fmt --all
```

### Linting

Ensure no clippy warnings:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Testing

All tests must pass:

```bash
cargo test --verbose
```

## Architecture

The project follows **Domain-Driven Design (DDD)** principles:

- **Domain**: Core business entities (`Asset`, `OptimizationResult`)
- **Repository**: Data access abstraction (CSV, formulas)
- **Services**: Business logic (risk calculation, optimization)
- **Application**: Use case orchestration
- **UI**: Presentation layer (TUI with ratatui)

## Making Changes

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/your-feature`
3. **Make your changes**
4. **Run tests**: `cargo test`
5. **Format code**: `cargo fmt --all`
6. **Check with clippy**: `cargo clippy --all-targets --all-features -- -D warnings`
7. **Commit with clear message**: `git commit -m "Add feature: description"`
8. **Push to your fork**: `git push origin feature/your-feature`
9. **Create a Pull Request**

## Pull Request Guidelines

- **Clear title and description** explaining what and why
- **Reference related issues** if applicable
- **All tests passing** in CI
- **Code formatted** with `cargo fmt`
- **No clippy warnings** with strict settings
- **Documentation updated** if adding features

## Adding Features

### Adding New Formulas

Formulas are defined in `src/repository/formula_repository.rs`:

```rust
Formula::new(
    "your_formula_name",
    "if(condition, true_value, false_value) + get_output_from('other_formula')",
    vec!["other_formula"], // Dependencies
)
```

### Adding Optimization Strategies

Add new optimization methods in `src/services/optimizer.rs`:

```rust
pub fn optimize_by_custom(
    &self,
    results: &[OptimizationResult],
    budget: f64,
) -> Result<OptimizationSolution, Box<dyn std::error::Error>> {
    // Your optimization logic using minilp
}
```

## Project Structure

```
capalloc/
├── src/
│   ├── main.rs                      # Entry point with TUI
│   ├── domain/                      # Business entities
│   │   └── mod.rs                   # Asset, OptimizationResult
│   ├── repository/                  # Data access
│   │   ├── asset_repository.rs      # CSV loading
│   │   └── formula_repository.rs    # Formula definitions
│   ├── services/                    # Business logic
│   │   ├── risk_calculator.rs       # Risk calculations
│   │   └── optimizer.rs             # LP optimization
│   ├── application/                 # Use cases
│   │   └── mod.rs                   # Application orchestration
│   ├── ui/                          # User interface
│   │   ├── app_state.rs             # UI state management
│   │   ├── renderer.rs              # Main renderer
│   │   └── ui_renderer.rs           # Widget rendering
│   └── bin/
│       └── generate_data.rs         # Data generator
├── assets.csv                       # Sample data
├── .github/
│   └── workflows/
│       ├── ci.yml                   # CI pipeline
│       └── release.yml              # Release automation
├── README.md                        # Main documentation
├── FORMULAS.md                      # Formula documentation
├── OPTIMIZATION.md                  # Optimization details
└── Cargo.toml                       # Dependencies
```

## Testing

### Unit Tests

Each module has comprehensive unit tests:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_optimize_under_budget

# Run with output
cargo test -- --nocapture
```

### Benchmarking

```bash
# Small dataset
cargo run --release --bin capalloc -- --benchmark

# Large dataset with optimization
cargo run --release --bin capalloc -- -b -B 10000000
```

## Documentation

### Code Comments

- **Public APIs**: Documented with doc comments (`///`)
- **Complex logic**: Inline comments explaining why, not what
- **Minimal comments**: Self-documenting code preferred

### README Updates

Update README.md when:
- Adding new features
- Changing usage patterns
- Updating dependencies

## Questions?

Feel free to open an issue for:
- Bug reports
- Feature requests
- Questions about architecture
- Help with contributions

Thank you for contributing! 🚀
