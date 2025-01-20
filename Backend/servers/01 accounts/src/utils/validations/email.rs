// https://html.spec.whatwg.org/multipage/input.html#valid-e-mail-address

use idna;

pub fn validate_email(email: &str) -> Result<(), String> {
    // Length constraint
    if email.is_empty() {
        return Err("Email is empty".to_string());
    }
    if email.len() > 254 {
        return Err("Email is too long".to_string());
    }

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return Err("Email must contain exactly one '@'".to_string());
    }
    let (local, domain) = (parts[0], parts[1]);

    if !is_valid_local_part(local) {
        return Err("Invalid local".to_string().to_string());
    }

    if !is_valid_domain_part(domain) {
        return Err("Invalid domain".to_string().to_string());
    }

    return Ok(());
}

fn is_valid_local_part(local: &str) -> bool {
    if local.is_empty() || local.len() > 64 {
        return false;
    }

    if local.starts_with('.') || local.ends_with('.') {
        return false;
    }

    let mut prev_char = '\0';
    for ch in local.chars() {
        if ch == '.' && prev_char == '.' {
            return false; // Consecutive dots are not allowed
        }
        if !is_atext(ch) && ch != '.' {
            return false; // Only valid atext or dots are allowed
        }
        prev_char = ch;
    }

    true
}

/// Check if a character is a valid `atext` as defined in RFC 5322.
fn is_atext(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || "!#$%&'*+/=?^_`{|}~-".contains(ch)
}

fn is_valid_domain_part(domain: &str) -> bool {
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }

    // Convert the domain to ASCII/Punycode
    if domain.starts_with('[') && domain.ends_with(']') {
        let ip = &domain[1..domain.len() - 1];
        return is_valid_ip_address(ip);
    }

    let ascii_domain = match idna::domain_to_ascii(domain) {
        Ok(domain) => domain,
        Err(_) => return false, // Invalid Unicode domain
    };

    let label_vec: Vec<&str> = ascii_domain.split('.').collect();

    for label in &label_vec {
        if label.is_empty() {
            return false;
        }
        if !is_valid_label(label) {
            return false;
        }
    }

    true
}

fn is_valid_ip_address(ip: &str) -> bool {
    if ip.parse::<std::net::IpAddr>().is_ok() {
        return true;
    }

    false
}

/// Validates a label in the domain part.
fn is_valid_label(label: &str) -> bool {
    if label.is_empty() || label.len() > 63 {
        return false;
    }

    // Label must start and end with let-dig (letters or digits)
    if !is_let_dig(label.chars().next().unwrap()) || !is_let_dig(label.chars().last().unwrap()) {
        return false;
    }

    // Check for valid ldh-str and no consecutive hyphens
    for ch in label.chars() {
        if !is_ldh_str(ch) {
            return false;
        }
    }

    true
}

/// Checks if a character is a valid `let-dig` (letter or digit).
fn is_let_dig(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
}

/// Check if a character is a valid `ldh-str` (letter, digit or hyphen)
fn is_ldh_str(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '-'
}

#[cfg(test)]
mod tests {
    use crate::utils::validations::email::validate_email;
    #[test]
    fn test_validate_email() {
        let tests =
            vec![
            // Valid emails
            ("email@here.com", true),
            ("weirder-email@here.and.there.com", true),
            ("abc@bar", true),
            (r#"!def!xyz%abc@example.com"#, true),
            ("email@[127.0.0.1]", true),
            ("email@[2001:dB8::1]", true),
            ("email@[2001:dB8:0:0:0:0:0:1]", true),
            ("email@[::fffF:127.0.0.1]", true),
            ("email@127.0.0.1", true),
            ("example@valid-----hyphens.com", true),
            ("example@valid-with-hyphens.com", true),
            ("test@domain.with.idn.tld.उदाहरण.परीक्षा", true),
            // max length for domain name labels is 63 characters per RFC 1034
            ("a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", true),
            ("a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.atm", true),
            (
                "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbb.atm",
                true,
            ),

            // Invalid emails
            ("", false),
            ("abc", false),
            ("abc@", false),
            (r#""test@test"@example.com"#, false),
            // 64 * a
            ("a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", false),
            ("a @x.cz", false),
            ("abc@.com", false),
            ("something@@somewhere.com", false),
            ("email@[127.0.0.256]", false),
            ("email@[2001:db8::12345]", false),
            ("email@[2001:db8:0:0:0:0:1]", false),
            ("email@[::ffff:127.0.0.256]", false),
            ("example@invalid-.com", false),
            ("example@-invalid.com", false),
            ("example@invalid.com-", false),
            ("example@inv-.alid-.com", false),
            ("example@inv-.-alid.com", false),
            (r#"test@example.com\n\n<script src="x.js">"#, false),
            (r#""\\\011"@here.com"#, false),
            (r#""\\\012"@here.com"#, false),
            ("trailingdot@shouldfail.com.", false),
            // Trailing newlines in username or domain not allowed
            ("a@b.com\n", false),
            ("a\n@b.com", false),
            (r#""test@test"\n@example.com"#, false),
            ("a@[127.0.0.1]\n", false),
            // underscores are not allowed
            ("John.Doe@exam_ple.com", false),
        ];

        for (input, expected) in tests {
            // println!("{} - {}", input, expected);
            assert_eq!(
                validate_email(input).is_ok(),
                expected,
                "Email `{}` was not classified correctly",
                input
            );
        }
    }
}
