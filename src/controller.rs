use postgres::Connection;

//use crate::;
use crate::pwgen;

pub fn get_pass(conn: &Connection, pw_word_count: usize, n: usize) -> Vec<String> {

    let limit : i64 = (pw_word_count*n*2) as i64;

    let words: &Vec<String> = &conn.query("SELECT name FROM words LIMIT $1", &[&limit])
        .unwrap().iter().map(|r|r.get(0)).collect();

    pwgen::create_password_list(words,
                                 pw_word_count,
                                 n)

}