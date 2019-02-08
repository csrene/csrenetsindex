use postgres::{Connection, TlsMode};
use actix_web::{Result, HttpResponse, client, HttpRequest, HttpMessage};
use scraper::{Html, Selector};
use futures::future::Future;

use std::borrow::Cow;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::config;


pub fn parse_body(raw_body: &str) -> Vec<String> {
    let document = Html::parse_document(&raw_body);
    // the list of words are on the second paragraph of the page, right after the page summary
    let selector = Selector::parse("p ~ p").unwrap();

    let text_list = document.select(&selector)
        .into_iter().take(1) //only the first item in the list correspond to the list of words
        .flat_map(|x|{
            x.text() // returns an iterator
        });

    text_list.flat_map(|x| {
        x.split_whitespace().map(|x| {
            String::from(x)
        })
    }).collect()
}

pub fn scrape(uri: &str) {

    actix::run(|| {

        let response_future = {
            client::get(uri)
                .header("User-Agent", "Actix-web")
                .finish()
                .unwrap()
                .send()
                .map_err(|e| {
                    println!("something went wrong at the start{}", e);
                })
        };

        let response_body_future = {
            response_future.and_then(|response| {
                response.body().map_err(|_| ())
            })
        };

        response_body_future.and_then(|body| {
            let db_uri = config::db_uri();
            let conn = Connection::connect(db_uri.clone(), TlsMode::None).expect("could not connect to database");
            let utf8body: Cow<str> = String::from_utf8_lossy(&body);
            let parsed_body: Vec<String> = parse_body(&utf8body);
            println!("most popular words: {:?}", &parsed_body);
            insert_into_words(&conn, parsed_body, "english");
            Ok(())
        }).map_err(|_| println!("something went wrong at the end"))


    })
}


fn insert_into_words(conn: &Connection, words: Vec<String>, lang: &str) {

    let stmt = conn.prepare("INSERT INTO words (name, language) VALUES ($1, $2)").unwrap();


    for word in words {
        stmt.execute(&[&word, &lang]).unwrap();
    }

}


fn get_words_from_file(file: &str) -> Vec<String> {
    let file = File::open(file).expect("oh well");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("dang");
    parse_body(&contents)
}
