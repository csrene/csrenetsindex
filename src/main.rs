extern crate scraper;
extern crate actix_web;
extern crate postgres;
#[macro_use]
extern crate askama;
extern crate dotenv;

use postgres::{Connection, TlsMode};

use std::fs::File;
use std::io::{Read, BufReader};
use std::collections::HashMap;

use actix_web::{server, App, HttpRequest, HttpResponse, Responder, Query, Result};
use actix_web::http::Method;
use actix_web::Path;
use actix_web::State;



mod pwgen;
mod routes;
mod controller;
mod config;
mod uriscraper;



pub struct AppState {
    pub conn: Connection,
}



fn main() {

    server::new(move || {

        let uri = config::db_uri();

        let conn = Connection::connect(uri, TlsMode::None).expect("could not connect to database");

        App::with_state(AppState { conn })
            .resource("/passwords", |r| r.method(Method::GET).with(routes::passwords))
        })

        .bind("127.0.0.1:8000").expect("could not bind to port 8000")
        .run()
}


