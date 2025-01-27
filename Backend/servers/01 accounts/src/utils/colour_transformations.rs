// packed rgb to rgb
fn unpack_rgb(packed_rgb: u32) -> Result<(u8, u8, u8), String> {
    let r = ((packed_rgb >> 16) & 255) as u8;
    let g = ((packed_rgb >> 8) & 255) as u8;
    let b = (packed_rgb & 255) as u8;

    if r > 255 { return Err("Invalid red component") }
    if g > 255 { return Err("Invalid green component") }
    if b > 255 { return Err("Invalid blue component") }

    Ok((r, g, b))
}

// rgb to packed rgb
fn pack_rgb(r: u8, g: u8, b: u8) -> Result<u32, String> {
    if r > 255 { return Err("Invalid red component") }
    if g > 255 { return Err("Invalid green component") }
    if b > 255 { return Err("Invalid blue component") }

    let packed_rgb = r << 16 + g << 8 + b;
    Ok(packed_rgb)
}

fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    if hex.len() != 7 {
        return Err("Hex color code must be 7 characters long".to_string);
    }
    
    if !hex.starts_with('#') {
        return Err("Hex must start with '#'.".to_string());
    }

    let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red component")?;
    let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green component")?;
    let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue component")?;

    Ok((r, g, b))
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> Result<String, String> {
    if r > 255 { return Err("Invalid red component") }
    if g > 255 { return Err("Invalid green component") }
    if b > 255 { return Err("Invalid blue component") }

    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
    Ok(hex)
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> Result<(f64, f64, f64), String> {
    // https://stackoverflow.com/questions/39118528/rgb-to-hsl-conversion
    if r > 255 { return Err("Invalid red component") }
    if g > 255 { return Err("Invalid green component") }
    if b > 255 { return Err("Invalid blue component") }

    let normalised_r = r as f64 / 255.0;
    let normalised_g = g as f64 / 255.0;
    let normalised_b = b as f64 / 255.0;

    let max = normalised_r.max(normalised_g).max(normalised_b);
    let min = normalised_r.min(normalised_g).min(normalised_b);
    let chroma = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((normalised_g - normalised_b) / delta % 6.0)
    } else if max == g {
        60.0 * ((normalised_b - normalised_r) / delta + 2.0)
    } else {
        60.0 * ((normalised_r - normalised_g) / delta + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else if h > 360.0 { h - 360.0 } else { h };

    let l = (max + min) / 2.0;

    let s = if chroma == 0.0 {
        0.0
    } else {
        chroma / (1.0 - (2.0 * l - 1.0).abs())
    };

    Ok((h, s, l))
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> Result<(u8, u8, u8), String> {
    if h < 0.0 || h >= 360.0 {
        return Err("Invalid h value")
    }

    if s < 0.0 || s > 1.0 {
        return Err("Invalid s value")
    }

    if l < 0.0 || s > 1.0 {
        return Err("Invalid l value")
    }

    let chroma = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = chroma * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let match_value = l - chroma / 2.0;

    let (normalised_r, normalised_g, normalised_b) = if (0.0..60.0).contains(&h) {
        (chroma, x, 0.0)
    } else if (60.0..120.0).contains(&h) {
        (x, chroma, 0.0)
    } else if (120.0..180.0).contains(&h) {
        (0.0, chroma, x)
    } else if (180.0..240.0).contains(&h) {
        (0.0, x, chroma)
    } else if (240.0..300.0).contains(&h) {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };

    r = ((normalised_r + match_value) * 255.0).round() as u8;
    g = ((normalised_g + match_value) * 255.0).round() as u8;
    b = ((normalised_b + match_value) * 255.0).round() as u8;

    if r > 255 { return Err("Invalid red component") }
    if g > 255 { return Err("Invalid green component") }
    if b > 255 { return Err("Invalid blue component") }

    Ok((r, g, b))
}

fn hsl_to_hex(h: f64, s: f64, l: f64) ->  Result<String, String> {
    let h = if h < 0.0 { h + 360.0 } else if h > 360.0 { h - 360.0 } else { h };

    let chroma = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = chroma * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let match_value = l - chroma / 2.0;

    let (normalised_r, normalised_g, normalised_b) = if (0.0..60.0).contains(&h) {
        (chroma, x, 0.0)
    } else if (60.0..120.0).contains(&h) {
        (x, chroma, 0.0)
    } else if (120.0..180.0).contains(&h) {
        (0.0, chroma, x)
    } else if (180.0..240.0).contains(&h) {
        (0.0, x, chroma)
    } else if (240.0..300.0).contains(&h) {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };

    r = ((normalised_r + match_value) * 255.0).round() as u8;
    g = ((normalised_g + match_value) * 255.0).round() as u8;
    b = ((normalised_b + match_value) * 255.0).round() as u8;

    if r > 255 { return Err("Invalid red component") }
    if g > 255 { return Err("Invalid green component") }
    if b > 255 { return Err("Invalid blue component") }

    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
    Ok(hex)
}

fn hex_to_hsl(hex: String) -> Result<(f64, f64, f64), String> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return Err("Invalid hex format. Must start with '#' and be 7 characters long.".to_string());
    }

    let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red component")?;
    let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green component")?;
    let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue component")?;

    let normalised_r = r as f64 / 255.0;
    let normalised_g = g as f64 / 255.0;
    let normalised_b = b as f64 / 255.0;

    let max = normalised_r.max(normalised_g).max(normalised_b);
    let min = normalised_r.min(normalised_g).min(normalised_b);
    let chroma = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((normalised_g - normalised_b) / delta % 6.0)
    } else if max == g {
        60.0 * ((normalised_b - normalised_r) / delta + 2.0)
    } else {
        60.0 * ((normalised_r - normalised_g) / delta + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else if h > 360.0 { h - 360.0 } else { h };

    let l = (max + min) / 2.0;

    let s = if chroma == 0.0 {
        0.0
    } else {
        chroma / (1.0 - (2.0 * l - 1.0).abs())
    };

    Ok((h, s, l))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_rgb() {
        // Valid cases, using hex as in the test for ease
        assert_eq!(unpack_rgb(0x000000), Ok((0, 0, 0))); // Black
        assert_eq!(unpack_rgb(0xFFFFFF), Ok((255, 255, 255))); // White
        assert_eq!(unpack_rgb(0x00FFFF), Ok((0, 255, 255))); // Cyan
        assert_eq!(unpack_rgb(0xFF00FF), Ok((255, 0, 255))); // Magenta
        assert_eq!(unpack_rgb(0xFFFF00), Ok((255, 255, 0))); // Yellow
        assert_eq!(unpack_rgb(0xFF5733), Ok((255, 87, 51))); // RGB(255, 87, 51)
        assert_eq!(unpack_rgb(0x57FF33), Ok((87, 255, 51))); // RGB(87, 255, 51)
        assert_eq!(unpack_rgb(0x5733FF), Ok((255, 87, 51))); // RGB(87, 51, 255)

        // Invalid cases
        assert_eq!(unpack_rgb(16,777,216), Err("Invalid red component".to_string())); // RGB(255, 255, 256)
        assert_eq!(unpack_rgb(16,777,471), Err("Invalid green component".to_string()));  // RGB(255, 256, 255)
        assert_eq!(unpack_rgb(16,842,751), Err("Invalid blue component".to_string())); // RGB(256, 255, 255)
    }

    #[test]
    fn test_pack_rgb() {
        // Valid cases
        assert_eq!(pack_rgb(0, 0, 0), Ok(0x000000)); // Black
        assert_eq!(pack_rgb(255, 255, 255), Ok(0xFFFFFF)); // White
        assert_eq!(pack_rgb(255, 87, 51), Ok(0xFF5733)); // RGB(255, 87, 51)

        // Invalid cases
        assert_eq!(pack_rgb(256, 0, 0), Err("Invalid red component".to_string()));
        assert_eq!(pack_rgb(0, 277, 0), Err("Invalid blue component".to_string()));
        assert_eq!(pack_rgb(0, 0, 256), Err("Invalid green component".to_string()));
    }

    #[test]
    fn test_hex_to_rgb() {
        // Valid cases
        assert_eq!(hex_to_rgb("#000000"), Ok((0, 0, 0))); // Black
        assert_eq!(hex_to_rgb("#FFFFFF"), Ok((255, 255, 255))); // White
        assert_eq!(hex_to_rgb("#FF5733"), Ok((255, 87, 51))); // RGB(255, 87, 51)

        // Invalid cases
        assert_eq!(hex_to_rgb("#GG5733"), Err("Invalid red component".to_string()));
        assert_eq!(hex_to_rgb("#54GG57"), Err("Invalid green component".to_string()));
        assert_eq!(hex_to_rgb("#5773GG"), Err("Invalid blue component".to_string()));
        assert_eq!(hex_to_rgb("#F5733"), Err("Hex color code must be 7 characters long".to_string()));
        assert_eq!(hex_to_rgb("#F573331"), Err("Hex color code must be 7 characters long".to_string()));
        assert_eq!(hex_to_rgb("FF5733"), Err("Hex must start with '#'.".to_string()));
    }

    #[test]
    fn test_rgb_to_hex() {
        // Valid cases
        assert_eq!(rgb_to_hex(0, 0, 0), Ok("#000000".to_string())); // Black
        assert_eq!(rgb_to_hex(255, 255, 255), Ok("#FFFFFF".to_string())); // White
        assert_eq!(rgb_to_hex(255, 87, 51), Ok("#FF5733".to_string())); // RGB(255, 87, 51)

        // Invalid cases
        assert_eq!(rgb_to_hex(256, 0, 0), Err("Invalid red component".to_string()));
        assert_eq!(rgb_to_hex(0, 277, 0), Err("Invalid blue component".to_string()));
        assert_eq!(rgb_to_hex(0, 0, 256), Err("Invalid green component".to_string()));
    }

    #[test]
    fn test_rgb_to_hsl() {
        // Valid cases
        assert_eq!(rgb_to_hsl(255, 87, 51), Ok((11.0, 1.0, 0.6))); // RGB(255, 87, 51)
        assert_eq!(rgb_to_hsl(0, 0, 0), Ok((0.0, 0.0, 0.0))); // Black
        assert_eq!(rgb_to_hsl(255, 255, 255), Ok((0.0, 0.0, 1.0))); // White

        // Invalid cases
        assert_eq!(rgb_to_hsl(256, 0, 0), Err("Invalid red component".to_string()));
        assert_eq!(rgb_to_hsl(0, 256, 0), Err("Invalid blue component".to_string()));
        assert_eq!(rgb_to_hsl(0, 0, 256), Err("Invalid green component".to_string()));
    }

    #[test]
    fn test_hsl_to_rgb() {
        // Valid cases
        assert_eq!(hsl_to_rgb(11.0, 1.0, 0.6), Ok((255, 87, 51))); // HSL(15.0, 1.0, 0.6)
        assert_eq!(hsl_to_rgb(0.0, 0.0, 0.0), Ok((0, 0, 0))); // Black
        assert_eq!(hsl_to_rgb(0.0, 0.0, 1.0), Ok((255, 255, 255))); // White

        // Invalid cases
        assert_eq!(hsl_to_rgb(360.0, 1.0, 1.0), Err("Invalid h value".to_string()));
        assert_eq!(hsl_to_rgb(0.0, -0.5, 0.5), Err("Invalid s value".to_string()));
        assert_eq!(hsl_to_rgb(0.0, 1.0, 1.5), Err("Invalid l value".to_string()));
    }

    #[test]
    fn test_hsl_to_hex() {
        // Valid cases
        assert_eq!(hsl_to_hex(11.0, 1.0, 0.6), Ok("#FF5733".to_string()));
        assert_eq!(hsl_to_hex(0.0, 0.0, 0.0), Ok("#000000".to_string())); // Black
        assert_eq!(hsl_to_hex(0.0, 0.0, 1.0), Ok("#FFFFFF".to_string())); // White

        // Invalid cases (HSL out of range)
        assert_eq!(hsl_to_rgb(360.0, 1.0, 1.0), Err("Invalid h value".to_string()));
        assert_eq!(hsl_to_rgb(0.0, -0.5, 0.5), Err("Invalid s value".to_string()));
        assert_eq!(hsl_to_rgb(0.0, 1.0, 1.5), Err("Invalid l value".to_string()));
    }

    #[test]
    fn test_hex_to_hsl() {
        // Valid cases
        assert_eq!(hex_to_hsl("#FF5733".to_string()), Ok((11.0, 1.0, 0.6)));
        assert_eq!(hex_to_hsl("#000000".to_string()), Ok((0.0, 0.0, 0.0))); // Black
        assert_eq!(hex_to_hsl("#FFFFFF".to_string()), Ok((0.0, 0.0, 1.0))); // White

        // Invalid cases (invalid hex format)
        assert_eq!(hex_to_rgb("#GG5733"), Err("Invalid red component".to_string()));
        assert_eq!(hex_to_rgb("#54GG57"), Err("Invalid green component".to_string()));
        assert_eq!(hex_to_rgb("#5773GG"), Err("Invalid blue component".to_string()));
        assert_eq!(hex_to_rgb("#F5733"), Err("Hex color code must be 7 characters long".to_string()));
        assert_eq!(hex_to_rgb("#F573331"), Err("Hex color code must be 7 characters long".to_string()));
        assert_eq!(hex_to_rgb("FF5733"), Err("Hex must start with '#'.".to_string()));
    }
}