// Domain-Driven Design and SOLID Principles Applied
// - Domain: Models representing core business entities (Asset, Alternative, OptimizationResult)
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

fn main() -> Result<(), Box<dyn Error>> {
    // Check for benchmark mode
    let args: Vec<String> = std::env::args().collect();
    let benchmark_mode =
        args.contains(&"--benchmark".to_string()) || args.contains(&"-b".to_string());

    // Initialize application with CSV repository and formula repository
    let asset_repository = Box::new(CsvAssetRepository::new("assets.csv".to_string()));
    let formula_repository = Box::new(InMemoryFormulaRepository::new());
    let app = CapitalAllocationApp::new(asset_repository, formula_repository);

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

    // If in benchmark mode, exit without launching UI
    if benchmark_mode {
        println!("\nBenchmark complete!");
        return Ok(());
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create UI state and run
    let mut state = AppState::new(results, total_time);
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
