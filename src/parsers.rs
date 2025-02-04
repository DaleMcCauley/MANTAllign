use std::io::prelude;
use std::io::BufReader;
use std::fs::File;
pub mod fasta {
    use std::{fs::File, io::{BufRead, BufReader}};


    #[derive(Debug)]
    pub struct FastaEntry{
        header: String,
        sequence: String
    }
    //Reads fasta file into a vector where each element is a Fasta_Entry containing a header and its sequence "seq"
    pub fn fasta_parse(path: &str) -> Vec<FastaEntry> {
        let f = File::open(path).expect("Opening file failed");
        let mut reader = BufReader::new(f);
        let file_lines = reader.lines();
        let mut  entries: Vec<FastaEntry> = Vec::new();
        let mut count: i32 = -1;
        for line in file_lines {
            let line_result = line.expect("Failed to read line");
            if line_result.starts_with('>')  {

                entries.push(FastaEntry{
                    header: line_result,
                    sequence: String::new(),
                } );
                count += 1;
            }
            else {
                entries[count as usize].sequence += &line_result;
            }
            }
        entries

    }

}