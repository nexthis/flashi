fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    // println!("cargo:rerun-if-changed=src/hello.c");
    cc::Build::new()
        .file("robot/mouse.c")
        .compile("robotjs");
}
