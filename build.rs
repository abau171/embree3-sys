use std::env;
use std::path::PathBuf;

fn main() {

    // Based on https://rust-lang.github.io/rust-bindgen/tutorial-0.html

    // If EMBREE_DIR is set, add it as a linker search path.
    if let Ok(embree_dir_str) = env::var("EMBREE_DIR") {
        let embree_lib_dir = PathBuf::from(embree_dir_str).join("lib");
        println!("cargo:rustc-link-search=native={}", embree_lib_dir.display());
        println!("cargo:rerun-if-env-changed=EMBREE_DIR");
    }

    // Link to the Embree lib.
    println!("cargo:rustc-link-lib=embree3");

    // If wrapper.h changes, rebuild the crate.
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
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
    let bindings_source =
        bindings.to_string()
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
            "pub type size_t = usize")
        .replace(
            "pub type __ssize_t = ::std::os::raw::c_long",
            "pub type __ssize_t = isize");

    // Write the processed bindings
    let out_file =
        PathBuf::from(env::var("OUT_DIR").unwrap())
        .join("bindings.rs");
    std::fs::write(out_file, bindings_source)
        .expect("Unable to save generated Embree bindings.");
}
