use sport::item_list::ItemList;
use std::fs;
use std::path::Path;

fn main() {
    let dest_path = Path::new("./assets/items.json");
    let items_str = ItemList::export_json();
    fs::write(dest_path, items_str).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
