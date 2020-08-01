use regex::{self, Regex};


pub fn is_suggestion_id(s: &str) -> Result<bool, regex::Error> {
    let re = Regex::new(r"^\d+$")?;

    Ok(re.is_match(s))
}
