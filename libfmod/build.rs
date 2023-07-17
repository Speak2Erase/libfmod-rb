// Copyright (C) 2023 Lily Lyons
//
// This file is part of libfmod-rb.
//
// libfmod-rb is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// libfmod-rb is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with libfmod-rb.  If not, see <http://www.gnu.org/licenses/>.
use std::path::PathBuf;

fn find_fmod_dir() -> PathBuf {
    #[cfg(windows)]
    {
        for drive in ["C", "D"] {
            let test_path = PathBuf::from(format!(
                "{drive}:/Program Files (x86)/FMOD SoundSystem/FMOD Studio API Windows"
            ));
            if test_path.exists() {
                return test_path;
            }
        }

        if let Some(path) = std::env::var_os("LIBFMOD_FMOD_API_DIR") {
            let path = PathBuf::from(path);
            if path.exists() {
                return path;
            }
        }

        for path in ["./FMOD Studio API Windows", "./FMOD SoundSystem"] {
            let path = PathBuf::from(path)
                .canonicalize()
                .expect("failed to canonicalize fmod path");
            if path.exists() {
                return path;
            }
        }

        panic!("unable to find fmod api dir. please set LIBFMOD_FMOD_API_DIR to the path of fmod")
    }
    #[cfg(not(windows))]
    {
        todo!()
    }
}

fn main() {
    let api_dir = find_fmod_dir().join("api");
    let api_dir = api_dir.to_str().unwrap();

    println!("cargo:rerun-if-changed=\"{api_dir}/core/inc\"");
    println!("cargo:rerun-if-changed=\"{api_dir}/studio/inc\"");
    println!("cargo:rerun-if-changed=\"{api_dir}/fsbank/inc\"");

    let bindgen = bindgen::builder()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg(format!("-I{api_dir}/core/inc"))
        .clang_arg(format!("-I{api_dir}/studio/inc"))
        .clang_arg(format!("-I{api_dir}/fsbank/inc"))
        .header("src/wrapper.h");

    #[cfg(target_arch = "x86")]
    {
        println!("cargo:rustc-link-search={api_dir}/core/lib/x86");
        println!("cargo:rustc-link-search={api_dir}/studio/lib/x86");
        println!("cargo:rustc-link-search={api_dir}/fsbank/lib/x86");
    }
    #[cfg(target_arch = "x86_64")]
    {
        println!("cargo:rustc-link-search={api_dir}/core/lib/x64");
        println!("cargo:rustc-link-search={api_dir}/studio/lib/x64");
        println!("cargo:rustc-link-search={api_dir}/fsbank/lib/x64");
    }
    #[cfg(any(debug_assertions, feature = "force-debug"))]
    {
        #[cfg(target_env = "msvc")]
        {
            println!("cargo:rustc-link-lib=fmodL_vc");
            println!("cargo:rustc-link-lib=fmodstudioL_vc");
        }
        #[cfg(target_env = "gnu")]
        {
            println!("cargo:rustc-link-lib=fmodL");
            println!("cargo:rustc-link-lib=fmodstudioL");
        }
    }
    #[cfg(not(any(debug_assertions, feature = "force-debug")))]
    {
        #[cfg(target_env = "msvc")]
        {
            println!("cargo:rustc-link-lib=fmod_vc");
            println!("cargo:rustc-link-lib=fmodstudio_vc");
        }
        #[cfg(target_env = "gnu")]
        {
            println!("cargo:rustc-link-lib=fmod");
            println!("cargo:rustc-link-lib=fmodstudio");
        }
    }
    #[cfg(target_env = "msvc")]
    {
        println!("cargo:rustc-link-lib=fsbank_vc");
    }
    #[cfg(target_env = "gnu")]
    {
        println!("cargo:rustc-link-lib=fsbank");
    }

    let bindings = bindgen.generate().expect("failed to generate bindings");
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
