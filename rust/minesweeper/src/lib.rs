use std::char;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
//    unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");

    if minefield.is_empty() {
        return vec![];
    }
    let mut mine_vec: Vec<Vec<char>> = Vec::new();
    for  (_i, s) in minefield.iter().enumerate() {
        let row : Vec<char> = s.chars().collect();
        println!("{:?}", row); 
        mine_vec.push(row);
    }
    let rows = mine_vec.len();
    // assume all rows have the same # of cols
    let cols = mine_vec[0].len();
    if rows == 1 && cols == 0 {
        return vec!["".to_string()];
    }
    println!("rows {rows} cols {cols}");

    for i in 0..rows {
        for j in 0..cols {
            if mine_vec[i][j] == '*' {
                continue;
            }

            let mut score: u32 = 0;
            if j >= 1 && mine_vec[i][j-1] == '*' {
                score +=1;
            }
            if j<cols-1 && mine_vec[i][j+1] == '*' {
                score +=1;
            }
            if i>=1 && mine_vec[i-1][j] == '*' {
                score +=1;
            }
            if i<rows-1 && mine_vec[i+1][j] == '*' {
                score +=1;
            }
            // diagonal above left
            if i>=1 && j>=1 && mine_vec[i-1][j-1] == '*' {
                score +=1;
            }
            // diagonal below left
            if i<rows-1 && j>=1 && mine_vec[i+1][j-1] == '*' {
                score +=1;
            }
            // diagonal above right
            if i>=1 && j<cols-1 && mine_vec[i-1][j+1] == '*' {
                score +=1;
            }
            // diagonal below right
            if i<rows-1 && j<cols-1 && mine_vec[i+1][j+1] == '*' {
                score +=1;
            }
            if score > 0 {
                mine_vec[i][j] = char::from_digit(score, 10).unwrap();
            } else {
                mine_vec[i][j] = ' ';
            }
        }
    }
    //let mut mine_vec_ret : Vec<String> = vec![];
    let mine_vec_ret = mine_vec.into_iter().map(|v| { let s : String = v.iter().collect(); s}).collect();
    println!("mine_vec_ret : {:?}", mine_vec_ret);
    mine_vec_ret
}
