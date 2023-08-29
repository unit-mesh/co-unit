use std::{
    env, fs,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

fn main() {
    if env::var("ORT_DYLIB_PATH").is_err() {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let profile_dir = out_dir
            // "target/.../build/bloop-hash"
            .parent()
            .unwrap()
            // "target/.../build"
            .parent()
            .unwrap()
            // "target/.../"
            .parent()
            .unwrap();

        copy(profile_dir);
    } else {
        println!("cargo:rerun-if-env-changed=ORT_DYLIB_PATH");
    }
}

fn copy(profile_dir: &Path) {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let (dylib_name, target_path) = match target_os.as_str() {
        "macos" => {
            let name = "libonnxruntime.dylib";
            (name, Path::new(".").join("public").join(name))
        }
        "linux" => {
            let name = "libonnxruntime.so";
            (name, Path::new(".").join("public").join(name))
        }
        "windows" => return,
        other => panic!("unknown OS {other}"),
    };

    let dylib_path = profile_dir.join(dylib_name);
    wait_for(&dylib_path);
    println!("target: {target_path:?}, {:?}", env::current_dir());
    fs::copy(dylib_path, &target_path).unwrap();

    // copy model/* file to target_path
    let model_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("model");
    fs_extra::dir::copy(model_path, Path::new(".").join("public"), &fs_extra::dir::CopyOptions::new().overwrite(true)).unwrap();

    // copy domain example file to target_path
    let domain_path = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("domain");
    fs_extra::dir::copy(domain_path, Path::new(".").join("public"), &fs_extra::dir::CopyOptions::new().overwrite(true)).unwrap();
}

fn wait_for(dylib_path: &Path) {
    println!("waiting for: {dylib_path:?}");
    for _ in 0..1000 {
        if dylib_path.exists() {
            return;
        }

        thread::sleep(Duration::from_millis(500));
    }

    panic!("timeout waiting for ort download");
}
