fn main() {
    // Build the assembly file and create a static archive the linker will use.
    cc::Build::new()
        .file("src/os_assembly.s")
        .compile("os_assembly");   
}