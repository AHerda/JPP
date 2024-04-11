fn main() {
    cc::Build::new()
        //.file("../zad1-c/lib_iter.c")
        .file("../zad1-c/lib_recur.c")
        .compile("iter");
}
