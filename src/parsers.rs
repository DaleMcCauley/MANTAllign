
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
        let reader = BufReader::new(f);
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

pub mod fastq {
    use std::{fs::File, io::{BufRead, BufReader}};
    #[derive(PartialEq)]
    #[derive(Debug)]
    pub struct FastqRead {
        pub seqid: String,
        pub sequence: String,
        pub extra_info: String,
        pub quality: String
    }



    //Creates a vector containing FastqReads storing seqid, sequence, extra_info, and quality
    pub fn fastq_parse(path: &str) -> Vec<FastqRead> {
        //Holds information about what the previous line was while iterating over lines
        #[derive(PartialEq)]
        enum ReadDetected {
            Sequence_Detected,
            QualityDetected,
            PlaceHolder
            }
        let f = File::open(path).expect("Opening file failed");
        let reader = BufReader::new(f);
        let file_lines = reader.lines();
        let mut  entries: Vec<FastqRead> = Vec::new();
        let mut count: i32 = -1;

        let mut detected: ReadDetected = ReadDetected::PlaceHolder;
        //Iterates over lines of file, detecting and storing appropriate data
        for line in file_lines {
            let line_result = line.expect("Failed to read line");

            
            if line_result.starts_with('@')  {
                entries.push(
                    FastqRead {
                        seqid: line_result,
                        sequence: String::new(),
                        extra_info: String::new(),
                        quality: String::new()
                    });
                detected = ReadDetected::Sequence_Detected;
                count += 1;
            }

            else if detected == ReadDetected::Sequence_Detected {
                entries[count as usize].sequence = line_result;
                detected = ReadDetected::PlaceHolder;    
            }
            else if line_result.starts_with('+'){
                entries[count as usize].extra_info = line_result;
                detected = ReadDetected::QualityDetected;
            }
            else if detected == ReadDetected::QualityDetected{
                entries[count as usize].quality = line_result;
                detected = ReadDetected::PlaceHolder;
            }
        }
        entries
    }
}
               
#[cfg(test)]
pub mod parser_test {
use super::fastq::{fastq_parse, FastqRead};
    #[test]
    pub fn test_fastq_parse(){
    let test_parse: Vec<FastqRead> = super::fastq::fastq_parse("/home/dale/rust/projects/MANTAllign/samples/trimmed_reads.fastq");
    let compare_to: FastqRead = super::fastq::FastqRead {
        seqid: String::from("@SRR23645212.3 3/1"),
        sequence: String::from("GGCAGTACAGACAACACGATGCACCACCAAAGGATTCTTGATCCATATTGGCTTCCGGTGTAACTGTTATTGCCTGACCAGTACCAGTGTGTGTACACAACATCTTAACACAATTAGTGATTGGTTGTCCCCCACTAGCTAGATAATCTT"),
        extra_info: String::from("+"),
        quality: String::from("??????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????")
        
    };
    assert_eq!(compare_to, test_parse[2]);
    }

}