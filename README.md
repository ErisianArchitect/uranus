A simple library for getting the README or license text from the current package.

# Example
```rust
use uranus::{readme_text, license_text};

pub const README: &'static str = readme_text!();
pub const LICENSE: &'static str = license_text!();
```
