fn main() {
    cc::Build::new()
        .file("../zad1-c/lib_iter.c")
        .compile("iter");
}
