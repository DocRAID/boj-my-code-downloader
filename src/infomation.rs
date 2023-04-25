use std::io;


pub fn input_info() -> (String,String){
    let mut id:String=String::new();
    let mut cookie:String =String::new();
    let test:bool = false;
    if !test{
        print!("input your boj-id :");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        std::io::stdin().read_line(&mut id).unwrap();
        
        print!("input your cookie [OnlineJudge] :");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        std::io::stdin().read_line(&mut cookie).unwrap();
    }
    else{
        id="id".to_string();
        cookie="cookie".to_string();
    }

    return (id.trim().to_owned(),cookie.trim().to_owned());
}