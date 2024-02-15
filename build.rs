fn main() {
    println!("cargo:rerun-if-changed=src/ctranslate2/mod.rs");
    println!("cargo:rerun-if-changed=src/ctranslate2/wrapper.c");
    println!("cargo:rerun-if-changed=src/ctranslate2/wrapper.h");
    let mut wrapper = cxx_build::bridge("src/ctranslate2/mod.rs");
    wrapper.file("src/ctranslate2/wrapper.c").std("c++20");
    #[cfg(feature = "static")]
    {
        let out_dir =
            std::env::var("OUT_DIR").expect("Cannot find $OUT_DIR, which should be set by cargo");
        cmake::Config::new(env!("ONEDNN_SRC"))
            .define("CMAKE_DISABLE_FIND_PACKAGE_GIT", "TRUE")
            .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", out_dir.clone())
            .define("ONEDNN_LIBRARY_TYPE", "STATIC")
            .define("ONEDNN_BUILD_EXAMPLES", "OFF")
            .define("ONEDNN_BUILD_TESTS", "OFF")
            .define("ONEDNN_EXPERIMENTAL", "ON")
            .build();
        cmake::Config::new(env!("CTRANSLATE2_SRC"))
            .define("BUILD_CLI", "OFF")
            .define("BUILD_SHARED_LIBS", "OFF")
            .define("CMAKE_ARCHIVE_OUTPUT_DIRECTORY", out_dir.clone())
            .define("OPENMP_RUNTIME", "COMP")
            .define("WITH_DNNL", "ON")
            .define("WITH_MKL", "OFF")
            .build();
        println!("cargo:rustc-link-arg=-fopenmp");
        println!("cargo:rustc-link-lib=static=dnnl");
        println!("cargo:rustc-link-lib=static=cpu_features");
        println!("cargo:rustc-link-lib=static=ctranslate2");
        wrapper.include(std::path::Path::new(out_dir.as_str()).join("include"));
    }
    #[cfg(not(feature = "static"))]
    println!("cargo:rustc-link-lib=dylib=ctranslate2");
    wrapper.compile("ctranslate2-spoilers");
}
