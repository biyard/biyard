fn password_policy_checklist(password: &str, email: Option<&str>, name: Option<&str>) -> [bool; 7] {
    let lowered_password = password.to_ascii_lowercase();
    let email_ok = email
        .and_then(|value| value.split('@').next())
        .map(str::trim)
        .filter(|value| value.len() >= 3)
        .map(|value| !lowered_password.contains(&value.to_ascii_lowercase()))
        .unwrap_or(true);
    let name_ok = name
        .map(|value| {
            !value
                .split(|c: char| !c.is_ascii_alphanumeric())
                .map(str::trim)
                .filter(|part| part.len() >= 3)
                .map(|part| part.to_ascii_lowercase())
                .any(|part| lowered_password.contains(&part))
        })
        .unwrap_or(true);

    [
        password.chars().count() >= 12,
        password.chars().any(|c| c.is_ascii_lowercase()),
        password.chars().any(|c| c.is_ascii_uppercase()),
        password.chars().any(|c| c.is_ascii_digit()),
        password
            .chars()
            .any(|c| !c.is_ascii_alphanumeric() && !c.is_whitespace()),
        !password.chars().any(|c| c.is_whitespace()),
        email_ok && name_ok,
    ]
}

pub fn validate_password_rules(
    password: &str,
    email: Option<&str>,
    name: Option<&str>,
) -> Vec<String> {
    let mut violations = Vec::new();

    let checklist = password_policy_checklist(password, email, name);
    let lowered_password = password.to_ascii_lowercase();

    if !checklist[0] {
        violations.push("Password must be at least 12 characters.".to_string());
    }

    if !checklist[1] {
        violations.push("Password must include a lowercase letter.".to_string());
    }

    if !checklist[2] {
        violations.push("Password must include an uppercase letter.".to_string());
    }

    if !checklist[3] {
        violations.push("Password must include a number.".to_string());
    }

    if !checklist[4] {
        violations.push("Password must include a special character.".to_string());
    }

    if !checklist[5] {
        violations.push("Password cannot contain spaces.".to_string());
    }

    if !checklist[6] {
        let contains_email = email
            .and_then(|value| value.split('@').next())
            .map(str::trim)
            .filter(|value| value.len() >= 3)
            .map(|value| lowered_password.contains(&value.to_ascii_lowercase()))
            .unwrap_or(false);
        let contains_name = name
            .map(|value| {
                value
                    .split(|c: char| !c.is_ascii_alphanumeric())
                    .map(str::trim)
                    .filter(|part| part.len() >= 3)
                    .map(|part| part.to_ascii_lowercase())
                    .any(|part| lowered_password.contains(&part))
            })
            .unwrap_or(false);

        if contains_email {
            violations.push("Password cannot contain your email.".to_string());
        }
        if contains_name {
            violations.push("Password cannot contain your name.".to_string());
        }
        if !contains_email && !contains_name {
            violations.push(
                "Password cannot contain personal identifiers such as your email or name."
                    .to_string(),
            );
        }
    }

    violations
}

pub fn password_policy_error_message(
    password: &str,
    email: Option<&str>,
    name: Option<&str>,
) -> Option<String> {
    let violations = validate_password_rules(password, email, name);
    if violations.is_empty() {
        None
    } else {
        Some(violations.join(" "))
    }
}
