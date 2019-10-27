extern crate flate2;

use std::{fs};
use std::io;
use std::io::prelude::*;
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::path::Path;
use regex::{Regex};


fn file_extension(file_path: &str) ->  Option<&str> {
    let path = Path::new(file_path);
    let p_ext = path.extension();
    match p_ext {
        Some(ext) => ext.to_str(),
        None => Some("")
    }
}

fn get_ip(line: &str) -> Option<&str> {
    let mut iter = line.split_whitespace();
    iter.nth(0)
}

fn gunzip(file_path: &str) ->  io::Result<String> {
    let bytes = fs::read(file_path)?;
    let mut gz = GzDecoder::new(&bytes[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    Ok(s)
}

fn get_file_content(file_path: &str) -> String {
    let content = match file_extension(file_path) {
        Some("gz") => {
            println!("gunzip file: {}, could take somme times ", file_path);
            gunzip(file_path)
        },
        _ => fs::read_to_string(file_path)
    };

    match content {
        Ok(lines) => lines,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    }
}


fn print_head(filename: &str){
    println!("===================================================");
    println!("Top access by IP  in  {}", filename);
    println!("===================================================");
    println!("{:^16}\t{:^7}\t{:^7}", "IP", "nbr access", "percent");
    println!("---------------------------------------------------");

}

fn print_footer(access_nbr: u32){
    println!("---------------------------------------------------");
    println!(" There is {} requests parsed ", access_nbr);
    println!("---------------------------------------------------");

}

fn show_result(file_path: &str, mut ip_count: HashMap<&str, u32>, file_lines: u32, ip_nbr: usize) {
    let mut count_vec = Vec::new();
    for (k, v) in ip_count.drain() {
        count_vec.push((k, v));
    }
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));
    count_vec.truncate(ip_nbr);
    print_head(file_path);
    for (ip, count) in count_vec.iter(){
        println!("{:>16}\t{}\t\t{:.2}\u{2030}",ip, count,  *count as f64 * 100.0 / file_lines as f64);
    }
    print_footer(file_lines);

}
pub fn all_file(file_path: &str, ip_nbr: usize){
    println!("all file");
    let content = get_file_content(&file_path);
    let mut map: HashMap<&str, u32>  = HashMap::new();
    let mut file_lines: u32 = 0; // somme of all IP
    for line in content.lines() {
        file_lines += 1;
        match get_ip(line){
            Some(ip) =>  {
                *map.entry(ip).or_insert(0) += 1;
            },
            None => ()
        }
    };
    show_result(&file_path, map, file_lines, ip_nbr);

}
pub fn filter_lines(file_path: &str, regex_str: &str, ip_nbr: usize){
    let content = get_file_content(&file_path);
    let filter = Regex::new(&regex_str).unwrap();
    let to_parse = content.lines().filter(|line|  filter.is_match(line));
    let mut map: HashMap<&str, u32>  = HashMap::new();
    let mut lines_match: u32 = 0; // somme of all IP
    for line in to_parse {
        lines_match += 1;
        match get_ip(line){
            Some(ip) =>  {
                *map.entry(ip).or_insert(0) += 1;
            },
            None => ()
        }
    };
    show_result(&file_path, map, lines_match, ip_nbr);
}


