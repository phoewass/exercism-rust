pub fn raindrops(number: u16) -> String {

    let mut result = String::new();

    if number%3 == 0 {
        result += "Pling"
    }
    if number%5 == 0 {
        result += "Plang"
    }
    if number%7 == 0 {
        result += "Plong"
    }
     if result.is_empty() {
        result = number.to_string()
    }
    return result
}
