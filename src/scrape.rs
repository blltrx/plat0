use reqwest::blocking::get;
use scraper::{ElementRef, Html, Selector};

pub struct Service {
    pub departure_time: String,
    pub destination: String,
    pub platform: u8,
    pub toc: String,
}

fn get_inner_html(selector: &Selector, service: &ElementRef) -> String {
    let element = service
        .select(selector)
        .map(|x| x.inner_html())
        .next()
        .unwrap_or(String::new());
    return element;
}

fn parse_document(document: &Html) -> Vec<Service> {
    let mut service_list: Vec<Service> = Vec::new();

    for service in document.select(&Selector::parse("a.service").unwrap()) {
        let destination =
            get_inner_html(&Selector::parse("div.location.d>span").unwrap(), &service);
        let platform_string =
            get_inner_html(&Selector::parse("div.platform.c.act").unwrap(), &service);
        let departure_time = get_inner_html(&Selector::parse("div.time.d.gbtt").unwrap(), &service);
        let toc = get_inner_html(&Selector::parse("div.toc").unwrap(), &service);
        let platform = platform_string.parse().unwrap_or(255);

        if destination == String::new() {
            // removes terminating services
            continue;
        };

        service_list.push(Service {
            departure_time,
            destination,
            platform,
            toc,
        });
    }
    return service_list;
}

pub fn get_services(date: &str, station: &str) -> Vec<Service> {
    let url = format!("https://www.realtimetrains.co.uk/search/detailed/gb-nr:{}/{}/0000-2359?stp=WVS&show=pax-calls&order=wtt", station, date);
    let result = get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&result);

    parse_document(&document)
}

pub fn csv_services(date: &String, service_list: &Vec<Service>, all_plats: bool) {
    for service in service_list.iter() {
        if service.platform == 0 || all_plats {
            println!(
                "{}, {}, {}, {}, {}",
                date, service.departure_time, service.destination, service.platform, service.toc
            )
        }
    }
}
