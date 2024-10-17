fn main() {
    // Compiling C code and linking it
    cc::Build::new()
        .files(["c-files/functions.c", "c-files/struct.c"])
        .compile("build.c");

    cc::Build::new()
        .cpp(true) // Enable C++ compilation
        .file("cpp-files/functions.cpp")
        .compile("build.cpp");
}
