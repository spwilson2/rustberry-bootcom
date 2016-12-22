extern crate gcc;

fn main() {
        gcc::Config::new()
        .cpp(true) // Switch to C++ library compilation.
        .file("src/raspbootcom.cc")
        .compile("libother.a");
}
