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

// Convert from hex to rgb
fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), String> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return Err("Invalid hex format. Must start with '#' and be 7 characters long.".to_string());
    }

    let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red component")?;
    let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green component")?;
    let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue component")?;

    Ok((r, g, b))
}

// Convert from rgb to hex
fn rgb_to_hex(r: u8, g: u8, b: u8) -> Result<String, String> {
    if r > 255 { return Err("Invalid red component") }
    if g > 255 { return Err("Invalid green component") }
    if b > 255 { return Err("Invalid blue component") }

    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
    Ok(hex)
}

// Convert from rgb to hsl
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

// Convert from hsl to rgb
fn hsl_to_rgb(h: f64, s: f64, l: f64) -> Result<(u8, u8, u8), String> {
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

    Ok((r, g, b))
}

// Convert from hsl to hex
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

// Convert from hex to hsl
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
