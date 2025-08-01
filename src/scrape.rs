use reqwest::blocking::get;
use scraper::{ElementRef, Html, Selector};

pub struct Service {
    pub arrival_time: String,
    pub departure_time: String,
    pub origin: String,
    pub destination: String,
    pub platform: u8,
    pub toc: String,
}

fn get_inner_html(selector: &Selector, service: &ElementRef, missing: &str) -> String {
    let element = service
        .select(selector)
        .map(|x| x.inner_html())
        .next()
        .unwrap_or(String::from(missing));
    element
}

fn get_service_list_from_html(document: &Html, missing: &str) -> Vec<Service> {
    let mut service_list: Vec<Service> = Vec::new();

    for service in document.select(&Selector::parse("a.service").unwrap()) {
        let destination = get_inner_html(
            &Selector::parse("div.location.d>span").unwrap(),
            &service,
            missing,
        );
        let origin = get_inner_html(
            &Selector::parse("div.location.o>span").unwrap(),
            &service,
            missing,
        );
        let platform_string = get_inner_html(
            &Selector::parse("div.platform.c.act").unwrap(),
            &service,
            missing,
        );
        let departure_time = get_inner_html(
            &Selector::parse("div.time.d.gbtt").unwrap(),
            &service,
            missing,
        );
        let arrival_time = get_inner_html(
            &Selector::parse("div.time.a.gbtt").unwrap(),
            &service,
            missing,
        );
        let toc = get_inner_html(&Selector::parse("div.toc").unwrap(), &service, missing);
        let platform = platform_string.parse().unwrap_or(255);

        // if destination == String::new() {
        //     // removes terminating services
        //     continue;
        // };

        service_list.push(Service {
            arrival_time,
            departure_time,
            origin,
            destination,
            platform,
            toc,
        });
    }
    service_list
}

pub fn get_services(date: &str, station: &str, missing: &str) -> Vec<Service> {
    let url = format!("https://www.realtimetrains.co.uk/search/detailed/gb-nr:{station}/{date}/0000-2359?stp=WVS&show=pax-calls&order=wtt");
    let result = get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&result);

    get_service_list_from_html(&document, missing)
}

pub fn csv_services(date: &String, service_list: &[Service], all_plats: bool) {
    for service in service_list.iter() {
        if service.platform == 0 || all_plats {
            println!(
                "{:<10}, {:<4}, {:<26}, {:<26}, {:<4}, {:<3}, {:<3}",
                date,
                service.arrival_time,
                service.origin,
                service.destination,
                service.departure_time,
                service.platform,
                service.toc
            )
        }
    }
}
