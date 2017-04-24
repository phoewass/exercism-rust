pub fn reply(request: &str) -> &'static str{
    let mut reply = "Whatever.";
    if request.is_empty() {
        reply = "Fine. Be that way!"
    } else if request.ends_with("?") {
        reply = "Sure."
    } else if request.split(|c:char| !c.is_alphabetic())
                    .collect::<String>()
                    .replace(char::is_uppercase,"")
                    .is_empty()
    {
        reply = "Whoa, chill out!"
    }
    reply
}
