# Prebuilt DirectCompute shaders

`compiled/` holds the 14 compute shaders that DirectXTex's GPU BC6H/BC7 encoder
(`BCDirectCompute.cpp`, behind `compress_gpu`) includes by bare name: seven
entry points, each for `cs_5_0` and for `cs_4_0` (`_cs40`). Upstream DirectXTex
does not commit them; its CMake generates them with `fxc.exe`, or takes them
prebuilt with `USE_PREBUILT_SHADERS`. They are vendored here so building the
crate needs no shader compiler.

They were generated from DirectXTex `may2026` (`4feb3e11`) with `fxc.exe` from
Windows SDK 10.0.26100, by upstream's own script, and the `.pdb` files it also
writes were deleted:

```bat
set LegacyShaderCompiler=C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\fxc.exe
set CompileShadersOutput=<this repository>\shaders\compiled
cd external\DirectXTex\DirectXTex\Shaders
CompileShaders.cmd
```

Regenerate them whenever `external/DirectXTex` moves and its `Shaders/*.hlsl`
change.
