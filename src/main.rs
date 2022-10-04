mod function;
mod infomation;
struct boj_code {
    submission_number : u64,
    boj_number : u32,
    lang : String,
    your_code : String,
}
fn main() {
    let (mut id, mut cookie) = infomation::input_info();

    let test_connection = function::connection_test(id.clone(),cookie.clone()).expect("인증 실패");
    println!("{test_connection}");

}
