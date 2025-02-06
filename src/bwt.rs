use bwt::{fwd_bwt, inverse_bwt};


pub mod bwt {
    

  
    
    //Encodes using Burrows-Wheeler transform
    pub fn fwd_bwt(input_str: &str) -> String {
        let string_appended = input_str.to_owned() + "$";
        let mut indices: Vec<usize> = (0..string_appended.len()).collect();
        indices.sort_by_key(|&i| (string_appended[i..].to_string() + &string_appended[..i]));
        //println!("{:?}",indices);
        let bwt_string:String = indices.iter().map(|&i|{
            let last_index = (i + string_appended.len() - 1) % string_appended.len();
            string_appended.chars().nth(last_index).unwrap()
        }).collect();
        bwt_string
    }
    
     //Adds ranks to each letter based on how many of them came before it
    pub fn assign_ranks(input:&str) -> Vec<(char, usize)> {
        let mut temp_string:Vec<(char, usize)> = Vec::new();
        let (mut a_cnt, mut c_cnt, mut g_cnt, mut t_cnt) = (0, 0, 0 ,0);
        for c in input.chars() {
            match c {
                'A' => {temp_string.push(('A', a_cnt));
                 a_cnt +=1 },
                 'C' => {temp_string.push(('C', c_cnt));
                 c_cnt +=1 },
                 'G' => {temp_string.push(('G', g_cnt));
                 g_cnt +=1 },
                 'T' => {temp_string.push(('T', t_cnt));
                 t_cnt +=1 },
                '$' => {temp_string.push(('$', a_cnt))
                    },
                _ => println!("Invalid char")
                  }
              }
                     temp_string
        }

    pub fn inverse_bwt(input_str: &str) -> String {
       
       
                    
        // Decodes BWT
        //Follows path used in inverse BWT algorithm
        let mut result_vec:Vec<(char, usize)> = Vec::new();       
        let mut row = 0;
        let encoded_ranked: Vec<(char, usize)> = assign_ranks(input_str);
        let mut encoded_sorted_ranked:Vec<(char, usize)> = assign_ranks(input_str);
        encoded_sorted_ranked.sort();
        println!("{:?}", encoded_sorted_ranked);
        result_vec.push(encoded_ranked[row].clone());

        while result_vec.len() < input_str.len() - 1 {
            let sorted_index = encoded_sorted_ranked.iter().position(|x| *x == encoded_ranked[row].clone()).unwrap();
            println!("{:?}", sorted_index);
            row = sorted_index.clone();
            result_vec.insert(0, encoded_ranked[row].clone());
            
        } 
        //Removes the numbers and exclamation point
        let decoded_vec: Vec<char> = result_vec.iter().map(|&(ch,_)|ch)
        .collect(); 
        decoded_vec.iter().collect()



    }
    pub fn find_section_range (input: char, reference: &Index) -> (usize,usize) {
        //println!("fm_sections{:?}", reference.fm_sections);
        let mut search_range = (0, reference.Last.len());
        match input {
            'A' => {search_range = (1, reference.fm_sections.1)},
            'C' => {search_range = (reference.fm_sections.1, reference.fm_sections.2)},
            'G' => {search_range = (reference.fm_sections.2, reference.fm_sections.3)},
            'T' => {search_range = (reference.fm_sections.3, reference.Last.len())},
            _ => {}
        }
        search_range
    }
    
    

