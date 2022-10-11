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
 
    fs::create_dir_all("./download").expect("directory create failed or already exitst");

    for index in 0..submission_numbers.len(){
        //num으로 코드 크롤링
        //제출번호를 모두 가져온걸 바탕으로 소스코드를 긁어와 구조체에 저장.
        match function::create_boj_code(submission_numbers[index].clone(),cookie.clone()) {
            Ok(code_info) => {
                print!("{} _ 문제번호 : {}", index, code_info.boj_number);

                match file_system::restore_files(code_info) {
                    Ok(_) => println!(" ..complite!"),
                    Err(code_info) => println!(" ..failed!"),
                }
            }
            Err(_) => {
                println!("정보를 가져오는데 문제가 발생했습니다. 이슈에 남겨주세요");
            }
        }
        sleep(Duration::from_millis(200));
    }

    println!("다운로드 성공! 프로그램을 종료합니다.");
    
}
