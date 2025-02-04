use bwt::{fwd_bwt, inverse_bwt};


pub mod bwt {
    

  
    
    //Encodes using Burrows-Wheeler transform
    pub fn fwd_bwt(input_str: &str) -> String {
        let string_appended = input_str.to_owned() + "$";
        let mut indices: Vec<usize> = (0..string_appended.len()).collect();
        indices.sort_by_key(|&i| (string_appended[i..].to_string() + &string_appended[..i]));
        println!("{:?}",indices);
        let bwt_string:String = indices.iter().map(|&i|{
            let last_index = (i + string_appended.len() - 1) % string_appended.len();
            string_appended.chars().nth(last_index).unwrap()
        }).collect();
        bwt_string
    }
    // Decodes BWT
    pub fn inverse_bwt(input_str: &str) -> String {
        //Adds ranks to each letter based on how many of them came before it
         fn assign_ranks(input:String) -> Vec<(char, usize)> {
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
                    '$' => {temp_string.push(('$', a_cnt))},
                    _ => println!("Invalid char")
                    
                }
            }
           temp_string
        }

        //Follows path used in inverse BWT algorithm
        let mut result_vec:Vec<(char, usize)> = Vec::new();       
        let mut row = 0;
        let encoded_ranked: Vec<(char, usize)> = assign_ranks(input_str.to_string());
        let mut encoded_sorted_ranked:Vec<(char, usize)> = assign_ranks(input_str.to_string());
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
}
#[cfg(test)]
mod tests {
    use super::bwt::{fwd_bwt, inverse_bwt};
    #[test]
    fn bwt_test() {
        let transformed = fwd_bwt("GATA");
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