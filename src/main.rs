mod checks;
mod cleaner;
mod folder;
mod logger;

use clap::Parser;
use cleaner::Cleaner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    pretend: bool,
}

fn main() {
    logger::init();

    let args = Args::parse();

    checks::check_all();
    let mut cleaner = Cleaner::new(args.pretend);
    cleaner.perform();
}
