fn main() {
    // This runs new migrations on cargo build
    println!("cargo:rerun-if-changed=migrations");
}