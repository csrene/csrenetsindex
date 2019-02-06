extern crate rand;
extern crate scraper;
extern crate actix_web;
extern crate postgres;
use rand::Rng;
use rand::RngCore;

use scraper::{Html, Selector};

use std::borrow::Cow;

use actix;
use actix_web::{client, HttpMessage};
use futures::Future;




pub fn create_password_list2(all_words: &Vec<String>, pw_word_count: usize, possible_words: usize) -> Vec<String> {

    let random_words_vec: Vec<String> = (0..possible_words*pw_word_count).flat_map(|_| {
        rand::thread_rng().choose(all_words)
    }).map(|x| x.clone()).collect();

    let words_vec: Vec<String> = random_words_vec
        .chunks(pw_word_count).into_iter()
        .map(|x| words_to_password(x.to_vec())).collect();

    words_vec
}

pub fn create_password_list(all_words: &Vec<String>, pw_word_count: usize, n: usize) -> Vec<String> {

    let get_random_words_vec = ||{
        (0..pw_word_count).flat_map(|_| {
            rand::thread_rng().choose(all_words).map(|x|x.clone())
        }).collect()
    };

    let words_vec: Vec<String> = (0..n).map(|_|{
        words_to_password(get_random_words_vec())
    }).collect();

    words_vec
}

pub fn words_to_password(words: Vec<String>) -> String {

    let joined_words = words.join("");

    swap_letters(&joined_words, 3)
}

pub fn swap_letters(text: &String, n: usize) -> String {

    let mut rng = rand::thread_rng();

    let number_gen= (0..).map(|_| {
        rng.gen_range(0, 10) < n
    });

    text.chars().zip(number_gen).map(|(x, y)|{
        match y {
            true => get_alternative(x, rand::thread_rng().next_u32() as usize),
            _ => x
        }
    }).collect()
}

// this function uses this choice logic to make it realiably tested.
// It might not be the best idea, since accessing an array by its index is unsafe
// TODO use a hash map instead
pub fn get_alternative(character: char, choice: usize) -> char {
    match character {
        'a' => ['A','@','4'][choice%3],
        'b' => 'B',
        'c' => 'C',
        'd' => 'D',
        'e' => ['E','&','3'][choice%2],
        'f' => ['F','£'][choice%2],
        'g' => 'G',
        'h' => 'H',
        'i' => ['I','!','1'][choice%3],
        'j' => 'J',
        'o' => ['O','0'][choice%2],
        't' => ['T','7'][choice%2],
        default => default,
    }
}
