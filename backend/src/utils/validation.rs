pub fn validate_password(password: &str) -> bool {
    let uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let digit = password.chars().any(|c| c.is_ascii_digit());
    let lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let max_length = (1..=32).contains(&password.len());
    let min_length = (1..=16).contains(&password.len());
    
    uppercase && digit && max_length && !min_length && lowercase
}