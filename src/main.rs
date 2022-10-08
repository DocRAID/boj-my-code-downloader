use std::{time::Duration, thread::sleep, fs};

mod function;
mod infomation;
mod file_system;

#[derive(Debug)]
pub struct BojCode {
    boj_number : String,
    lang : String,
    your_code : String,
}
fn main() {
    //id, cookie 정보를 입력받는다.
    let (id,cookie) = infomation::input_info();

    //Cookie 가 유효한지 확인한다.
    let test_connection = function::connection_test(id.clone(),cookie.clone()).expect("인증 실패");
    println!("{test_connection}");
    
    // //정답을 맞은 제출번호를 다 가져와서 vector로 저장한다.
    let mut submission_numbers:Vec<String> = Vec::new();
    match function::get_submission_num(id.clone(),cookie.clone()) {
        Ok(val) => {
            println!("총 {}개의 코드를 가져오는중",val.len());
            submission_numbers=val;
            },
        Err(_) => {println!("의도치 않은 에러가 발생했습니다. 이슈를 남겨주세요!"); },
    }  
    //dir 없으면 만들고 
    fs::create_dir_all("./download").expect("directory create failed or already exitst");

    for submission_number in submission_numbers{
        //num으로 코드 크롤링
        //제출번호를 모두 가져온걸 바탕으로 소스코드를 긁어와 구조체에 저장.
        // let code_info:BojCode = function::create_BojCode(submission_number,cookie.clone());
        match function::create_boj_code(submission_number,cookie.clone()) {
            Ok(code_info) => {
                println!("문제번호 : {} 번의 코드를 가져오는중.",code_info.boj_number);
                file_system::restore_files(code_info).expect("file exist");
            }
            Err(_) => {

            }
        }
        sleep(Duration::from_millis(500));
    }
    

    //그리고 boj number가 겹치지 않는지 검사.
    //https://www.acmicpc.net/source/46588382

    //구조체 벡터에 있는 정보를 파일로 저장
    
}
