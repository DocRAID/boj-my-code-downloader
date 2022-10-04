use std::fmt::format;

use hyper::header;
use reqwest::Error;
use reqwest::header::COOKIE;
pub fn connection_test(cookie:String) -> Result<(),Error> {
    //https://docs.rs/reqwest/0.7.2/reqwest/header/struct.SetCookie.html
    
    let set_cookie=format!("OnlineJudge={}",cookie);

    let client = reqwest::blocking::Client::new();
    let res = client.get("https://www.acmicpc.net/")
    .header(COOKIE, set_cookie)
    .send()?
    .text()
    .unwrap();
    
    let document = scraper::Html::parse_document(&res);
    let selector = scraper::Selector::parse("ul.loginbar.pull-right>a").unwrap();
    let is_login = document.select(&selector).map(|x| x.inner_html());
    
    println!("{:?}",is_login);
    
    
    //헤더 넣고 로그인 버튼이 있는지 확인한다.
    Ok(())
}
pub fn get_submission_num() -> Vec<i32>{
    let mut num:Vec<i32> = Vec::new();

    return num;
}