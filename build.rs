use std::env;

fn main(){
    if env::var("CARGO_FEATURE_ALT_LINKER").is_ok(){
        println!("cargo:rustc-link-arg=-Ttestlink.x");
    }else{
    	println!("cargo:rustc-link-arg=-Tlink.x");
    }
}
