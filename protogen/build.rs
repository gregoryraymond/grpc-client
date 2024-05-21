use walkdir::WalkDir;
use std::path::Path;
use std::fs::File;
use std::io::Read;

fn dynamic_replace(file: &Path) {
    let mut file = File::open(file).expect("Unable to open file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");
    for element in syntax.items {
        match element {
            syn::Item::Enum(x) => {

            },
            syn::Item::Struct(x) => {

            },
            syn::Item::Type(x) => {

            },
            syn::Item::Impl(x) => {
                for item in x {
                    match item {
                        syn::ImplItem::Fn(y) => {

                        },
                        syn::ImplImte::Type(y) => {
                            
                        }
                    }
                }
            },
            syn::Item::Fn(x) => {

            },
            _ => {}
        }
    }
}

pub fn main() {
    for entry in WalkDir::new("*.rs") {
        if let Ok(file) = entry {
            dynamic_replace(file.path());
        }
    }
}