use std::env;
use std::path::PathBuf;

// If any of these environment variables are set and are not "0", then we use
// some backup headers and the Embree lib is not linked. This is useful for
// providing documentation and running CI without requiring an Embree
// installation.
const USE_BACKUP_HEADERS_ENV_VARS: &[&str] = &["DOCS_RS", "CI"];

fn main() {
    // Based on https://rust-lang.github.io/rust-bindgen/tutorial-0.html

    // Determine if we should use the backup headers (if any of the environment
    // variables is set and not "0").
    let mut use_backup_headers = false;
    for env_var in USE_BACKUP_HEADERS_ENV_VARS {
        println!("cargo:rerun-if-env-changed={}", env_var);
        use_backup_headers =
            use_backup_headers || env::var_os(env_var).filter(|v| v != "0").is_some();
    }

    // Select a wrapper header and link to the Embree lib.
    let wrapper_path;
    if !use_backup_headers {
        wrapper_path = "wrapper.h";

        // If EMBREE_DIR is set, add it as a linker search path.
        println!("cargo:rerun-if-env-changed=EMBREE_DIR");
        if let Ok(embree_dir_str) = env::var("EMBREE_DIR") {
            let embree_lib_dir = PathBuf::from(embree_dir_str).join("lib");
            println!(
                "cargo:rustc-link-search=native={}",
                embree_lib_dir.display()
            );
        }

        // Link to the Embree lib.
        println!("cargo:rustc-link-lib=embree3");
    } else {
        wrapper_path = "backup_embree_headers/wrapper.h";

        // We are using the backup headers, so don't attempt to link to the
        // Embree lib.
    }

    // If wrapper.h changes, rebuild the crate.
    println!("cargo:rerun-if-changed={}", wrapper_path);

    // Generate the bindings.
    let bindings = bindgen::Builder::default()
        .header(wrapper_path)
        // If headers included in wrapper.h change, rebuild the crate.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate_comments(false)
        .trust_clang_mangling(false)
        .allowlist_function("rtc.*")
        .allowlist_type("RTC.*")
        .allowlist_var("RTC.*")
        .rustified_enum("RTCFormat")
        .rustified_enum("RTCBuildQuality")
        .rustified_enum("RTCDeviceProperty")
        .rustified_enum("RTCError")
        .rustified_enum("RTCBufferType")
        .rustified_enum("RTCGeometryType")
        .rustified_enum("RTCSubdivisionMode")
        .bitfield_enum("RTC.*Flags")
        .generate()
        .expect("Unable to generate Embree bindings.");

    // Extract the bindings source code, then clean up some names and types.
    // Note: It'd be nice to replace all patterns at once rather than using
    //       several replace() calls, but there's no point in including another
    //       build dependency (eg. regex) to handle this. The runtime of the
    //       replace() calls is dwarfed by bindgen anyway.
    let bindings_source = bindings
        .to_string()
        .replace("RTC_FORMAT_", "")
        .replace("RTC_BUILD_QUALITY_", "")
        .replace("RTC_DEVICE_PROPERTY_", "")
        .replace("RTC_ERROR_", "")
        .replace("RTC_BUFFER_TYPE_", "")
        .replace("RTC_GEOMETRY_TYPE_", "")
        .replace("RTC_SUBDIVISION_MODE_", "")
        .replace("RTC_INTERSECT_CONTEXT_FLAG_", "")
        .replace("RTC_CURVE_FLAG_", "")
        .replace("RTC_SCENE_FLAG_", "")
        .replace("RTC_BUILD_FLAG_", "")
        .replace(
            "pub type size_t = ::std::os::raw::c_ulong",
            "pub type size_t = usize",
        )
        .replace(
            "pub type __ssize_t = ::std::os::raw::c_long",
            "pub type __ssize_t = isize",
        );

    // Write the processed bindings.
    let out_file = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    std::fs::write(out_file, bindings_source).expect("Unable to save generated Embree bindings.");
}
