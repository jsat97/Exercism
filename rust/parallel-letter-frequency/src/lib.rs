use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use std::cmp;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let counter : HashMap<char, usize> = HashMap::new();
    let len = input.len();
    if len == 0 {
        return counter;
    }
    //let counter_lock : Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(counter));
    let counter_lock =  Arc::new(Mutex::new(counter));

    let input_bytes : Vec<char> =
        input.iter()
        .flat_map (|&s| s.chars())
        .filter(|c| !c.is_whitespace() && !c.is_ascii_punctuation() && !c.is_ascii_digit())
        .map(|c| c.to_lowercase().next().unwrap_or(c))
        .collect();
    
    let chunk_len = cmp::max(1, len/worker_count);

    // iterator over chunks
    let mut chunks = Vec::new();
    for chunk in input_bytes.chunks(chunk_len) {
        chunks.push(chunk.to_owned());
    }

    println!("chunks {:?}", chunks);
    let mut handles = Vec::new();
     
    // iterate over the vector chunks
    for chunk in chunks {
        println!("chunk {:?}", chunk);
        let counter_clone = Arc::clone(&counter_lock);
        let handle = thread::spawn(move||  {
            chunk.iter()
                .for_each( |c|  {let _=*counter_clone.lock().unwrap().entry(*c).and_modify(|c| *c += 1).or_insert(1); });
                println!("counter in closure: {:?}", counter_clone.lock().unwrap());
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // take hashmap out of arc
    let guard = Arc::into_inner(counter_lock).unwrap();
    // remove guard 
    guard.into_inner().unwrap()
}
