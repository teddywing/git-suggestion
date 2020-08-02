use regex::{self, Regex};


/// Check if `s` is a numeric ID.
pub fn is_suggestion_id(s: &str) -> Result<bool, regex::Error> {
    let re = Regex::new(r"^\d+$")?;

    Ok(re.is_match(s))
}
