
use std::process::Command;
use std::path::Path;
use std::env;

fn main() {
    let out = env::var( "OUT_DIR" ).unwrap();

    Command::new("clang").args( &[ "-arch", "x86_64", "-c", "src/curseswrap.c", "-lncurses", "-o" ] )
                         .arg( &format!( "{}/curseswrap.o", out ) )
                         .status()
                         .unwrap();

    Command::new( "libtool" ).args( &[ "-static", "curseswrap.o", "-o", "libcurseswrap.a" ] )
                             .current_dir( &Path::new( &out ) )
                             .status()
                             .unwrap();

    println!( "cargo:rustc-link-search=native={}", out );
    println!( "cargo:rustc-link-lib=static=curseswrap" );

}
