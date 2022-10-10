use reqwest::Error;
use reqwest::header::COOKIE;
use std::time::Duration;
use std::thread::sleep;

use crate::BojCode;

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



pub(crate) fn create_boj_code(submission_number:String,cookie:String) -> Result<BojCode,Error>{
    let client = reqwest::blocking::Client::new();
    let res = client.get(format!("https://www.acmicpc.net/source/{}",submission_number))
        .header(COOKIE, format!("OnlineJudge={}",cookie))
        .send()?
        .text()
        .unwrap();

    //가져와서 구조체에 저장.
    let (num,lang,code) = get_code_info_from_response(res);
    let code_info:BojCode = BojCode { 
        boj_number: num, lang: lang, your_code: code
    };
    return Ok(code_info);
    //이제 이거가지고 코드 가져오면 됨
}

fn get_code_info_from_response(res:String) -> (String ,String ,String){

    //num lang code
    //code
    let mut code = filter(res.clone(),"readonly>".to_string(),"</textarea>".to_string());
    code = html_escape::decode_html_entities(&code).to_string();

    //num
    let num = filter(res.clone(),"</td><td><a href=\"/problem/".to_string(),"\">".to_string());
    
    //lang
    let lang = lang_extension_selector(filter(res.clone(),"<span class='ms-text'></span></td><td>".to_string(),"</td>".to_string()));
    
    return (num,lang,code);
}
fn filter(source:String,from:String,to:String)->String {
    //num lang code
    let mut filter_tag:Vec<&str>=Vec::new();
    let mut filter_temporary:String=String::new();
    //code
    filter_tag = source.split(&from).collect();
    filter_temporary = filter_tag.last().unwrap().to_string();
    filter_tag = filter_temporary.split(&to).collect();
    return filter_tag[0].to_string();

}

fn lang_extension_selector(lang:String)-> String{
    let mut extension:&str = "";
    match lang.as_str() {
        "C++14"|"C++11"|"C++20"|"C++17"|"C++"|"C++98"|"C++98 (Clang)"|"C++11 (Clang)"|"C++14 (Clang)"|"C++17 (Clang)"|"C++20 (Clang)"=> { extension = "cpp"}
        "C11"|"C90"|"C2x"|"C99"|"C99 (Clang)"|"C11 (Clang)"|"C90 (Clang)"|"C2x (Clang)"=> { extension = "c"}
        "Java 8 (OpenJDK)"|"Java 15"|"Java 11"|"Java 8"=> { extension = "java"}
        "Haxe"|"Python 3"|"PyPy3"|"Python 2"|"PyPy2"=> { extension = "py"}
        "C#"|"C# 6.0 (Mono)"|"C# 3.0 (Mono)"=> { extension = "cs"}
        "Rust 2015"|"Rust 2018"|"Rust 2021"=> { extension = "rs"}
        "Go"|"Go (gccgo)"=> { extension = "go"}
        "Kotlin (JVM)"|"Kotlin (Native)"=> { extension = "kt"}
        "Ruby 1.8"|"Ruby"|"Ruby 1.9"=> { extension = "rb"}
        "Assembly (32bit)"|"Assembly (64bit)"=> { extension = "asm"}
        "Bash"=> { extension = "sh"}
        "엄준식"=> { extension = "umm"}
        // ""|""=> { extension = ""}
        _=> { extension = "txt"}
    }
    return extension.to_string();
}
/* 
헉헉 힘들다

"Swift": "swift"
"Text": "txt"
"node.js": "js"
"D": "d"
"D (LDC)": "d"
"F# (Mono)": "fs"
"PHP": "php"
"Pascal": "pas"
"Scala": "scala"
"Lua": "lua"
"Perl": "pl"
"Haskell": "hs"
"F#": "fs"
"Visual Basic": "vb"
"Objective-C": "m"
"Objective-C++": "mm"
"Golfscript": "gs"
"TypeScript": "ts"
"VB.NET 2.0 (Mono)": "vb"
"VB.NET 4.0 (Mono)": "vb"
"Fortran": "f95"
"Scheme": "scm"
"Ada": "ada"
"awk": "awk"
"OCaml": "ml"
"Brainf**k": "bf"
"Whitespace": "ws"
"Tcl": "tcl"
"Rhino": "js"
"Cobol": "cob"
"Pike": "pike"
"sed": "sed"
"Boo": "boo"
"INTERCAL": "i"
"bc": "bc"
"Nemerle": "n"
"Cobra": "cobra"
"Nimrod": "nim"
"Algol 68": "a68"
"Befunge": "bf"
"FreeBASIC": "bas"
"LOLCODE": "lol"
"아희": "aheui"
"Coq": "v"
"Minecraft": "mca"
"SystemVerilog": "sv"
"APECODE": "ape"
"Crystal": "cr"
 */