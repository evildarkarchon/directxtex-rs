#![warn(clippy::pedantic)]

use cc::Build;
use std::path::Path;

fn make_standard_build() -> Build {
    let mut build = Build::new();
    build
        .cpp(true)
        .std("c++17")
        .warnings(false)
        .extra_warnings(false)
        .includes([
            "external/DirectX-Headers/include",
            "external/DirectXMath/Inc",
            "external/DirectXTex/DirectXTex",
        ]);

    if !cfg!(windows) {
        build.includes(["external/DirectX-Headers/include/wsl/stubs", "ffi/include"]);
    }

    let tool = build.get_compiler();
    if tool.is_like_gnu() {
        build.flag("-Wp,-w");
    }

    build
}

fn build_headers() {
    let root = Path::new("external/DirectX-Headers");
    let mut headers = make_standard_build();
    let mut guids = make_standard_build();
    let tool = headers.get_compiler();

    headers
        .include(root.join("include/directx"))
        .file(root.join("src/d3dx12_property_format_table.cpp"));
    guids
        .include(root.join("include/directx"))
        .file(root.join("src/dxguids.cpp"));

    if !cfg!(windows) && (tool.is_like_gnu() || tool.is_like_clang()) {
        headers.define("__REQUIRED_RPCNDR_H_VERSION__", "475");
        guids.define("__REQUIRED_RPCNDR_H_VERSION__", "475");
    }

    headers.compile("DirectX-Headers");
    guids.compile("DirectX-Guids");
}

fn build_tex() {
    let root = Path::new("external/DirectXTex");
    let mut build = make_standard_build();

    build.files(
        [
            "DirectXTex/BC.cpp",
            "DirectXTex/BC4BC5.cpp",
            "DirectXTex/BC6HBC7.cpp",
            "DirectXTex/DirectXTexCompress.cpp",
            "DirectXTex/DirectXTexConvert.cpp",
            "DirectXTex/DirectXTexDDS.cpp",
            "DirectXTex/DirectXTexHDR.cpp",
            "DirectXTex/DirectXTexImage.cpp",
            "DirectXTex/DirectXTexMipmaps.cpp",
            "DirectXTex/DirectXTexMisc.cpp",
            "DirectXTex/DirectXTexNormalMaps.cpp",
            "DirectXTex/DirectXTexPMAlpha.cpp",
            "DirectXTex/DirectXTexResize.cpp",
            "DirectXTex/DirectXTexTGA.cpp",
            "DirectXTex/DirectXTexUtil.cpp",
        ]
        .into_iter()
        .map(|x| root.join(x)),
    );

    if cfg!(windows) {
        build
            .files(
                [
                    "DirectXTex/BCDirectCompute.cpp",
                    "DirectXTex/DirectXTexCompressGPU.cpp",
                    "DirectXTex/DirectXTexFlipRotate.cpp",
                    "DirectXTex/DirectXTexWIC.cpp",
                ]
                .into_iter()
                .map(|x| root.join(x)),
            )
            // The D3D11 compute-shader BC6H/BC7 encoder behind `compress_gpu`.
            // `BCDirectCompute.cpp` includes its 14 shaders by bare name.
            // Upstream does not commit them, so they are vendored prebuilt, as
            // DirectXTex's CMake option `USE_PREBUILT_SHADERS` expects; see
            // shaders/README.md.
            .include("shaders/compiled");
        // DirectXTexWIC calls COM (CoCreateInstance etc.), so the final binary needs
        // ole32. Ask Cargo to link the system import library rather than handing
        // "Ole32.lib" to cc as an object: that only worked while cc passed the bare
        // name to lib.exe/llvm-lib, which found it on `LIB`. Since cc 1.9.0
        // (rust-lang/cc-rs#2033) relative objects are resolved against the build
        // script's working directory, so the archive step fails with
        // "cannot open input file ...\directxtex-1.3.0\Ole32.lib".
        println!("cargo:rustc-link-lib=ole32");
    }

    // DirectXTex's own CMake (build/CompilerAndLinker.cmake) compiles with
    // /fp:fast on MSVC, and so does the vcpkg build CAO's parity oracle links.
    // Without it, the BC1-BC5 encoders' axis fit can resolve near-ties the
    // other way, flipping a block's endpoints and producing different bytes.
    if build.get_compiler().is_like_msvc() {
        build.flag("/fp:fast");
    }

    build.compile("DirectXTex");
}

fn build_ffi() {
    let root = Path::new("ffi");
    let mut build = make_standard_build();
    build.file(root.join("main.cpp")).include(root);
    if cfg!(windows) {
        build.define("CONFIG_WINDOWS", "1");
    }
    build.compile("directxtex-ffi");
}

fn main() {
    build_headers();
    build_tex();
    build_ffi();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=ffi/main.cpp");
    println!("cargo:rerun-if-changed=shaders/compiled");
}
