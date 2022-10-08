use std::fs::File;
use std::io::{Write, Error};

use crate::BojCode;

pub fn restore_files(code_info:BojCode) -> Result<(),Error>{
    let mut output = File::create(format!("./download/{}.{}",code_info.boj_number,code_info.lang))?;
    write!(output, "{}",code_info.your_code)?; 
    return Ok(());
}