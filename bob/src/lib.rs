pub fn reply(request: &str) -> &'static str{
    match request {
        "" => "Fine. Be that way!",
        _ if request.ends_with('?') => "Sure.",
        _ if request == request.to_uppercase() => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
