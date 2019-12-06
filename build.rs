#![allow(unused)]

use std::convert::AsRef;
use std::path::{PathBuf, Path};
use std::string::ToString;
use tar::Archive;
use flate2::read::GzDecoder;


///////////////////////////////////////////////////////////////////////////////
// UTILS - ENVIROMENT
///////////////////////////////////////////////////////////////////////////////

fn is_release_mode() -> bool {
    let value = std::env::var("PROFILE")
        .expect("missing PROFILE")
        .to_lowercase();
    &value == "release"
}

fn is_debug_mode() -> bool {
    let value = std::env::var("PROFILE")
        .expect("missing PROFILE")
        .to_lowercase();
    &value == "debug"
}

fn out_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR env var"))
}

///////////////////////////////////////////////////////////////////////////////
// UTILS - BUILD
///////////////////////////////////////////////////////////////////////////////

pub fn extract_tar_file<P: AsRef<Path>, Q: AsRef<Path>>(tar_file: P, dest: Q) -> Result<(), String> {
    let source = std::fs::read(tar_file).expect("read tar file");
    let tar = GzDecoder::new(&source[..]);
    let mut archive = Archive::new(tar);
    // UNPACK ARCHIVE
    let tmp_source_dir: Option<PathBuf> = {
        archive
            .unpack(&dest)
            .map_err(|x| format!("[{:?}] failed to unpack tar file: {:?}", dest.as_ref(), x))?;
        let xs = std::fs::read_dir(&dest)
            .expect(&format!("unable to read dir {:?}", dest.as_ref()))
            .filter_map(Result::ok)
            .filter(|file| file.file_type().map(|x| x.is_dir()).unwrap_or(false))
            .collect::<Vec<std::fs::DirEntry>>();
        match &xs[..] {
            [x] => Some(x.path()),
            _ => None,
        }
    };
    Ok(())
}

pub fn lookup_newest(paths: Vec<PathBuf>) -> Option<PathBuf> {
    use std::time::{SystemTime, Duration};
    let mut newest: Option<(PathBuf, Duration)> = None;
    paths
        .clone()
        .into_iter()
        .filter_map(|x: PathBuf| {
            let timestamp = x
                .metadata()
                .ok()
                .and_then(|y| y.created().ok())
                .and_then(|x| x.duration_since(SystemTime::UNIX_EPOCH).ok());
            match timestamp {
                Some(y) => Some((x, y)),
                _ => None
            }
        })
        .for_each(|(x_path, x_created)| match &newest {
            None => {
                newest = Some((x_path, x_created));
            }
            Some((_, y_created)) => {
                if &x_created > y_created {
                    newest = Some((x_path, x_created));
                }
            }
        });
    // DONE
    newest.map(|(x, _)| x)
}

pub fn files_with_prefix(dir: &PathBuf, pattern: &str) -> Vec<PathBuf> {
    std::fs::read_dir(dir)
        .expect(&format!("get dir contents: {:?}", dir))
        .filter_map(Result::ok)
        .filter_map(|x| {
            let file_name = x
                .file_name()
                .to_str()?
                .to_owned();
            if file_name.starts_with(pattern) {
                Some(x.path())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn run_make(source_path: &PathBuf, makefile: &str) {
    let result = std::process::Command::new("make")
        .arg("-C")
        .arg(source_path)
        .arg("-f")
        .arg(makefile)
        .output()
        .expect(&format!("make -C {:?} failed", source_path));
}

fn cpy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) {
    std::fs::copy(&from, &to)
        .expect(&format!(
            "unable to cpy from {:?} to {:?}",
            from.as_ref(),
            to.as_ref(),
        ));
}

///////////////////////////////////////////////////////////////////////////////
// PATHS
///////////////////////////////////////////////////////////////////////////////

pub const STATIC_LIBS: &[(&str, &str)] = &[
    ("libx264", "./libx264.a"),
];

pub const HEADERS: &[&str] = &[
    "x264.h",
];

///////////////////////////////////////////////////////////////////////////////
// BUILD PIPELINE
///////////////////////////////////////////////////////////////////////////////

fn build() {
    let out_path = out_dir();
    // // SETUP
    // extract_tar_file("archive/libwebp@v1.0.3.tar", &out_path);
    // let source_path = {
    //     let xs = files_with_prefix(&out_path, "webmproject-libwebp-");
    //     lookup_newest(xs).expect("extracted webp source files from tar archive")
    // };
    // // BUILD
    // run_make(&source_path, "makefile.unix");
    // // LINK
    // println!("cargo:rustc-link-search=native={}", {
    //     source_path
    //         .join("src")
    //         .to_str()
    //         .expect("PathBuf as str")
    // });
    // println!("cargo:rustc-link-search=native={}", {
    //     source_path
    //         .join("src/demux")
    //         .to_str()
    //         .expect("PathBuf as str")
    // });
    // // println!("cargo:rustc-link-search=native={}", {
    // //     source_path
    // //         .join("imageio")
    // //         .to_str()
    // //         .expect("PathBuf as str")
    // // });
    // for (name, _) in WEBP_STATIC_LIBS {
    //     println!("cargo:rustc-link-lib=static={}", name);
    // }
    // // for (name, _) in IMAGEIO_STATIC_LIBS {
    // //     println!("cargo:rustc-link-lib=static={}", name);
    // // }
    // // // * DYNAMIC LIBRARY DEPENDENCIES - TODO: PHASE OUT
    // // println!("cargo:rustc-link-lib=jpeg");
    // // println!("cargo:rustc-link-lib=png");
    // // CODEGEN
    // let codegen = |file_name: &str, headers: &[&str]| {
    //     let codegen = bindgen::Builder::default();
    //     let codegen = headers
    //         .iter()
    //         .fold(codegen, |codegen: bindgen::Builder, path: &&str| -> bindgen::Builder {
    //             let path: &str = path.clone();
    //             let path: PathBuf = source_path.join(path);
    //             let path: &str = path.to_str().expect("PathBuf to str");
    //             codegen.header(path)
    //         });
    //     codegen
    //         .generate_comments(true)
    //         .generate()
    //         .expect("Unable to generate bindings")
    //         .write_to_file(out_path.join(file_name))
    //         .expect("Couldn't write bindings!");    
    // };
    // codegen("bindings_webp.rs", WEBP_HEADERS);
    // // // codegen("bindings_imageio.rs", IMAGEIO_HEADERS);
    // // // COMPILE CBITS
    // // cc::Build::new()
    // //     .include({
    // //         source_path
    // //             .to_str()
    // //             .expect("PathBuf to str")
    // //     })
    // //     .include({
    // //         source_path
    // //             .join("src")
    // //             .to_str()
    // //             .expect("PathBuf to str")
    // //     })
    // //     .file("cbits.c")
    // //     .compile("cbits");
}

///////////////////////////////////////////////////////////////////////////////
// MAIN
///////////////////////////////////////////////////////////////////////////////

fn main() {

}