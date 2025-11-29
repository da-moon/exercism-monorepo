use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::collections::HashSet;

use regex::RegexBuilder;
#[rustfmt::skip]
#[allow(unused_parens, unused_variables)]
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // NOTE:  useful references
    // The following fails bec
    // let word = word.to_string().to_lowercase().as_str();
    // ------
    // What the compiler sees for the failing case:
    // let word = {
    //     let temp1 = word.to_string();   // lives in this block
    //     let temp2 = temp1.to_lowercase(); // lives in this block
    //     temp2.as_str()  // returns pointer into temp2
    //     // temp2 dropped here!
    //     // temp1 dropped here!
    // };  // word now points to freed memory - DENIED
    // -------
    let word : &str = &str::to_lowercase(word);
    let possible_anagrams  : Vec<&str> = possible_anagrams.to_owned();
    let mut possible_anagrams : Vec<String> = possible_anagrams // alternative way of creating a vector of owned strings
        .to_owned()  // derefrences pointer to the string slice and forms a Vec<&str>
        .into_iter() // into_iter() allows consecutive map function to take <&str> as input while iter() passes a borrow of the input to the upcoming map (&&)
        .map(|x : &str| -> Vec<String>{ // start: in each iteration create a 1x1 matrix that holds a vector with a single string in it
            Vec::<String>::from([x.to_string()].to_owned()) // demostrating a different way of creating a vector of owned strings from string slice
            }) // end of iteration. when iterator is consumed this will create a [1xm] matrix in which each element is a vector, holding a single string which is not desirable.
        .flatten() // flatten the created [1xm] matrix . this effectively converts source matrix elements from a vector to a single owned string which is what we want.
        .collect(); // actually consume the iterator and return the result
    possible_anagrams.dedup(); // deduplicating input
    let result : Vec<String>= possible_anagrams
    .into_iter() // allows iterating over element, each of type &String. iter() would allow iterating over &&String
    .filter(|candidate: &String| -> bool { word.len() == candidate.len()}) // removes candidites that do not have equal length to source word
    .map(move |candidate: String| -> Vec<Vec<char>> { // we are going to iterate over candidate and source strings. each iteration would result in a [1x2] matrix . the first column holds source character vector and the second column hold iteration candidates character vector
        vec![ // start : creating candidate vector of charactacters
            word.chars().collect::<Vec<char>>() // Converting string into a slice of owned chars
        ] // end : creating candidate vector of charactacters. at this point, we have a 1x1 matrix in which it's single element is the character vector of source string
        .iter() // we are not done yet. so we create a new iterator over our 1x1 matrix. as we want to join candidate char vector.
        .cloned() // ensures there are no references when iterator is consumed
        .chain( // start : joining a second iterator (candidate char vector)
                (
                    vec![candidate.chars().collect::<Vec<char>>()] // creating a 1x1 matrix that holds candidate characters vector
                    .iter().cloned() // ensures there are no references when iterator is consumed
                    .collect::<Vec<Vec<char>>>() // Cononsume the iterator and create a vector of size 1 that holds candidate character vector inside
                )
                .iter().cloned() // ensures there are no references when iterator is consumed
            ) // end : joining a second iterator (candidate char vector)
        .collect::<Vec<Vec<char>>>() // consume the iterators and give us the final resulting 1x2 matrix
    }) // at this point our 1x2 matrix is ready
    .fold(Vec::new(),|mut acc:Vec<String>,pair:Vec<Vec<char>>|{
        let mut word_vec : Vec<char> = pair[0].clone().into_iter().collect::<Vec<char>>();
        let mut candidate_vec : Vec<char> = pair[1].clone().into_iter().collect::<Vec<char>>();
        // let mut candidate : String = String::new();
        let candidate : String = pair[1].iter().cloned().fold(String::new(),|mut cs: String,c:char|{
                cs.push(c);
                return cs
            }
        );

        word_vec
        .drain(0..word_vec.len())
        .filter_map(|expected:char|  {
            let expected :String = expected.to_string().as_str().to_owned();
            let idx = (Borrow::<Vec<char>>::borrow(&candidate_vec))
            .iter()
            .cloned()
            .position(|actual : char| -> bool
                {
                    // TODO: check result type
                    let actual : regex::Regex =
                        RegexBuilder::new( actual.to_string().as_str() )
                        .case_insensitive(true)
                        .build()
                        .expect("Invalid Regex");
                    actual.is_match(&expected)
               });
            // TODO: maybe form candidate here
            idx.and_then(|i|{
                Some((BorrowMut::<Vec<char>>::borrow_mut(&mut candidate_vec)).swap_remove(i))
            })
        })
        .for_each(drop);
        // ────────────────────────────────────────────────────────────────────────────────
        let expected :String = word.to_string().as_str().to_owned();
        // TODO: check result type
        let actual : regex::Regex =
            RegexBuilder::new(candidate.to_string().as_str() )
            .case_insensitive(true)
            .build()
            .expect("Invalid Regex");
        if  actual.is_match(&expected) {
            return acc
        }
        // ────────────────────────────────────────────────────────────────────────────────
        if (candidate_vec.is_empty()){
           acc.push(candidate);
        }
        acc
    });
    // https://stackoverflow.com/questions/33216514/how-do-i-convert-a-vecstring-to-vecstr
    // https://users.rust-lang.org/t/hashset-string-intersection-hashset-str/28402/7
    // let r : Vec<&str> = result.iter().map(std::ops::Deref::deref).collect();
    // let h : HashSet<&str> = HashSet::from_iter(r.iter().cloned());
    let mut h :  HashSet<&'a str> = HashSet::new();
    result
        .into_iter().for_each(|x:String|{
        // XXX: figure out how to convert vec<string> into hashset<&'a str>
        h.insert(x.leak());
    });
    h
}