    //Takes pairs of letters starting at the end of the query and stores as a Vec of chars
    use crate::index_builder::index_builder::Index;
    pub fn search_ref(query: &str, reference: &Index) -> Vec<usize> {

        let search_length = query.len();
        //Iinitialize search window
        let mut search_range: (usize, usize) = (0, reference.Last.len());
        
        for i in 2..=search_length {

 
            let search_pair: Vec<char> = query[search_length- i .. search_length + 2 - i].chars().collect();
            println!("Search pair is{:?}", search_pair);
            let fwd_base = search_pair[0];
            let aft_base = search_pair[1];
            let initial_range = find_section_range(aft_base, &reference);
            
            
            // if it is the first iteration  use initial range if not use searchrange 
            println!("{:?}", initial_range);
            let ref_slice_to_search:&[(char, usize)] = if i == 2 {&reference.Last[initial_range.0.. initial_range.1]} else { &reference.Last[search_range.0.. search_range.1]};

            //Searches for fwd base in Range from first column in the last column
            let fwd_results:Vec<(char, usize)> = ref_slice_to_search.iter().enumerate().filter_map(|(i, &x)| if &x.0 == &fwd_base {Some(x)} else {None}).collect();
            println!("fwd results {:?}", fwd_results);
            if fwd_results.len() == 0 {
                println!("out of fwd results");
                return calc_offsets(search_range.0-1, search_range.1+1, reference)
            }
            //Finds indexes of min and max ranks from range in first column
            let range_max = fwd_results.iter().max_by_key(|(x, y)| y).unwrap();
            let range_min = fwd_results.iter().min_by_key(|(x, y)| y).unwrap();
            println!("rangmin, rangemax ={:?},{:?}", range_min, range_max);
            //Slices section of First coloumn containing the fwd base
            let new_range = find_section_range(fwd_base, &reference); 
            let first_slice_to_search = &reference.First[new_range.0.. new_range.1];
            search_range.0 = first_slice_to_search.iter().position(|x| &x == &range_min).unwrap() + new_range.0;
            search_range.1 = first_slice_to_search.iter().rposition(|x| &x == &range_max).unwrap() + new_range.0 + 1;
            println!("searchrange.0 = {:?}, searchrange.1 = {:?}", search_range.0, search_range.1);
            if search_range.1 - search_range.0 == 0 {
                let offset = (reference.Suffix_Arr[search_range.0]) - (search_length - i);
                let offsetminus1 = (reference.Suffix_Arr[search_range.0 ]) - (search_length - i);
                let offsetplus1 = (reference.Suffix_Arr[search_range.0 +2]) - (search_length - i);
                //let offsetplus1 = (reference.Suffix_Arr[search_range.0+2]) - (search_length - i);
                println!("Result found = {:?} The suffix array output just before was :{:?} and just after was:{:?} ", offset, offsetminus1, offsetplus1);

                 return vec![offset]
            };
        
      
            println!("Loop end")
        }
        pub fn calc_offsets(lower_bound:usize, upperbound:usize, reference: &Index) -> Vec<usize> {
            let mut results_vec = Vec::new();
            for i in lower_bound..upperbound {
                print!("Result found via ranges");
                results_vec.push(reference.Suffix_Arr[i])
            }
            results_vec
        }
        println!("result");
        return calc_offsets(search_range.0, search_range.1, &reference);
       

                
    }
    
        
     


}
#[cfg(test)]
mod tests {
    use super::bwt::{fwd_bwt, inverse_bwt};
    #[test]
    fn bwt_test() {
        let transformed = fwd_bwt("GATA");
        println!("{:?}", transformed);
        assert_eq!(transformed, "ATG$A");
    }
    #[test]
    fn inverse_bwt_test() {
        let decoded = inverse_bwt("ATG$A");
        assert_eq!(decoded, "GATA");
    }}
    #[test]
    fn inverse_bwt_test2() {
        let input = fwd_bwt("GAATGCATCGATCCGACTCACTGAGAATGCATCGATCCGACTCACTGAGAATGCATCGATCCGACTCACTGAGAATGCATCGATCCGACTCACTGAGAATGCATCGATCCGACTCACTGAGAATGCATCGATCCGACTCACTGA");
        
        let output = inverse_bwt(&input);
        assert_eq!("GAATGCATCGATCCGACTCACTGAGAATGCATCGATCCGACTCACTGAGAATGCATCGATCCGACTCACTGAGAATGCATCGATCCGACTCACTGAGAATGCATCGATCCGACTCACTGAGAATGCATCGATCCGACTCACTGA", output);
    }
        #[test]
        fn search_test() {
            
        }