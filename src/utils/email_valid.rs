use regex::Regex;

pub fn email_valid(input_email: &str) -> bool {
    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .expect("Email regex is invalid !");
    email_regex.is_match(input_email)
}
