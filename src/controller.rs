use postgres::Connection;

//use crate::;
use crate::routes::{Order, Customer};
use crate::pwgen;

pub struct DbExecutor {
    pub conn: Connection,
}

impl DbExecutor {

    pub fn get_pass(&self, pw_word_count: usize, n: usize) -> Vec<String> {

        let limit : i64 = (pw_word_count*n*2) as i64;

        let words: &Vec<String> = &self.conn.query("SELECT name FROM words LIMIT $1", &[&limit])
            .unwrap().iter().map(|r|r.get(0)).collect();

        pwgen::create_password_list(words, pw_word_count, n)

    }

    pub fn get_orders_by_customer(&self, customer: &String) -> Vec<Order> {
        self.conn.query("SELECT item, quantity, customer FROM orders WHERE customer = $1", &[customer])
            .unwrap().iter().map(|r| {
            Order {
                item: r.get(0),
                item_qt: r.get(1),
                customer: r.get(2)
            }
        }).collect()
    }

    pub fn create_order(&self, order: &Order) {
        self.conn.execute("INSERT INTO orders (item, quantity, customer) VALUES ($1, $2, $3)",
                          &[&order.item, &order.item_qt, &order.customer]).unwrap();
    }

    pub fn create_customer(&self, customer: &Customer) {
        self.conn.execute("INSERT INTO customers (name, email) VALUES ($1, $2)",
                          &[&customer.name, &customer.email]).unwrap();
    }

    pub fn find_customer(&self, email: &String) -> Option<Customer> {
        let mut result: Vec<Customer> = self.conn.query("SELECT name, email FROM customers WHERE email = $1", &[email])
            .unwrap().iter().map(|r| {
            Customer {
                name: r.get(0),
                email: r.get(1),
            }
        }).collect();
        result.pop()
    }
}