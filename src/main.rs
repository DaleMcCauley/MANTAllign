mod bwt;
mod index_builder;
mod parsers;

pub fn main() {

    let reference = crate::parsers::fasta::fasta_parse("./samples/reference.fasta");
    let ref_index = crate::index_builder::index_builder::index_builder(& reference[0].sequence, 20);
    println!("{:?}", ref_index.fm_sections);
    
    let query = "CCTCCATTAGCATAGACATAAAAGGACCTTCTAAC";
    
    
}



#[test]
fn search_test1() {
    
    let ref_index = crate::index_builder::index_builder::index_builder("GATACA", 3);
    let query_results = crate::bwt::bwt::search_ref("ATA", &ref_index);
    assert_eq!(query_results, vec![6]);
    println!("{:?}", query_results);

}
#[test]
fn search_test2() {
    let input_test = "CCTCCATTAGCATAGACATAAAAGGACCTTCTAACCCTCCATTAGCATAGACATAAAAGGACCTTCTAAC";
    let test_slice: &str = &input_test[6..18];
    let ref_index = crate::index_builder::index_builder::index_builder(&input_test, test_slice.len());
    
    println!("{:?}", test_slice);
    let query_results = crate::bwt::bwt::search_ref(&test_slice, &ref_index);
    assert_eq!(query_results, vec![6]);
    println!("{:?}", query_results);
}
//parsers::fasta::fasta_parse("/home/dale/rust/projects/MANTAllign/samples/reference.fasta");  TGTTGCTCTATTACGTTTGTAACACATCATACAAGTTGATGAATTACAACCGTCTACAACATGCACATAACTTTTCCATACATAAATAAAAGGACCTTCTAACATACCATTAACAATTAAGTTGTACATTCGACTCT
#[test]
fn search_test3() {
    let input_test = "CCTCCATTAGCATAGACATAAAAGGACCTTCTAACACCATTAACAATAGTTGTACATTCGACTCTTGTTGCTCTATTACGTTTGTAACACATCATACAAGTTGATGAATTACAACCGTCTACAACATGCACATAACTTTTCCATACATAA";
    let test_slice: &str = &&input_test[17..60];
    let ref_index = crate::index_builder::index_builder::index_builder(&input_test, test_slice.len());
    println!("First: {:?}  \n Last {:?}", ref_index.First, ref_index.Last);
    println!("{:?}", test_slice);
    let query_results = crate::bwt::bwt::search_ref(&test_slice, &ref_index);
    assert_eq!(query_results, vec![17]);
    println!("{:?}", query_results);

}