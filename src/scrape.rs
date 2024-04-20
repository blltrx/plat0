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

pub fn parse_services(document: &Html) -> Vec<Service> {
    let mut service_list: Vec<Service> = vec![];
    let service_selector = Selector::parse("a.service").unwrap();
    let destination_selector = Selector::parse("div.location.d>span").unwrap();
    let platform_selector = Selector::parse("div.platform.c.act").unwrap();
    let departure_time_selector = Selector::parse("div.time.d.gbtt").unwrap();
    let toc_selector = Selector::parse("div.toc").unwrap();

    for service in document.select(&service_selector) {
        let destination = get_inner_html(&destination_selector, &service);
        let platform_string = get_inner_html(&platform_selector, &service);
        let toc = get_inner_html(&toc_selector, &service);
        let departure_time = get_inner_html(&departure_time_selector, &service);
        let platform = platform_string.parse().unwrap_or(255);
        if destination == String::new() {
            continue;
        };
        service_list.push(Service {
            departure_time,
            destination,
            platform,
            toc,
        });
    }
    service_list
}

pub fn request_document(date: &str, station: &str) -> Html {
    let url = format!("https://www.realtimetrains.co.uk/search/detailed/gb-nr:{}/{}/0000-2359?stp=WVS&show=pax-calls&order=wtt", station, date);
    let result = get(url).unwrap().text().unwrap();
    return Html::parse_document(&result);
}

pub fn csv_services(date: &String, service_list: &Vec<Service>, all_plats: bool) {
    println!("date, departure time, destination, platform, toc");
    for service in service_list.iter() {
        if service.platform == 0 || all_plats {
            println!(
                "{}, {}, {}, {}, {}",
                date, service.departure_time, service.destination, service.platform, service.toc
            )
        }
    }
}
