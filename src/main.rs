mod bwt;
mod parsers;
/*Flow of program. Functions will be filled out as I develop the program */
fn main() {
    let reference_fa = parsers::fasta::fasta_parse("reference.fasta");
    print!("{:?}", reference_fa);
    //let input = takeinput();
/* 
    match input { 
        index_ref => build_index(reference_seq),
        allign_fq => allign(input.fastq_path),


    }
*/
}
