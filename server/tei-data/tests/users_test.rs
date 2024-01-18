use std::time::Duration;

use tokio::time::sleep;

mod common;

#[ctor::ctor]
fn setup() {
    common::start_up();
}

#[ctor::dtor]
fn clean_up() {
    common::clean_up();
}

#[tokio::test]
pub async fn test() {
    let pool = common::get_db().await;
    sleep(Duration::from_secs(10)).await;
    println!("done!");
}

#[tokio::test]
pub async fn test2() {
    let pool = common::get_db().await;
    sleep(Duration::from_secs(10)).await;
    println!("done!");
}

#[tokio::test]
pub async fn test3() {
    let pool = common::get_db().await;
    sleep(Duration::from_secs(10)).await;
    println!("done!");
}
