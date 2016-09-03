extern crate serde_codegen;

use std::path::Path;

fn main() {
    let src = Path::new("src/serde_types.in.rs");
    let dst = Path::new("src/serde_types.rs");
    serde_codegen::expand(&src, &dst).unwrap();
}
