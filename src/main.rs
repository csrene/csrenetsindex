extern crate scraper;
extern crate actix_web;
extern crate postgres;
#[macro_use]
extern crate askama;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use postgres::{Connection, TlsMode};

use actix_web::{server, App, HttpRequest, HttpResponse, Responder, Query, Result};
use actix_web::http::Method;
use actix_web::Path;
use actix_web::State;
use crate::routes::Order;

mod pwgen;
mod routes;
mod controller;
mod config;
mod uriscraper;

use controller::DbExecutor;

pub struct AppState {
    pub db: DbExecutor,
}

fn main() {

    server::new(move || {

        let uri = config::db_uri();

        let conn = Connection::connect(uri, TlsMode::None).expect("could not connect to database");

        App::with_state(AppState { db: DbExecutor { conn } })
            .resource("/passwords", |r| r.method(Method::GET).with(routes::passwords))
            .resource("/pizzas", |r| {
                r.method(Method::GET).with(routes::get_pizzas);
                r.method(Method::POST).with(routes::post_pizzas)
            })
            .resource("/customer", |r| r.method(Method::POST).with(routes::post_customer))
        })
        .bind("127.0.0.1:8000").expect("could not bind to port 8000")
        .run()
}






fn start_db() {

    create_table();

    uriscraper::scrape("https://www.ef.edu/english-resources/english-vocabulary/top-3000-words/")
}

fn create_table() {
    let uri = config::db_uri();

    let conn = Connection::connect(uri, TlsMode::None).expect("could not connect to database");

    conn.execute("CREATE TABLE words( id SERIAL PRIMARY KEY, name VARCHAR NOT NULL, language VARCHAR)",&[]).expect("could not create table");
}

