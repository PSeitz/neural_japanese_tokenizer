use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use itertools::Itertools;
#[macro_use] extern crate itertools;

extern crate regex;
use regex::Regex;

    // # data
const MAX_LEN:usize = 50;// # maximum length of text
    
    // // # model
    // embed_size = 256 # alias = E
    // encoder_num_banks = 16
    // decoder_num_banks = 8
    // num_highwaynet_blocks = 4
    
    // # training scheme
    // lr = 0.0001
    // logdir = "logdir"
    // batch_size = 32
    // num_epochs = 5
    
    // # inference
    // beam_width = 1 # if beam width is 1, we apply a regular greedy decoding.
    

use std::collections::HashMap;
use std::hash::{Hash};
fn counter<'a, I, K:'a>(vals: I) -> HashMap<K, usize>
    where I: Iterator<Item=&'a K> , K: Hash + Eq + Clone
{
    let mut mappo = HashMap::new();
    for el in vals {
        let count = mappo.entry((*el).clone()).or_insert(0);
        *count += 1;
    }
    mappo
}

fn counter_to_vec<K>(mappo:HashMap<K, usize>) ->  Vec<(K, usize)> 
    where K: Hash + Eq + Clone
{
    let mut veco: Vec<(K, usize)> = mappo.iter().map(|(keyo, count)| (keyo.clone(), count.clone())).collect();
    veco.sort_by(|a, b| b.1.cmp(&a.1));
    veco
}

fn create_counter() {
    let file = BufReader::new(File::open("preprocessed/ja.tsv").expect("Unable to open file"));
    let sentences = file.lines().map(|line|line.unwrap().split("\t").nth(1).unwrap().to_string()).collect_vec();

    let all_chars = sentences.iter().map(|el|el.chars()).flatten().collect_vec();
    let veco = counter_to_vec(counter(all_chars.iter()));
    let mut fout = OpenOptions::new().write(true).append(true).create(true).open("preprocessed/vocab.surface.txt").expect("Unable to create file ja.tsv");

    fout.write("E\t\nU\t\nS\t\n".as_bytes()).expect("Could not write header"); //#E: Empty, U: Unkown
    fout.write_all(veco.iter().map(|el| format!("{}\t{}", el.0, el.1)).join("\n").as_bytes()).expect("Could not write counter");
    // veco
}


fn create_tsv() {
    let mut fout = OpenOptions::new().write(true).append(true).create(true).open("preprocessed/ja.tsv").expect("Unable to create file ja.tsv");

    let regex = Regex::new(r"\s+").unwrap();

    let file = BufReader::new(File::open("jpn_news_2005-2008_1M-sentences.txt").expect("Unable to open file"));
    let mut i = 1;
    for line in file.lines() {
        if i%1000 == 0 { println!("{:?}", i);}
        i+=1;
        let da_line = line.unwrap();
        if da_line.len() > MAX_LEN {
            continue;
        }
        let mut split = da_line.split("\t");
        let idx = split.next().unwrap();
        let line2 = split.next().unwrap();
        let line_no_spaces = regex.replace_all(&line2.as_ref(), "").into_owned();
        if let Err(e) = writeln!(fout, "{}\t{}\t{}", idx, line_no_spaces, line2) {
            println!("{}", e);
        }
    }
}


fn main() {
    create_tsv();
    create_counter();


}