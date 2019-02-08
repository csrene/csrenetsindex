use crate::{controller, AppState};
use actix_web::{Form, Query, HttpResponse, State, Result};
use std::collections::HashMap;

use askama::Template;
use std::prelude::v1::Vec;
use serde::{Serialize, Deserialize};

#[derive(Template)]
#[template(path = "passwords.html")]
struct PasswordsTemplate<'a> {
    passwords: &'a Vec<String>,
    pw_wc: usize,
    n: usize,
}

#[derive(Template)]
#[template(path = "pizzas.html")]
struct PizzasTemplate<'a> {
    orders: &'a Vec<Order>,
//    order: &'a Order,
}

#[derive(Debug)]
pub struct Order {
    pub item: String,
    pub item_qt: i32,
    pub customer: String
}

#[derive(Deserialize)]
pub struct Customer {
    pub name: String,
    pub email: String
}

#[derive(Deserialize)]
pub struct PostPizzasParams {
    email: String,
    item: String,
    item_qt: i32
}

pub fn passwords((query, state): (Query<HashMap<String, String>>, State<AppState>)) -> Result<HttpResponse> {
    let pw_wc: usize = query.get("pw_wc").map(|wc|wc.parse().unwrap_or(3)).unwrap_or(3);
    let n: usize = query.get("n").map(|n|n.parse().unwrap_or(30)).unwrap_or(30);

    let passwords = &state.db.get_pass(pw_wc, n);

    let s = PasswordsTemplate {
        passwords,
        pw_wc,
        n
    }.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn get_pizzas((query, state): (Query<HashMap<String, String>>, State<AppState>)) -> Result<HttpResponse> {
    let customer: Option<&String> = query.get("customer");
//    let customer_email: Option<String> = query.get("customer_email").flat_map(|x|x.parse());


    let orders: Vec<Order> = customer.map_or_else(||Vec::new(), |x|state.db.get_orders_by_customer(x));

    let s = PizzasTemplate {
        orders: &orders
    }.render().unwrap();



    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub fn post_pizzas((params, state): (Form<PostPizzasParams>, State<AppState>)) -> Result<HttpResponse> {

    let customer : Option<Customer> = state.db.find_customer(&params.email);

    match customer {
        Some(c) => {
            state.db.create_order(&Order{ item: params.item.clone(), item_qt: params.item_qt.clone(), customer: c.name});
            Ok(HttpResponse::from(HttpResponse::Created()))
        },
        None => Ok(HttpResponse::from(HttpResponse::NotFound()))
    }
}

pub fn post_customer((params, state): (Form<Customer>, State<AppState>)) -> Result<HttpResponse> {

    let customer : Option<Customer> = state.db.find_customer(&params.email);

    match customer {
        Some(_) => {
            Ok(HttpResponse::from(HttpResponse::NotAcceptable()))
        },
        None => {
            let submitted_customer = params.into_inner();
            state.db.create_customer( &submitted_customer);
            Ok(HttpResponse::from(HttpResponse::Created()))
        }

    }

}