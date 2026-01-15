// Domain-Driven Design and SOLID Principles Applied
// - Domain: Models representing core business entities (Asset, Alternative, RiskCalculationResult)
// - Services: Business logic for capital allocation optimization
// - Repository: Data access abstraction
// - Application: Use case orchestration
// - UI: Presentation layer

mod application;
mod domain;
mod repository;
mod services;
mod ui;

use application::CapitalAllocationApp;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use repository::{CsvAssetRepository, InMemoryFormulaRepository};
use std::{error::Error, io, time::Duration};
use ui::AppState;

fn format_money(value: f64) -> String {
    let abs_value = value.abs();
    let formatted = format!("{:.2}", abs_value);

    let parts: Vec<&str> = formatted.split('.').collect();
    let integer_part = parts[0];
    let decimal_part = if parts.len() > 1 { parts[1] } else { "00" };

    let mut result = String::new();
    for (i, ch) in integer_part.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
    }

    let formatted_integer: String = result.chars().rev().collect();
    let sign = if value < 0.0 { "-" } else { "" };
    format!("{}{}.{}", sign, formatted_integer, decimal_part)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Check for benchmark mode and budget
    let args: Vec<String> = std::env::args().collect();
    let benchmark_mode =
        args.contains(&"--benchmark".to_string()) || args.contains(&"-b".to_string());

    // Parse budget if provided
    let budget = args
        .iter()
        .position(|a| a == "--budget" || a == "-B")
        .and_then(|i| args.get(i + 1))
        .and_then(|b| b.parse::<f64>().ok());

    // Initialize application with CSV repository and formula repository
    let applicant_repository = Box::new(CsvAssetRepository::new("assets.csv".to_string()));
    let formula_repository = Box::new(InMemoryFormulaRepository::new());
    let app = CapitalAllocationApp::new(applicant_repository, formula_repository);

    // Load assets from repository
    let assets = app.load_assets()?;
    let asset_count = assets.len();

    println!("Loaded {} asset alternatives", asset_count);

    // Warn about large datasets in debug mode
    #[cfg(debug_assertions)]
    if asset_count > 1000 {
        eprintln!(
            "\n⚠️  WARNING: Running {} assets in DEBUG mode will be very slow!",
            asset_count
        );
        eprintln!("   For large datasets, use RELEASE mode:");
        eprintln!(
            "   cargo run --release{}\n",
            if benchmark_mode {
                " -- --benchmark"
            } else {
                ""
            }
        );
        eprintln!("   Press Ctrl+C to cancel, or wait for debug build to complete...\n");
    }

    println!("Calculating risk metrics in parallel...");

    // Calculate all risk metrics in parallel
    let (results, total_time) = app.calculate_all_risks(assets);

    println!(
        "Calculated risk metrics for {} alternatives in {:.2}ms",
        results.len(),
        total_time.as_secs_f64() * 1000.0
    );
    println!(
        "Average time per calculation: {:.2}ms",
        total_time.as_secs_f64() * 1000.0 / results.len() as f64
    );

    // Run optimization if budget is provided
    if let Some(budget_amount) = budget {
        println!("\n=== PORTFOLIO OPTIMIZATION (Linear Programming) ===");
        println!("Budget constraint: ${}", format_money(budget_amount));
        println!("Using minilp solver for optimal solution");

        let opt_start = std::time::Instant::now();

        // Strategy 1: Maximize risk reduction
        match app.optimize_by_risk_reduction(&results, budget_amount) {
            Ok(solution) => {
                let opt_time = opt_start.elapsed();
                println!("\n--- Strategy 1: Maximize Risk Reduction ---");
                println!("Selected {} alternatives", solution.num_assets_optimized);
                println!("Total cost: ${}", format_money(solution.total_cost));
                println!(
                    "Total risk reduction: ${}",
                    format_money(solution.total_risk_reduction)
                );
                println!("Total priority score: {:.4}", solution.total_priority_score);
                println!(
                    "Optimization time: {:.2}ms",
                    opt_time.as_secs_f64() * 1000.0
                );

                if solution.num_assets_optimized <= 10 {
                    println!("\nSelected alternatives:");
                    for alt in &solution.selected_alternatives {
                        println!("  - {}", alt);
                    }
                }
            }
            Err(e) => eprintln!("Optimization error: {}", e),
        }

        // Strategy 2: Maximize priority score
        let opt_start = std::time::Instant::now();
        match app.optimize_by_priority(&results, budget_amount) {
            Ok(solution) => {
                let opt_time = opt_start.elapsed();
                println!("\n--- Strategy 2: Maximize Priority Score ---");
                println!("Selected {} alternatives", solution.num_assets_optimized);
                println!("Total cost: ${}", format_money(solution.total_cost));
                println!(
                    "Total risk reduction: ${}",
                    format_money(solution.total_risk_reduction)
                );
                println!("Total priority score: {:.4}", solution.total_priority_score);
                println!(
                    "Optimization time: {:.2}ms",
                    opt_time.as_secs_f64() * 1000.0
                );

                if solution.num_assets_optimized <= 10 {
                    println!("\nSelected alternatives:");
                    for alt in &solution.selected_alternatives {
                        println!("  - {}", alt);
                    }
                }
            }
            Err(e) => eprintln!("Priority optimization error: {}", e),
        }

        // Strategy 3: Combined weighted objective
        let opt_start = std::time::Instant::now();
        match app.optimize_combined(&results, budget_amount, 0.6, 0.4) {
            Ok(solution) => {
                let opt_time = opt_start.elapsed();
                println!("\n--- Strategy 3: Combined (60% Risk, 40% Priority) ---");
                println!("Selected {} alternatives", solution.num_assets_optimized);
                println!("Total cost: ${}", format_money(solution.total_cost));
                println!(
                    "Total risk reduction: ${}",
                    format_money(solution.total_risk_reduction)
                );
                println!("Total priority score: {:.4}", solution.total_priority_score);
                println!(
                    "Optimization time: {:.2}ms",
                    opt_time.as_secs_f64() * 1000.0
                );

                if solution.num_assets_optimized <= 10 {
                    println!("\nSelected alternatives:");
                    for alt in &solution.selected_alternatives {
                        println!("  - {}", alt);
                    }
                }
            }
            Err(e) => eprintln!("Combined optimization error: {}", e),
        }
    }

    // If in benchmark mode, exit without launching UI
    if benchmark_mode {
        println!("\nBenchmark complete!");
        return Ok(());
    }

    // Run all three optimizations for UI if budget provided
    let optimization_results = if let Some(budget_amount) = budget {
        println!("\nRunning all three optimization strategies for UI display...");

        let risk_solution = app.optimize_by_risk_reduction(&results, budget_amount).ok();
        let priority_solution = app.optimize_by_priority(&results, budget_amount).ok();
        let combined_solution = app
            .optimize_combined(&results, budget_amount, 0.6, 0.4)
            .ok();

        if risk_solution.is_some() || priority_solution.is_some() || combined_solution.is_some() {
            println!(
                "Risk Strategy: {} selected",
                risk_solution
                    .as_ref()
                    .map(|s| s.num_assets_optimized)
                    .unwrap_or(0)
            );
            println!(
                "Priority Strategy: {} selected",
                priority_solution
                    .as_ref()
                    .map(|s| s.num_assets_optimized)
                    .unwrap_or(0)
            );
            println!(
                "Combined Strategy: {} selected",
                combined_solution
                    .as_ref()
                    .map(|s| s.num_assets_optimized)
                    .unwrap_or(0)
            );
            Some((risk_solution, priority_solution, combined_solution))
        } else {
            eprintln!("Warning: All optimizations failed");
            None
        }
    } else {
        None
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create UI state and run
    let mut state = if let Some((risk_sol, priority_sol, combined_sol)) = optimization_results {
        AppState::with_optimization(
            results,
            total_time,
            risk_sol
                .map(|s| s.selected_alternatives)
                .unwrap_or_default(),
            priority_sol
                .map(|s| s.selected_alternatives)
                .unwrap_or_default(),
            combined_sol
                .map(|s| s.selected_alternatives)
                .unwrap_or_default(),
            budget.unwrap(),
        )
    } else {
        AppState::new(results, total_time)
    };
    let res = run_ui(&mut terminal, &mut state);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &mut AppState,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::render(f, state))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Down | KeyCode::Char('j') => state.select_next(),
                        KeyCode::Up | KeyCode::Char('k') => state.select_previous(),
                        KeyCode::PageDown => {
                            let page_size = terminal.size()?.height.saturating_sub(6) as usize;
                            state.page_down(page_size);
                        }
                        KeyCode::PageUp => {
                            let page_size = terminal.size()?.height.saturating_sub(6) as usize;
                            state.page_up(page_size);
                        }
                        KeyCode::Enter | KeyCode::Char(' ') => state.toggle_expand(),
                        _ => {}
                    }
                }
            }
        }
    }
}
