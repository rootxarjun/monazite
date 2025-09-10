use std::env;
use std::path::PathBuf;

use c2a_bind_utils::bind_c2a_builder;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bind = bind_c2a_builder()
        .header("include/iflash.h")
        // FIXME: c2a-example 以下を参照していてよくない。専門家のテクで解決する
        .clang_arg("-I../../c2a-example/src")
        .generate()
        .expect("Unable to generate bindings!");
    bind.write_to_file(out_dir.join("iflash.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=include/iflash.h");
}
