
use quote::{ToTokens, quote};
use syn::{
    parse::Parse,
    Token,
    Ident,
    LitStr,
};
use crate::fmt::{
    check_format,
    format,
    CheckFormatError,
};

#[derive(Debug, Default, Clone, Copy)]
pub enum StrOrInt {
    #[default]
    Str,
    Int,
}

impl Parse for StrOrInt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let word = input.parse::<Ident>()?;
        if word == "str" {
            Ok(Self::Str)
        } else if word == "int" {
            Ok(Self::Int)
        } else {
            Err(syn::Error::new(input.span(), "Unexpected input."))
        }
    }
}

#[derive(Default)]
pub enum VersionInput {
    #[default]
    Full,
    Id,
    // package_version!(patch);         // &'static str (default)
    // package_version!(patch, int);    // integer literal
    Patch(StrOrInt),
    Minor(StrOrInt),
    Major(StrOrInt),
    Prerelease,
    FmtLit(String),
}

impl Parse for VersionInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::Full);
        }

        if input.peek(LitStr) {
            let lit = input.parse::<LitStr>().unwrap();
            let lit_val = lit.value();
            let check_result = check_format(&lit_val, |s| {
                match s {
                      "package"
                    | "full"
                    | "id"
                    | "major"
                    | "minor"
                    | "patch"
                    | "prerelease" | "pre"  => Ok(()),
                    invalid => Err(CheckFormatError::InvalidInterpolation(invalid)),
                }
            });
            match check_result {
                Ok(()) => {},
                Err(CheckFormatError::UnclosedBrace(index)) => return Err(syn::Error::new(lit.span(), format!("Unclosed brace at index {index}."))),
                Err(CheckFormatError::InvalidInterpolation(interp)) => return Err(syn::Error::new(lit.span(), format!("Invalid interpolation: {interp:?}"))),
                Err(CheckFormatError::Io(err)) => return Err(syn::Error::new(lit.span(), format!("IO Error: {err}"))),
            }
            Ok(Self::FmtLit(lit_val))
        } else if input.peek(Ident) {
            let word_ident = input.parse::<Ident>().unwrap();
            let word = format!("{word_ident}");
            match word.as_str() {
                "full" => Ok(Self::Full),
                "id" => Ok(Self::Id),
                "patch" => {
                    if input.peek(Token![,]) {
                        _ = input.parse::<Token![,]>().unwrap();
                        let ty = input.parse::<StrOrInt>()?;
                        Ok(Self::Patch(ty))
                    } else {
                        Ok(Self::Patch(StrOrInt::Str))
                    }
                }
                "minor" => {
                    if input.peek(Token![,]) {
                        _ = input.parse::<Token![,]>().unwrap();
                        let ty = input.parse::<StrOrInt>()?;
                        Ok(Self::Minor(ty))
                    } else {
                        Ok(Self::Minor(StrOrInt::Str))
                    }
                }
                "major" => {
                    if input.peek(Token![,]) {
                        _ = input.parse::<Token![,]>().unwrap();
                        let ty = input.parse::<StrOrInt>()?;
                        Ok(Self::Major(ty))
                    } else {
                        Ok(Self::Major(StrOrInt::Str))
                    }
                }
                "prerelease" | "pre" => Ok(Self::Prerelease),
                _ => Err(syn::Error::new(word_ident.span(), "Unexpected word."))
            }
        } else {
            Err(syn::Error::new(input.span(), "Unexpected input."))
        }
    }
}

macro_rules! get_var {
    ($var:ident) => {
        get_var!(stringify!($var))
    };
    ($var:expr) => {
        std::env::var($var).expect(concat!("Failed to get ", $var, "."))
    };
}

fn get_package_name() -> String {
    get_var!(CARGO_PKG_NAME)
}

fn get_version_full() -> String {
    get_var!(CARGO_PKG_VERSION)
}

fn get_version_patch() -> String {
    get_var!(CARGO_PKG_VERSION_PATCH)
}

fn get_version_patch_int() -> u32 {
    get_version_patch().parse().expect("Failed to parse patch int.")
}

fn get_version_minor() -> String {
    get_var!(CARGO_PKG_VERSION_MINOR)
}

fn get_version_minor_int() -> u32 {
    get_version_minor().parse().expect("Failed to parse minor int.")
}

fn get_version_major() -> String {
    get_var!(CARGO_PKG_VERSION_MAJOR)
}

fn get_version_major_int() -> u32 {
    get_version_major().parse().expect("Failed to parse major int.")
}

fn get_version_prerelease() -> String {
    get_var!(CARGO_PKG_VERSION_PRE)
}

impl ToTokens for VersionInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            VersionInput::Full => {
                let full = get_version_full();
                tokens.extend(quote!( #full ));
            },
            VersionInput::Id => {
                let package = get_package_name();
                let full = get_version_full();
                let id = format!("{package}@{full}");
                tokens.extend(quote!( #id ));
            }
            VersionInput::Patch(StrOrInt::Int) => {
                let num = get_version_patch_int();
                tokens.extend(quote!( #num ));
            },
            VersionInput::Patch(StrOrInt::Str) => {
                let s = get_version_patch();
                tokens.extend(quote!( #s ));
            },
            VersionInput::Minor(StrOrInt::Int) => {
                let num = get_version_minor_int();
                tokens.extend(quote!( #num ));
            },
            VersionInput::Minor(StrOrInt::Str) => {
                let s = get_version_minor();
                tokens.extend(quote!( #s ));
            },
            VersionInput::Major(StrOrInt::Int) => {
                let num = get_version_major_int();
                tokens.extend(quote!( #num ));
            },
            VersionInput::Major(StrOrInt::Str) => {
                let s = get_version_major();
                tokens.extend(quote!( #s ));
            },
            VersionInput::Prerelease => {
                let s = get_version_prerelease();
                tokens.extend(quote!( #s ));
            },
            VersionInput::FmtLit(fmt) => {
                let result = format(fmt, |interp, sb| {
                    match interp {
                        "package" => {
                            let pkg = get_package_name();
                            sb.push_str(&pkg);
                        }
                        "full" => {
                            let full = get_version_full();
                            sb.push_str(&full);
                        }
                        "id" => {
                            let package = get_package_name();
                            let full = get_version_full();
                            sb.push_str(&package);
                            sb.push('@');
                            sb.push_str(&full);
                        }
                        "major" => {
                            let major = get_version_major();
                            sb.push_str(&major);
                        }
                        "minor" => {
                            let minor = get_version_minor();
                            sb.push_str(&minor);
                        }
                        "patch" => {
                            let patch = get_version_patch();
                            sb.push_str(&patch);
                        }
                        "prerelease" | "pre" => {
                            let pre = get_version_prerelease();
                            sb.push_str(&pre);
                        }
                        _ => return Err(CheckFormatError::InvalidInterpolation(interp))
                    }
                    Ok(())
                });
                match result {
                    Ok(formatted) => tokens.extend(quote!( #formatted )),
                    Err(CheckFormatError::InvalidInterpolation(interp)) => panic!("Invalid interpolation string: {interp:?}"),
                    Err(CheckFormatError::UnclosedBrace(index)) => panic!("Unclosed brace at index {index}."),
                    Err(CheckFormatError::Io(err)) => panic!("IO Error: {err}"),
                }
            },
        }
    }
}
