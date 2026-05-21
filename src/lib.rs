use std::{env, path::PathBuf};

use proc_macro::TokenStream;
use quote::quote;


#[proc_macro]
pub fn readme_text(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        panic!("Unexpected macro input.");
    }
    let Ok(readme_path) = env::var("CARGO_PKG_README") else {
        panic!("CARGO_PKG_README environment variable could not be resolved.");
    };
    let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") else {
        panic!("CARGO_MANIFEST_DIR environment variable could not be resolved.");
    };
    let manifest_dir = PathBuf::from(manifest_dir);
    let readme_path = manifest_dir.join(readme_path);
    if !readme_path.is_file() {
        panic!("README file was not found. {}", readme_path.display());
    }
    let Ok(readme_text) = std::fs::read_to_string(readme_path) else {
        panic!("Failed to read README file to string.");
    };
    let readme_text = readme_text.trim_end();
    quote!( #readme_text ).into()
}

#[proc_macro]
pub fn license_text(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        panic!("Unexpected macro input.");
    }
    let Ok(readme_path) = env::var("CARGO_PKG_LICENSE_FILE") else {
        panic!("CARGO_PKG_LICENSE_FILE environment variable could not be resolved.");
    };
    let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") else {
        panic!("CARGO_MANIFEST_DIR environment variable could not be resolved.");
    };
    let manifest_dir = PathBuf::from(manifest_dir);
    let license_path = manifest_dir.join(readme_path);
    if !license_path.is_file() {
        panic!("License file was not found: \"{}\"", license_path.display());
    }
    let Ok(license_text) = std::fs::read_to_string(license_path) else {
        panic!("Failed to read license file to string.");
    };
    let license_text = license_text.trim_end();
    quote!( #license_text ).into()
}
