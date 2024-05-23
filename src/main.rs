use clap::Parser;
pub mod date;
pub mod scrape;

#[derive(Parser, Debug)]
struct Args {
    /// NR 3 char station code
    #[clap(verbatim_doc_comment)]
    station: String,

    /// ISO formatted date between year 2001 and 2100
    /// <year>-<month>-<day>
    #[clap(verbatim_doc_comment, default_value_t = date::get_current_date_iso())]
    date: String,

    /// Range of days to search starting on DATE
    #[clap(verbatim_doc_comment)]
    #[arg(default_value_t = 1)]
    range: u8,

    /// Only show calls at platform zero
    #[clap(verbatim_doc_comment)]
    #[arg(short, long, default_value_t = false)]
    only_platform_zero: bool,
}

fn main() {
    let args = Args::parse();
    let station = args.station.as_str();
    let mut date = date::Date::from_iso(args.date.as_str())
        .expect("INVALID DATE (must exist between year 2001 and 2100)");

    // main program
    println!(
        "date      , arr., origin                    , destination               , dep., plt, toc"
    );
    for _ in 0..args.range {
        let day_service_list = scrape::get_services(&date.get_iso(), station);
        scrape::csv_services(&date.get_iso(), &day_service_list, !args.only_platform_zero);
        date.increment_day()
    }
}
