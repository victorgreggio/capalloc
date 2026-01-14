# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Capital Allocation Optimizer
- 13 interdependent formulas for risk calculation
- Linear programming optimization using minilp
- Three optimization strategies:
  - Risk Reduction Maximization
  - Priority Score Maximization
  - Combined Strategy (60% risk, 40% priority)
- Interactive TUI with ratatui
- Multi-strategy comparison view with colored asterisk indicators
- Portfolio optimization under budget constraints
- Parallel processing with Rayon
- Large dataset generator (4,000 alternatives)
- Comprehensive documentation (README, FORMULAS, OPTIMIZATION)
- GitHub Actions CI/CD workflows
- Cross-platform support (Linux, Windows, macOS)
- Security audit in CI pipeline
- Benchmark mode for performance testing

### Features

#### Formula Engine
- Automatic dependency resolution with 5-layer execution
- Complex mathematical functions (exp, ceil, max, min, power, rnd)
- Nested conditionals and multi-level dependencies
- Financial calculations (time value of money, ROI, payback period)
- Weighted scoring systems for multi-criteria optimization

#### Optimization
- Binary Integer Linear Programming formulation
- Pure Rust LP solver (minilp) - no native dependencies
- Optimal solutions for LP relaxation
- Budget constraint enforcement
- One alternative per asset constraint (SOS1)
- Sub-second optimization for 4,000 variables (~650ms)

#### User Interface
- Interactive TUI with scrolling list
- Three-column strategy comparison (R, P, C)
- Color-coded indicators (Red, Yellow, Green)
- Expanded detail view with all metrics
- Real-time performance statistics
- Keyboard navigation (arrows, j/k, Page Up/Down)

#### Performance
- 4,000 alternatives calculated in ~2.3 seconds
- 0.57ms average per calculation (13 formulas)
- Parallel processing with Rayon
- Efficient sparse constraint matrix handling

#### Data Management
- CSV-based asset repository
- Flexible data model (Asset, Alternative, OptimizationResult)
- Large dataset generator with deterministic PRNG
- Support for multiple asset types and risk levels

### Technical Details

#### Dependencies
- `formcalc` - Custom formula engine
- `minilp` - Pure Rust LP solver
- `ratatui` - Terminal UI framework
- `crossterm` - Terminal manipulation
- `rayon` - Parallel processing
- `csv` - CSV parsing
- `serde` - Serialization

#### Architecture
- Domain-Driven Design (DDD)
- Repository pattern for data access
- Service layer for business logic
- Application layer for use case orchestration
- Clean separation of concerns
- SOLID principles throughout

### Documentation
- Complete README with usage examples
- FORMULAS.md detailing all 13 formulas
- OPTIMIZATION.md explaining LP formulation
- CONTRIBUTING.md for development guidelines
- Inline code documentation
- GitHub Actions workflow documentation

### Quality Assurance
- 18 unit tests (100% passing)
- Clippy linting with strict settings
- Cargo fmt for consistent formatting
- Cross-platform CI testing
- Security auditing with cargo-audit
- Automated release builds

## [0.1.0] - 2026-01-14

### Initial Release
- First public release
- Full feature set as described above
- Production-ready optimization system
- Comprehensive documentation
- CI/CD automation
