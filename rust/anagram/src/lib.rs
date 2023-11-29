use std::collections::{HashSet, HashMap};


use ::std::iter::IntoIterator;
#[derive(Eq, PartialEq)]
struct StringWrapper<'a>(&'a str);

impl<'a> IntoIterator for StringWrapper<'a> {
    type Item = char;
    type IntoIter = ::std::str::Chars<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.chars()
    }
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    println!("word {word}");
    println!("anagrams  {:?}", possible_anagrams);
   let mut word_map = HashMap::new();
   for (_,c) in word.chars().enumerate() {
         let cs = c.to_string().to_lowercase();
    
       word_map.entry(cs).and_modify(|cs| *cs +=1).or_insert(1);
   }
    
    // in each of the anagrams, check if chars are in the map
    let mut result = HashSet::new();
    for w in possible_anagrams {
        //let w1 = StringWrapper(w);
        let ana_count : HashMap<String, i32>  = 
        w.to_lowercase().chars()
        .fold(HashMap::new(), |mut map, c| {
                *map.entry(c.to_string()).or_insert(0) += 1;
                map
            });
        let mut flag = true;
        if w.to_lowercase() == word.to_string().to_lowercase() || w.len() !=  word.len()  {
            continue;
        }
        for (c,n) in ana_count {
            if ! (word_map.contains_key(&c)  &&
                  word_map.get(&c) == Some(&n) ) {
                flag = false;
                break;
            }
        }
                
        if flag {
            result.insert(&w as &str);
        }
    }
    return result;
}

