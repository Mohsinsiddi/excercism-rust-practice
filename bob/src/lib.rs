pub fn reply(message: &str) -> &str {
    let s = message.trim();
     
    if s.is_empty() {
        return "Fine. Be that way!";
    }

    let is_question = s.ends_with("?");
    
    let msg = message.chars().filter(|x| x.is_alphabetic()).collect::<Vec<char>>();

    let all_caps = !msg.is_empty() && msg.iter().all(|x| x.is_uppercase());

    if is_question && all_caps {
        return "Calm down, I know what I'm doing!";
    }

    if is_question && !all_caps {
        return "Sure.";
    }

    if !is_question && all_caps {
        return "Whoa, chill out!";
    }

    "Whatever."

}
