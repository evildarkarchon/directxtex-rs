# CAO fork of directxtex-rs

This fork exists for [Cathedral Assets Optimizer](https://github.com/evildarkarchon/CAO)
(CAO). CAO's Rust port must encode Textures with the same DirectXTex build as
its C++ parity oracle, and needs a few changes upstream has not released. CAO's
workspace points `[patch.crates-io] directxtex` at a pinned commit of the `cao`
branch. That way `ba2`, which also depends on `directxtex`, links the same copy.

The fork stays on the 1.x line, so the patch still satisfies `ba2`'s
`directxtex = "1.1.0"` requirement.

## Upstream base

- Repository: <https://github.com/Ryan-rsm-McKenzie/directxtex-rs>
- Base: tag `v1.3.0`, commit `8e44d15166acb5c5d03864ac58e593da8f8e555b`
  (published to crates.io as `directxtex` 1.3.0 on 2025-01-08).

`main` mirrors upstream. All CAO changes live on the `cao` branch, as commits on
top of the base.

## Delta

| Change | Why |
| --- | --- |
| `external/DirectXTex`: `oct2024` (`9260384a`) → `may2026` (`4feb3e11`) | **Mandatory.** The C++ oracle's vcpkg build pins DirectXTex `may2026`. Parity needs both builds on the same library. |
| `external/DirectXMath`: `oct2024` (`0d821781`) → `jun2026` (`93e6399d`) | Matches the DirectXMath that the oracle's vcpkg baseline (`127402f1`) builds DirectXTex against. |
| `build.rs`: DirectXTex's sources compile with `/fp:fast` on MSVC | DirectXTex's own CMake does, so the oracle's vcpkg build does too. Without it the CPU BC1–BC5 encoders can orient a block's endpoints the other way on near-ties, and CAO's BC3 output is not byte-identical to the oracle's (CAO #491). |
| `unsafe impl Send for ScratchImage` | CAO creates and transforms Textures on its Run Worker thread. The pointers are uniquely owned CRT allocations, and the `SAFETY` comment in `src/scratch_image.rs` explains why moving them is sound. `Sync` is not added. |
| Version `1.3.0+cao.2` | Build metadata marks the fork in `Cargo.lock`. SemVer matching ignores it, so `^1.1.0` and `^1.3.0` requirements still match. |
| This file | Records the base and the delta. |

`external/DirectX-Headers` stays at upstream's `v1.614.1` (`48a76297`). On
Windows the oracle builds DirectXTex without DirectX-Headers, and only the
crate's own helper sources compile against them.

The public Rust API is unchanged apart from the `Send` impl. Between the two
releases, `DirectXTex.h` only adds functions and flags and adds
`DIRECTX_TEX_API` decorations. No existing enum or flag value changes, and the
crate's layout tests still pass.

## Verification

Only Windows with MSVC is verified, because that is CAO's only target. Upstream's
Linux and macOS CI runs only on `main` and was not run against this delta.

## Planned

- GPU compression (CAO #495): compile `BCDirectCompute.cpp` and
  `DirectXTexCompressGPU.cpp` with the 14 prebuilt shaders vendored, and add a
  `compress_gpu` wrapper over the D3D11 `Compress` overload.

## Updating

1. Rebase `cao` onto the new upstream base, or move a submodule, and update the
   tables above.
2. Bump the `+cao.N` build metadata.
3. Push, then update the `rev` in CAO's root `Cargo.toml` and run
   `cargo update -p directxtex` there.
