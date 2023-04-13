pub fn translate_c_type_to_rust(c_type: &str) -> String {
    // match c_type {
    //     "float" => "f32".to_string(),
    //     "float *" => "std::ptr::null_mut()".to_string(),
    //     "int" => "i32".to_string(),
    //     "unsigned int" => "u32".to_string(),
    //     "unsigned char *" => "std::ptr::null_mut()".to_string(),
    //     "unsigned short *" => "std::ptr::null_mut()".to_string(),
    //     "bool" => "bool".to_string(),
    //     "void *" => "std::ptr::null_mut()".to_string(),
    //     "Texture" => "crate::structs::Texture".to_string(),
    //     "Rectangle" => "crate::structs::Rectangle".to_string(),
    //     "Rectangle *" => "std::ptr::null_mut()".to_string(),
    //     "Image" => "crate::structs::Image".to_string(),
    //     "Texture2D" => "crate::structs::Texture2D".to_string(),
    //     "GlyphInfo *" => "std::ptr::null_mut()".to_string(),
    //     "Vector3" => "crate::structs::Vector3".to_string(),
    //     "Vector2" => "crate::structs::Vector2".to_string(),
    //     "Color" => "crate::structs::Color".to_string(),
    //     "float[4]" => "[f32; 4]".to_string(),
    //     "MaterialMap *" => "std::ptr::null_mut()".to_string(),
    //     "Quaternion" => "crate::structs::Quaternion".to_string(),
    //     "char[32]" => "[u8; 32]".to_string(),
    //     _ => {panic!("Unknown C type: {}", c_type)}
    // }

    // Translate basic C types to their Rust equivalents
    if let Some(ty) = match c_type {
        "char" => Some("i8"),
        "unsigned char" => Some("u8"),
        "short" => Some("i16"),
        "unsigned short" => Some("u16"),
        "int" => Some("i32"),
        "unsigned int" => Some("u32"),
        "long" => Some("i64"),
        "unsigned long" => Some("u64"),
        "float" => Some("f32"),
        "double" => Some("f64"),
        "bool" => Some("bool"),
        "char *" => Some("String"),
        "void *" => Some("*mut libc::c_void"),
        _ => None,
    } {
        return ty.to_string();
    }

    // For some reason, an internal data type is exposed.
    if c_type == "rAudioBuffer *" || c_type == "rAudioProcessor *" {
        return "std::ptr::null_mut()".to_string();
    }


    // If the type ends with a *, it's a pointer to an array of the type without the *
    if c_type.ends_with("*") {
        let ty = &c_type[..c_type.len() - 1].trim();
        return format!("Vec<{}>", translate_c_type_to_rust(ty));
    }

    // If the type ends with `[N]`, it's an array of N elements of the type without the `[N]`
    let arr_len_re = regex::Regex::new(r"\[(\d+)\]$").unwrap();
    if let Some(caps) = arr_len_re.captures(c_type) {
        let ty = &c_type[..c_type.len() - caps[0].len()].trim();
        let len = &caps[1];
        return format!("[{}; {}]", translate_c_type_to_rust(ty), len);
    }

    // Uppercase types are assumed to be structs
    if c_type.chars().next().unwrap().is_uppercase() {
        return format!("crate::structs::{}", c_type);
    }

    panic!("Unknown C type: {}", c_type)
}
