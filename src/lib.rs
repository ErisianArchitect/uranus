mod fmt;
mod version;

use std::{path::PathBuf};

use proc_macro::TokenStream;
use quote::quote;

use syn::parse_macro_input;
use version::*;

#[track_caller]
#[inline(always)]
fn expect_empty_input(input: &TokenStream) {
    if !input.is_empty() {
        panic!("Unexpected macro input.");
    }
}

macro_rules! var_not_found {
    ($var:expr) => {
        panic!(concat!($var, " environment variable could not be resolved."));
    };
    ($var:ident) => {
        var_not_found!(stringify!($var));
    };
}

macro_rules! get_var {
    ($binding:ident = $var:ident) => {
        let Ok($binding) = std::env::var(stringify!($var)) else {
            var_not_found!(stringify!($var));
        };
    };
    ($binding:ident = $var:ident) => {
        get_var!($binding = stringify!($var));
    };
}

fn get_readme_text() -> String {
    get_var!(readme_path = CARGO_PKG_README);
    get_var!(manifest_dir = CARGO_MANIFEST_DIR);
    
    let manifest_dir = PathBuf::from(manifest_dir);
    let readme_path = manifest_dir.join(readme_path
    );
    if !readme_path.is_file() {
        panic!("README file was not found: \"{}\"", readme_path.display());
    }
    
    let Ok(readme_text) = std::fs::read_to_string(readme_path) else {
        panic!("Failed to read README file to string.");
    };
    
    readme_text
}

fn get_license_text() -> String {
    get_var!(license_path = CARGO_PKG_LICENSE_FILE);
    get_var!(manifest_dir = CARGO_MANIFEST_DIR);
    let manifest_dir = PathBuf::from(manifest_dir);
    let license_path = manifest_dir.join(license_path);
    
    if !license_path.is_file() {
        panic!("License file was not found: \"{}\"", license_path.display());
    }
    
    let Ok(license_text) = std::fs::read_to_string(license_path) else {
        panic!("Failed to read license file to string.");
    };
    
    license_text
}

#[proc_macro]
pub fn readme_text(input: TokenStream) -> TokenStream {
    expect_empty_input(&input);
    
    let readme_text = get_readme_text();
    let readme_text = readme_text.trim_end();
    
    quote!( #readme_text ).into()
}

#[proc_macro]
pub fn license_text(input: TokenStream) -> TokenStream {
    expect_empty_input(&input);

    let license_text = get_license_text();
    let license_text = license_text.trim_end();
    
    quote!( #license_text ).into()
}

macro_rules! single_var_macros {
    (
        $(
            $(#[$attr:meta])*
            $fn_name:ident => $var_name:ident
        ),*
        $(,)?
    ) => {
        $(
            $(#[$attr])*
            #[proc_macro]
            pub fn $fn_name(input: TokenStream) -> TokenStream {
                expect_empty_input(&input);
                get_var!(single_var = $var_name);
                quote!( #single_var ).into()
            }
        )*
    };
}

// TODO: Add documentation (the macro supports attributes on each item)
single_var_macros!(
    /// Get the path to the currently executing instance of `cargo`.
    cargo_path => CARGO,
    /// Get the directory that the manifest (`Cargo.toml`) is in.
    manifest_dir => CARGO_MANIFEST_DIR,
    /// Get the path to the manifest (`Cargo.toml`).
    manifest_path => CARGO_MANIFEST_PATH,
    /// Get the package name field as defined in the package's manifest.
    package_name => CARGO_PKG_NAME,
    /// Get the package description field as defined in the package's manifest.
    /// 
    /// This will return an empty string if the field is not set.
    package_description => CARGO_PKG_DESCRIPTION,
    /// Get the package homepage field as defined in the package's manifest.
    /// 
    /// This will return an empty string if the field is not set.
    package_homepage => CARGO_PKG_HOMEPAGE,
    /// Get the package repository field as defined in the package's manifest.
    /// 
    /// This will return an empty string if the field is not set.
    package_repository => CARGO_PKG_REPOSITORY,
    /// Get the package license field as defined in the package's manifest.
    /// 
    /// This will return an empty string if the field is not set.
    package_license => CARGO_PKG_LICENSE,
    /// Get the package license-file field as defined in the package's manifest.
    /// 
    /// This will return an empty string if the field is not set.
    package_license_file => CARGO_PKG_LICENSE_FILE,
    /// Get the package rust-version field as defined in the package's manifest.
    /// 
    /// This will return an empty string if the field is not set.
    package_rust_version => CARGO_PKG_RUST_VERSION,
    /// Get the package readme field as defined in the package's manifest.
    /// 
    /// This will return an empty string if the field is not set.
    package_readme => CARGO_PKG_README,
    /// Get the crate name of the currently compiling crate.
    crate_name  => CARGO_CRATE_NAME,
    /// Get the bin name of the currently compiling binary.
    bin_name => CARGO_BINE_NAME,
);

/// Get package version information.
/// 
/// # Inputs:
/// - `package_version!(full)`
///   Get the full version info: `1.2.3-prerelease+build`
/// - `package_version!(id)`
///   Get the package ID spec: `package@1.2.3-prerelease+build`
/// - `package_version!(major)`
///   Get the version's major value as a string.
/// - `package_version!(major, int)`
///   Get the version's major value as an integer.
/// - `package_version!(minor)`
///   Get the version's minor value as a string.
/// - `package_version!(minor, int)`
///   Get the version's, minor value as an integer.
/// - `package_version!(patch)`
///   Get the version's patch value as a string.
/// - `package_version!(patch, int)`
///   Get the version's patch value as an integer.
/// - `package_version!(prerelease)` or `package_version!(pre)`
///   Get the version's prerelease value as a string.
/// 
/// # String Format
/// `package_version` also has the option to use a format literal.
/// ```rust, ignore
/// use uranus::package_version;
/// 
/// pub const VERSION_INFO: &'static str = package_version!("Package: {package}\nVersion: {full}\nPrerelease: {prerelease}")
/// ```
/// # String Format Interpolations
/// - `package`
///   The package name.
/// - `full`
///   The full version string.
/// - `id`
///   The package ID spec.
/// - `major`
///   The major version value.
/// - `minor`
///   The minor version value.
/// - `patch`
///   The patch version value.
/// - `prerelease` or `pre`
///   The prerelease version value.
#[proc_macro]
pub fn package_version(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as VersionInput);
    quote!( #input ).into()
}
