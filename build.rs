// build.rs
fn main() {
    println!("cargo:rerun-if-changed=src/memory.x");
}
