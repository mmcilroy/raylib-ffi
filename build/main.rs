use crate::wrap::{
    colors::wrap_default_colors, enums::wrap_exposed_enums, profiling::auto_profile_exported_fns,
    raylib_api::RayLibApiDefinition, structs::wrap_exposed_structs,
};

mod bind;
mod wrap;

pub fn main() {
    // Files to watch that should trigger a rebuild
    println!("cargo:rerun-if-changed=src/wrapper.h");

    // Compile raylib
    bind::compile_raylib("third_party/raylib");

    // Link libraries
    bind::link_libs();

    // Generate bindings
    bind::generate_bindings("src/wrapper.h");

    // Load the API definitions
    let api_defs =
        RayLibApiDefinition::load("third_party/raylib/parser/output/raylib_api.json").unwrap();

    // Generate safe wrappers
    wrap_exposed_enums(api_defs.clone());
    wrap_default_colors(api_defs.clone());
    wrap_exposed_structs(api_defs);

    // Make everything profile-able
    // auto_profile_exported_fns(&format!("{}/bindings.rs", std::env::var("OUT_DIR").unwrap()));
}
