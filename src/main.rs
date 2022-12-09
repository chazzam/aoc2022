//use log::{info, trace};
use simplelog;

pub mod days1_2;
pub mod days3_5;
pub mod days6_8;

fn main() {
    let _term_logger = simplelog::TermLogger::new(
        simplelog::LevelFilter::Info,
        //simplelog::LevelFilter::Trace,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );

    days1_2::run_days();
    days3_5::run_days();
    days6_8::run_days();

    println!("\n\nAll done for now, Goodbye!");
}
