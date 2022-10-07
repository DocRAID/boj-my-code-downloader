use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

use crate::BojCode;

pub fn restore_files(code_info:BojCode) -> Result<(),Error>{
    todo!("파일이 저장이 안됨");
    let mut output = File::create(format!(".\\download\\{}.{}",code_info.boj_number,code_info.lang))?;
    write!(output, "{}",code_info.your_code)?; 
    return Ok(());
}