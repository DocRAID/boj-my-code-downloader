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

        return Ok("Cookie 인증 성공".to_string());
    }
    else {
        return Ok("Cookie 인증 실패".to_string());
    }
   
}
pub fn get_submission_num(id:String,cookie:String) -> Vec<i32>{
    //조심해야 하는점 : 과도한 트레픽 발생 우려 (적절한 sleep으로 해결)
    let mut num:Vec<i32> = Vec::new();
    
    return num;
}