pub fn verse(index: u8) -> String {
    match index {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n",index,index-1)
    }
}

pub fn sing(upper: u8,lower:u8) -> String {
    (lower..upper+1).rev()
                    .map(|i| verse(i))
                    .collect::<Vec<_>>()
                    .join("\n")
}
