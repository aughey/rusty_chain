use anyhow::Result;
use rusty_chain::chain_functions;
use tracing::info;

// Nice and clean.  Macro cleans up extra temp and error handling.
// Downside is that it's not as easy to debug.
fn test_chain(input: i32) -> anyhow::Result<i32> {
    Ok(chain_functions!(
        input,
        add_one,
        multiply_by_two,
        subtract_three
    ))
}

// Sample functions for demonstration
fn add_one(x: i32) -> Result<i32> {
    Ok(x + 1)
}

fn multiply_by_two(x: i32) -> Result<i32> {
    Ok(x * 2)
}

fn subtract_three(x: i32) -> Result<i32> {
    Ok(x - 3)
}

fn main() {
    // Enable tracing printing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)

        .init();

    info!("Starting test_chain(1)");

    // Run the test
    assert_eq!(test_chain(5).unwrap(), 9);
}
