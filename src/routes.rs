use crate::{controller, AppState};
use actix_web::{Query, HttpResponse, State, Result};
use std::collections::HashMap;

use askama::Template;


#[derive(Template)]
#[template(path = "passwords.html")]
struct PasswordsTemplate<'a> {
    passwords: &'a Vec<String>,
    pw_wc: usize,
    n: usize,
}

pub fn passwords((query, state): (Query<HashMap<String, String>>, State<AppState>)) -> Result<HttpResponse> {
    let pw_wc: usize = query.get("pw_wc").map(|wc|wc.parse().unwrap_or(3)).unwrap_or(3);
    let n: usize = query.get("n").map(|n|n.parse().unwrap_or(30)).unwrap_or(30);

    let passwords = &controller::get_pass(&state.conn, pw_wc, n);

    let s = PasswordsTemplate {
        passwords,
        pw_wc,
        n
    }.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}