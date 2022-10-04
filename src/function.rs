use std::fmt::format;

use hyper::header;
use reqwest::Error;
use reqwest::header::COOKIE;
pub fn connection_test(id:String,cookie:String) -> Result<String,Error> {
    //https://docs.rs/reqwest/0.7.2/reqwest/header/struct.SetCookie.html
    
    let set_cookie=format!("OnlineJudge={}",cookie);

    let client = reqwest::blocking::Client::new();
    let res = client.get("https://www.acmicpc.net/")
    .header(COOKIE, set_cookie)
    .send()?
    .text()
    .unwrap();
    
    if res.contains(&id) {

        return Ok("로그인 성공".to_string());
    }
    else {
        return Ok("로그인 실패".to_string());
    }
   
}
pub fn get_submission_num() -> Vec<i32>{
    let mut num:Vec<i32> = Vec::new();

    return num;
}