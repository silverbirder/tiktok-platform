extern crate storer;


use mysql::*;
use mysql::prelude::*;
use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Input {
    id: String,
}

fn main() {
    dotenv().ok();
    let input_data = env::var("INPUT_DATA").expect("INPUT_DATA must be set");
    let v: Input = serde_json::from_str(&input_data).unwrap();
    print!("{:?}", v.id);
    // let db_user = env::var("DB_USER").expect("DB_USER must be set");
    // let db_pass = env::var("DB_PASS").expect("DB_PASS must be set");
    // let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    // let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    // let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    // let protocol = (*"mysql://").to_string();
    // let db_url = protocol + &db_user + ":" + &db_pass + "@" + &db_host + ":" + &db_port + "/" + &db_name;
    // let opts = Opts::from_url(&db_url).unwrap();
    // let pool = Pool::new(opts).unwrap();
    
    // let mut conn = pool.get_conn().unwrap();
    
    // conn.query_drop(
    //     r"CREATE TEMPORARY TABLE payment (
    //         customer_id int not null,
    //         amount int not null,
    //         account_name text
    //     )").unwrap();
    
    // let payments = vec![
    //     Payment { customer_id: 1, amount: 2, account_name: None },
    //     Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
    //     Payment { customer_id: 5, amount: 6, account_name: None },
    //     Payment { customer_id: 7, amount: 8, account_name: None },
    //     Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    // ];
    
    // conn.exec_batch(
    //     r"INSERT INTO payment (customer_id, amount, account_name)
    //       VALUES (:customer_id, :amount, :account_name)",
    //     payments.iter().map(|p| params! {
    //         "customer_id" => p.customer_id,
    //         "amount" => p.amount,
    //         "account_name" => &p.account_name,
    //     })
    // ).unwrap();

    // let selected_payments = conn
    //     .query_map(
    //         "SELECT customer_id, amount, account_name from payment",
    //         |(customer_id, amount, account_name)| {
    //             Payment { customer_id, amount, account_name }
    //         },
    //     ).unwrap();
    
    // assert_eq!(payments, selected_payments);
    // println!("Yay!");
}
