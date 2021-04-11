use std::{fs::File, io::Read, sync::Arc, time::Instant};

use url::Url;
use clap::{App, Arg, ArgMatches};
use reqwest::blocking::get;
use scraper::{Html, Selector};

fn main() {
    let now: Instant = Instant::now();

    let matches = App::new("Krista's command line html and css scraper")
        .version("0.1.0")
        .author("https://github.com/krista-chan")
        .about("A scraper for the CLI\nNotes:\n- Uses the w3 css selector syntax (https://www.w3schools.com/cssref/css_selectors.asp)")
        .arg(
            Arg::with_name("html")
                .help("The uri or html file to scrape from")
                .value_name("HTML")
                .takes_value(true)
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("selector")
                .short("s")
                .long("selector")
                .help("The html selector")
                .value_name("SELECTOR")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("is-fragment")
                .short("f")
                .long("is-fragment")
                .help("Only use if the html file is a fragment")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("json")
                .short("j")
                .long("json")
                .help("Output the scraped html as json (outputs html by default)")
        )
        .get_matches();

    parse(matches);

    println!("{}", Instant::now().duration_since(now).as_nanos())
}

fn parse(matches: ArgMatches) {
    let selector = matches.value_of("selector");
    let html = matches.value_of("html");
    let is_json = matches.is_present("json");

    if Url::parse(html.unwrap()).is_ok() {
        let selector = Selector::parse(selector.unwrap()).expect("Invalid syntax");

        log_html(
            Html::parse_document(get_uri(html.unwrap()).unwrap().as_str()),
            selector,
            is_json,
        )
    } else {
        let mut file = File::open(html.unwrap()).expect("Unable to open html file");
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        let selector = Selector::parse(selector.unwrap()).expect("Invalid syntax");

        if matches.is_present("is-fragment") {
            log_html(Html::parse_fragment(&buff), selector, is_json)
        } else {
            log_html(Html::parse_document(&buff), selector, is_json)
        }
    }
}

fn log_html(html: Html, selector: Selector, is_json: bool) {
    for elem in html.select(&selector) {
        let elem_arc = Arc::new(elem);
        if is_json {
            let mut json = std::collections::HashMap::<&str, &str>::new();

            let id = format!("{:?}", elem_arc.clone().id());
            let html = elem_arc.clone().html();
            let inner_html = elem_arc.clone().inner_html();
            let has_children = format!("{}", elem_arc.clone().has_children());
            let parent = elem_arc.clone().parent().unwrap().value().as_element().unwrap().name();

            json.insert("id", id.as_str());
            json.insert("html", &html);
            json.insert("innerHtml", &inner_html);
            json.insert("hasChildren", &has_children);
            json.insert("parentElem", parent);
            
            println!("{:?}", json)
        } else {
            println!("{}", elem.html())
        }
    }
}

fn get_uri(uri: &str) -> Result<String, Box<dyn std::error::Error>> {
    let res = get(uri)?.text()?;
    Ok(res)
}
