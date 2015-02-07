// Created by Jabok @ February 6th 2015
// hamsa
#![feature(core)]
#![feature(io)]
#![feature(path)]
use std::vec::Vec;
use std::iter::Iterator;
use std::old_io::{Buffer, Reader, BufferedReader, File, IoErrorKind};
use std::old_io::fs::PathExtensions;

/// A tokenizer returning string slices from a reader
struct Tokenizer <R: Reader + Buffer> {
    terminators: Vec<char>,
    reader: R,
    current: String,
}
impl <R: Reader + Buffer> Tokenizer <R> {
    /// Creates a new tokenizer from a reader and a set of separating characters
    fn new(reader: R, terminators: Vec<char>) -> Tokenizer<R> {
        Tokenizer{
            reader: reader,
            terminators: terminators,
            current: String::new(),
        }
    }
}
impl <'a> Tokenizer<BufferedReader<&'a [u8]>> {
    /// Creates a new tokenizer from a string slice and a set of separating characters
    fn from_str(s: &'a str, terminators: Vec<char>) -> Tokenizer<BufferedReader<&'a [u8]>> {
        let reader = BufferedReader::new(s.as_bytes());
        let tok = Tokenizer::new(reader, terminators);
        return tok;
    }
}
impl<'a, R: Reader + Buffer> Iterator for Tokenizer<R> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&str> {
        self.current.clear();
        'main: loop {
            match self.reader.read_char() {
                Ok(c) => {
                    // Check for token terminators
                    for &t in self.terminators.iter() {
                        if c == t {
                            if !self.current.is_empty() {
                                return Some(self.current.as_slice());
                            } 
                            continue 'main; // No token: ignore
                        }
                    }
                    
                    // Just add the char
                    self.current.push(c);
                },
            
                Err(e) => { // Error while reading
                    let kind: IoErrorKind = e.kind;
                    match kind {
                        IoErrorKind::EndOfFile => {
                            //println!("End of file reached!");
                            // Check for any remaining tokens
                            if !self.current.is_empty() {
                                return Some(self.current.as_slice());
                            } else {
                                return None
                            }
                        },
                        _ => panic!("Error while reading: {}", e)
                    };
                }
            }
        }
    }
    /// Unreliable due to the tokenizer's buffering nature
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

/// A structure for iteratively splitting stringy things into sentences
struct SentenceSplitter<'a, R: Reader + Buffer> {
    tokenizer: Tokenizer<R>,
    terminators: Vec<&'a str>,
    current: String,
    quote_types: Vec<&'a str>,
}
impl <'a, R: Reader + Buffer> SentenceSplitter<'a, R> {
    fn new(source: Tokenizer<R>) -> SentenceSplitter<'a, R> {
        SentenceSplitter{
            tokenizer: source, 
            current: String::new(),
            terminators: vec![".", "!", "?"],
            quote_types: vec!["\""]
        }
    }
}
impl <'a, 'b, R: Reader + Buffer> Iterator for SentenceSplitter<'a, R> {
    type Item = &'b str;
    fn next(&mut self) -> Option<&str> {
        self.current.clear();
        let mut quote = "";
        'main: loop {
            match self.tokenizer.next() {
                Some(s) => {
                    self.current.push_str(s);
                    
                    // Inside a quote
                    if !quote.is_empty() {
                        if s.ends_with(quote) {
                            return Some(self.current.as_slice())
                        }
                    } 
                    
                    // Not inside a quote
                    else {
                        // Check whether a quote is starting
                        for &qt in self.quote_types.iter() {
                            if s.starts_with(qt) {
                                if s.ends_with(qt) { // It can end again
                                    return Some(self.current.as_slice());
                                }
                                quote = qt;
                                self.current.push_str(" ");
                                continue 'main;
                            }
                        }
                        
                        // Check whether the token is ending normally
                        // It ends in a terminating character
                        if s.ends_with("..") { // Continue thought trails
                            self.current.push_str(" ");
                            continue;
                        }
                        for &t in self.terminators.iter() {
                            if s.ends_with(t) {
                                return Some(self.current.as_slice());
                            }
                        }
                    }
                    // SPAAAAAAAAACE
                    self.current.push_str(" ");
                    
                },
                None => {
                    if self.current.len() != 0 {
                        return Some(self.current.as_slice());
                    } else {
                        return None;
                    }
                }
            }
        }
    }
    /// Unreliable due to the tokenizer's buffering nature
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

fn main() {
    println!("Hello, world!");
    let terminators = vec![' ', '\n', '\t', '\r'];
    // let s: &str = "안녕하세요,\n 잘 지냈어요? \nWhat are \nyou doing? 난 쓰고 있어요. Sentences. Because! That's my????? Purpose.... hm";
//     let tokenizer = Tokenizer::from_str(s, terminators);
//     println!("Tokenizing...");
//     for token in tokenizer {
//         println!("- Got token: {}", token);
//     }
//     println!("Done!");
    let pathstr = "/Users/jakoblautrupnysom/Desktop/Korean/아크_[utf-8]/아크5권.txt";
    let path = Path::new(pathstr);
    if !path.exists() {
        panic!("Cannot find file at {}", pathstr);
    }
    let mut reader = BufferedReader::new(File::open(&path));
    let s = reader.read_to_string().unwrap();
    let tokenizer = Tokenizer::from_str(s.as_slice(), terminators);
    //let tokenizer = Tokenizer::new(reader, terminators);
    let splitter = SentenceSplitter::new(tokenizer);
    println!("Splitting...");
    for sentence in splitter {
        //println!("- Got sentence: '{}'", sentence);
    }
    println!("Done!");
}
