use reqwest::Error;
use reqwest::header::COOKIE;
use std::time::Duration;
use std::thread::sleep;

struct BojCode {
    submission_number : u64,
    boj_number : u32,
    lang : String,
    your_code : String,
}

pub fn connection_test(id:String,cookie:String) -> Result<String,Error> {
    //https://docs.rs/reqwest/0.7.2/reqwest/header/struct.SetCookie.html

    let client = reqwest::blocking::Client::new();
    let res = client.get("https://www.acmicpc.net/")
    .header(COOKIE, format!("OnlineJudge={}",cookie))
    .send()?
    .text()
    .unwrap();
    
    if !res.contains(&id) {
        panic!("Error log : 쿠키 인증 실패");
    }
    return Ok("Cookie 인증 성공\n잠시만 기다려 주세요.".to_string());
   
}
fn have_next_page(page:String) -> Option<String>{
    
    if !page.contains("다음 페이지") {
        return None; //없으면 걍 하지마
    }
    let mut filter_tag:Vec<&str> = page.split("\" id=\"next_page\">").collect();
    let filter_num:String = filter_tag[0].to_string();
    filter_tag = filter_num.split("result_id=4&amp;").collect();
    
    let mut next_url = String::new();
    if let Some(val) = filter_tag.last(){
        next_url=val.to_string();
    }
    // println!("{}",next_url); 

    Some(format!("&{}",next_url)) //다음페이지 url 넘기기 
}
pub fn get_submission_num(id:String,cookie:String) -> Result<Vec<String>,Error>{
    print!("제출번호를 가져옵니다");
    //조심해야 하는점 : 과도한 트레픽 발생 우려 (적절한 sleep으로 해결)
    let mut num:Vec<String> = Vec::new();
    let client = reqwest::blocking::Client::new();

    // let mut page:String = String::from("&top=49478682");
    let mut page:String = String::from("");
    loop {
        let res = client.get(format!("https://www.acmicpc.net/status?user_id={}&result_id=4&{}",id,page))
        .header(COOKIE, format!("OnlineJudge={}",cookie))
        .send()?
        .text()
        .unwrap();

        let document = scraper::Html::parse_document(&res);
        let num_selector = scraper::Selector::parse("table#status-table.table.table-striped.table-bordered>tbody>tr").unwrap();
        let source = document.select(&num_selector).map(|x| x.inner_html());
        //아래는 원하는 내용을 정제하는것. 이런... 너무 더럽고
        source
        .zip(1..100)
        .for_each(|(item, _number)| {
            let mut filter_tag:Vec<&str> = item.split("</td>").collect();
            let filter_num:String = filter_tag[0].to_string();
            filter_tag = filter_num.split("<td>").collect();
            num.push(filter_tag[1].to_string());

        });
        match have_next_page(res) {
            Some(value) => {page=value} //다음페이지 있는 경우 경로 넘겨주고
            None => {break} //없는경우 멈춤
        }
        print!(".");
        sleep(Duration::from_millis(100));
    }
    println!("완료!");
    return Ok(num);
}
pub fn get_my_codes(submission_number:String){
    format!("https://www.acmicpc.net/source/{}",submission_number);
    //이제 이거가지고 코드 가져오면 됨
}