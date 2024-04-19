pub mod date;
pub mod scrape;

fn main() {
    let all_platforms = false;
    let station = "SPT";
    let mut date = date::Date {
        year: 2024,
        month: 04,
        day: 15,
    };
    let range = 6;

    for _ in 0..range {
        let document = scrape::request_document(&date.get_iso(), station);

        let day_service_list = scrape::parse_services(&document);

        scrape::csv_services(&date.get_iso(), &day_service_list, all_platforms);
        date.increment_day()
    }
}
