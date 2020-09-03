//use std::{env, path::*};
//use std::process::Command;
extern crate os_info;
extern crate reqwest;
use bzip2::read::BzDecoder;
use tar::Archive;
use std::env;
use std::fs::File;
use std::io;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    /*let arch = match os_info::get().os_type() {
        os_info::Type::Macos => "osx-64",
        os_info::Type::Windows => "win-64",
        _ => "linux-64",
    };
    */
    let url = format!("https://anaconda.org/intel/mkl-static/2020.3/download/win-64/mkl-static-2020.3-intel_279.tar.bz2");
    let archive = format!("{}/mkl-static-2020.3-intel_279.tar.bz2", out_dir);

    let mut out = File::create(&archive).expect("failed to create mkl archive");
    let mut resp = reqwest::blocking::get(&url).expect("request failed");
    io::copy(&mut resp, &mut out).expect("failed to write archive");
    //println!("cargo:rustc-link-search={}/lib", out_dir);
    /*if !Path::new(&format!("{}/lib/libmkl_core.a", &out_dir)).exists() ||
       !Path::new(&format!("{}/lib/libmkl_sequential.a", &out_dir)).exists() ||
       !Path::new(&format!("{}/lib/libmkl_intel_ilp64.a", &out_dir)).exists() {
        Command::new("wget")
                .arg("https://anaconda.org/intel/mkl-static/2020.2/download/win-64/mkl-static-2020.2-intel_254.tar.bz2")
                .args(&["-P", &out_dir]) 
                .status().unwrap();

        Command::new("tar")
                .arg("-xvf")
                .arg(&format!("{}/mkl-static-2020.2-intel_254.tar.bz2", out_dir))
                .args(&["-C", &out_dir])
                .status().unwrap();
    }
    */
    // TODO: make this crossplatform?
    let archive_file = File::open(&archive).expect("Could not open archive");
    let tar = BzDecoder::new(archive_file);
    let mut _archive = Archive::new(tar);
    _archive.unpack(&out_dir).expect("uh-oh");
    
    println!("cargo:rustc-link-search={}/Library/lib", out_dir);
    //println!("cargo:rustc-link-lib=static-nobundle=mkl_intel_ilp64");
    println!("cargo:rustc-link-lib=static-nobundle=mkl_intel_lp64");
    println!("cargo:rustc-link-lib=static-nobundle=mkl_sequential");
    println!("cargo:rustc-link-lib=static-nobundle=mkl_core");
    
}
