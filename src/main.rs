use clap::Parser;
pub mod date;
pub mod scrape;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    station: String,
    date: String,
    range: u8,
    #[arg(short, long, default_value_t = false)]
    only_platform_zero: bool,
}

fn main() {
    let args = Args::parse();
    let station = args.station.as_str();
    let mut date = date::Date::from_iso(args.date.as_str())
        .expect("INVALID DATE (must exist between year 2001 and 2100)");

    // main program
    for _ in 0..args.range {
        let document = scrape::request_document(&date.get_iso(), station);
        let day_service_list = scrape::parse_services(&document);
        scrape::csv_services(&date.get_iso(), &day_service_list, !args.only_platform_zero);
        date.increment_day()
    }
}
