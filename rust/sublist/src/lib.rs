#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}
// use core::usize::MAX;

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.is_empty()   {
        if _second_list.is_empty() {
            return Comparison::Equal;
        } else {
            return Comparison::Sublist;
        }
    } else {
        if _second_list.is_empty() {
            return Comparison::Superlist;
        }
        // both non empty
    }

    if equal(_first_list, _second_list) {
        return Comparison::Equal;
    } else if sub_list(_first_list, _second_list) {
        return Comparison::Sublist;
    } else if sub_list(_second_list,_first_list) {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}

pub fn sub_list<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {
    let mut start_vec : Vec<usize> = vec![];
    let mut found = false;

    // possible start indices in 2nd array
    for j in 0.._second_list.len() {
        if _first_list[0] == _second_list[j] {
            start_vec.push(j);
            found = true;
        }
    }
    if !found {
        return false;
    }
    let m = _first_list.len();
    let n = _second_list.len();

    for start in start_vec {
        if m > n - start {  //remainder elements in 2nd list
            return false; 
        }
        let mut j = start +1;
        let mut i = 1;

        while _first_list[i] == _second_list[j] { 
            i += 1;
            j += 1;
            if i == m || j == n {
                break;
            }
        }
        if i == m {
            return true;
        }
    }

    return false;
}

pub fn equal<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> bool {

    let m = _first_list.len();
    let n = _second_list.len();

    if  m != n {
        return false;
    }

    let mut j : usize = 0;
    while _first_list[j] == _second_list[j] { 
        j += 1;
        if j == m {
            break;
        }
    }
    println!("j after comparison = {j}");
    if j == m {
        return true;
    }
    return false;

}
