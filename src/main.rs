mod function;
mod infomation;
struct BojCode {
    submission_number : u64,
    boj_number : u32,
    lang : String,
    your_code : String,
}
fn main() {
    //id, cookie 정보를 입력받는다.
    let (id,cookie) = infomation::input_info();

    //Cookie 가 유효한지 확인한다.
    let test_connection = function::connection_test(id.clone(),cookie.clone()).expect("인증 실패");
    println!("{test_connection}");
    
    //정답을 맞은 제출번호를 다 가져와서 vector로 저장한다.
    //https://www.acmicpc.net/status?&user_id=ldj050101&result_id=4
    let submission_num = function::get_submission_num(id.clone(),cookie.clone());


    //제출번호를 모두 가져온걸 바탕으로 소스코드를 긁어와 구조체 벡터에 저장.
    //그리고 boj number가 겹치지 않는지 검사.
    //https://www.acmicpc.net/source/46588382

    //구조체 벡터에 있는 정보를 파일로 저장
    
}
