extern crate regex;
extern crate data_encoding;

use std::fs::File;
use regex::Regex;
use std::io::Read;
use std::string::String;
use data_encoding::BASE64_MIME;

pub fn get_file_type(hex: &str) -> &str {
    if Regex::new(r"^ffd8ffe0").unwrap().is_match(hex) { 
        return "jpeg" 
    } else if Regex::new(r"^89504e47").unwrap().is_match(hex) {  
        return "png" 
    } else if Regex::new(r"^47494638").unwrap().is_match(hex) { 
        return "gif"
    } else if Regex::new(r"^00000100").unwrap().is_match(hex) {
        return "x-icon"
    }
    panic!("invalid file type")
}

pub fn vec_to_base64(vec: Vec<u8>) -> String {
    let base64 = BASE64_MIME.encode(&*vec);
    let hex = hex::encode(vec);
    return format!("data:image/{};base64,{}", get_file_type(&hex), base64.replace("\r\n", ""));
}

pub fn to_base64(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    return vec_to_base64(vec);
}

pub fn from_base64(base64: String) -> Vec<u8> {
    let offset = base64.find(',').unwrap_or(base64.len())+1;
    let mut value = base64;
    value.drain(..offset);
    return BASE64_MIME.decode(value.as_ref()).unwrap();
}