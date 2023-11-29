#![feature(exclusive_range_pattern)]
const MAX_BEERS: u32 = 99;
const NO_BEERS: u32 = 100;

pub fn verse(n: u32) -> String {
    let line : String = match n { 
        1..MAX_BEERS =>
        format!("{} bottles of beer on the wall, {} bottles of beer.\n Take one down and pass it around, {} bottles of beer on the wall.", MAX_BEERS -n+1, MAX_BEERS -n+1, MAX_BEERS -n),
        MAX_BEERS =>
            format!("1 bottle of beer on the wall, 1 bottle of beer.\n Take it down and pass it around, no more bottles of beer on the wall."),

        NO_BEERS =>
            format!("No more bottles of beer on the wall, no more bottles of beer.\n
                      Go to the store and buy some more, {} bottles of beer on the wall.", MAX_BEERS),
        _ => unreachable!(),
    };
    line
}

pub fn sing(start: u32, end: u32) -> String {
        verse(start)
}
