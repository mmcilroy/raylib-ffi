use super::{raylib_api::RayLibApiDefinition, type_xlat::translate_c_type_to_rust};


pub fn wrap_exposed_structs(api_defs: RayLibApiDefinition) {
    // Allocate an output buffer for lines
    let mut lines = Vec::new();

    // Handle each struct
    for st in api_defs.structs {
        // Write a doc comment with raylib's provided struct description
        lines.push("".to_string());
        lines.push(format!("/// {}", st.description));

        // Write the struct definition
        lines.push(format!("#[repr(C)]"));
        lines.push(format!("pub struct {} {{", st.name));

        // Write each field
        for field in st.fields {
            // Write a doc comment with raylib's provided field description
            lines.push(format!("    /// {}", field.description));

            // Write the field definition
            lines.push(format!("    pub {}: {},", field.name, translate_c_type_to_rust(&field.kind)));
        }

        // Close the struct definition
        lines.push(format!("}}"));
    }

    // Write the output file
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = format!("{}/structs.rs", out_dir);
    std::fs::write(out_path, lines.join("\n")).unwrap();
}
