use scraper::{ElementRef, Html, Selector};

struct Service {
    departure_time: String,
    destination: String,
    platform: u8,
}

struct Date {
    year: u32,
    month: u8,
    day: u8,
}

impl Date {
    fn get_iso(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    fn valid(&self) -> bool {
        if self.year > 2024 {
            return false;
        };
        if self.year < 2021 {
            return false;
        };
        if self.day < 1 {
            return false;
        };
        let max_days = match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.year % 4 == 0 {
                    30
                } else {
                    29
                }
            }
            _ => 0,
        };
        if self.day > max_days {
            return false;
        }
        true
    }

    fn next_day(&mut self) {
        self.day += 1;
        if !self.valid() {
            self.day = 1;
            self.month += 1;
            if !self.valid() {
                self.month = 1;
                self.year += 1;
            }
        };
    }
}

fn get_inner_html(selector: &Selector, service: &ElementRef) -> String {
    let element = service
        .select(selector)
        .map(|x| x.inner_html())
        .next()
        .unwrap_or(String::new());
    return element;
}

fn get_day_services(document: &Html) -> Vec<Service> {
    let mut service_list: Vec<Service> = vec![];
    let service_selector = Selector::parse("a.service").unwrap();
    let destination_selector = Selector::parse("div.location.d>span").unwrap();
    let platform_selector = Selector::parse("div.platform.c.act").unwrap();
    let departure_time_selector = Selector::parse("div.time.d.gbtt").unwrap();

    for service in document.select(&service_selector) {
        let destination = get_inner_html(&destination_selector, &service);
        let platform_string = get_inner_html(&platform_selector, &service);
        let departure_time = get_inner_html(&departure_time_selector, &service);
        let platform = platform_string.parse().unwrap_or(255);
        service_list.push(Service {
            departure_time,
            destination,
            platform,
        });
    }
    service_list
}

fn pretty_print_services(service_list: &Vec<Service>, all_plat: bool) {
    for service in service_list.iter() {
        if service.platform == 0 || all_plat {
            println!(
                "{:4} {:21} {:3} ",
                service.departure_time, service.destination, service.platform
            )
        }
    }
}

fn csv_to_stdout(column1: &String, service_list: &Vec<Service>, all_plat: bool) {
    for service in service_list.iter() {
        if service.platform == 0 || all_plat {
            println!(
                "{}, {}, {}, {}",
                column1, service.departure_time, service.destination, service.platform
            )
        }
    }
}

fn request_document(date: &str, station: &str) -> Html {
    let url = format!("https://www.realtimetrains.co.uk/search/detailed/gb-nr:{}/{}/0000-2359?stp=WVS&show=pax-calls&order=wtt", station, date);
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();
    Html::parse_document(&res)
}

fn main() {
    let csv = false;
    let all_platforms = false;
    let station = "SPT";
    let mut date = Date {
        year: 2024,
        month: 04,
        day: 09,
    };
    let range = 1;

    for _ in 0..range {
        let document = request_document(&date.get_iso(), station);

        let day_service_list = get_day_services(&document);
        if csv {
            csv_to_stdout(&date.get_iso(), &day_service_list, all_platforms);
        } else {
            println!("date: {}", date.get_iso());
            pretty_print_services(&day_service_list, all_platforms);
        }
        date.next_day()
    }
}
