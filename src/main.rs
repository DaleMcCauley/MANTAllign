mod bwt;
mod parsers;
/*Flow of program. Functions will be filled out as I develop the program */
fn main() {
    let reference_fq = parsers::fastq::fastq_parse("/home/dale/rust/projects/MANTAllign/samples/trimmed_reads.fastq");
    print!("{:?}", reference_fq);
    //let input = takeinput();
/* 
    match input { 
        index_ref => build_index(reference_seq),
        allign_fq => allign(input.fastq_path),


    }
*/
}
