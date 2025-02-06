pub mod index_builder{
    use std::str;
    use crate::bwt::bwt;

    #[derive(Debug)]
    pub struct Index {
        pub Last: Vec<(char,usize)>,
        pub First: Vec<(char, usize)>,
        pub Suffix_Arr: Vec<usize>,
        //The start of sections A, C, G, T in the First row of the index
        pub fm_sections: (usize, usize, usize, usize)
    }
    //Builds an index for the search algorithm to  traverse
    pub fn index_builder(input_str: &str, sort_window: usize) -> Index {
        let mut encoder_variable = bwt::fwd_bwt(input_str);
        println!("{:?}", encoder_variable);
        let mut indexed_seq = Index {
            Last: bwt::assign_ranks(&encoder_variable),
            First: Vec::new(),
            Suffix_Arr: Vec::new(),
            fm_sections: (0,0,0,0)
            };
        
        //Creates the first line of the BWT by taking the transformed line, adding ranks, and sorting it.   
        let mut encoder_variable_vectorized = bwt::assign_ranks(&encoder_variable);
        encoder_variable_vectorized.sort();
        indexed_seq.First = encoder_variable_vectorized;
        
        
        println!("Building index");
        //Suffix array builder, iterates through a window of window_size,
        
        //the index item in a tuple is unpacked and is kept
        
        //Window size for sorting suffixes
        let window_size: usize = 12;
        let uncoded_with_dollar = input_str.to_owned() + "$";
        let mut suffixes: Vec<(&str, usize)> = Vec::new();
        //makes a vector of suffix slices with an index number
        for i in 0..=uncoded_with_dollar.len().saturating_sub(window_size ) {
            let temp_slice = &uncoded_with_dollar[i..(i + window_size)];
            suffixes.push((temp_slice, i));
            println!("Temp_slice in building suffix array{:?}", temp_slice)
        }
        
        for i in uncoded_with_dollar.len() - window_size+1..=uncoded_with_dollar.len() -1 {
            let temp_slice = &uncoded_with_dollar[i..];
            suffixes.push((temp_slice, i));
            println!("Temp_slice in building suffix array{:?}", temp_slice)
        }
        
        // Sorts by string slice
        suffixes.sort_by(|a, b| a.0.cmp(&b.0));
        println!("Suffixes: {:?}", suffixes);
        //Unpacks the tuple and keeps their index values in a new vector
        let suffix_array: Vec<usize> = suffixes.iter().map(|(a, b)| b.to_owned()).collect();
        println!("suffix array {:?}", suffix_array); 
        indexed_seq.Suffix_Arr = suffix_array;

        //Maps the beginning of each character section in the First row
        let mut section_counter: usize = 0;
        let mut previous_char = '$';
        for base in &indexed_seq.First{
            //println!("{:?}", base);
            if base.0 == previous_char{
                section_counter += 1; 
                continue;
                
            }
            else {
                match base.0 {
                    'A' => {indexed_seq.fm_sections.0 = section_counter},
                    'C' => {indexed_seq.fm_sections.1 = section_counter},
                    'G' => {indexed_seq.fm_sections.2 = section_counter},
                    'T' => {indexed_seq.fm_sections.3 = section_counter},
                    _  => continue
                }
                previous_char = base.0;
                section_counter += 1;
            }
            
        }

      
        
        indexed_seq
    }
    

}
#[cfg(test)]
mod test {
    use super::index_builder;


   
    #[test]
    fn indexer_test() {
        let index1 = index_builder::index_builder("GATTACA", 3);
        println!("{:?}", index1)
    }
    
}