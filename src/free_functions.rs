use crate::{
    ffi::{self, prelude::*},
    Blob, Image, Result, ScratchImage, TexMetadata, CNMAP_FLAGS, DDS_FLAGS, DXGI_FORMAT,
    TEX_COMPRESS_FLAGS, TEX_FILTER_FLAGS, TEX_PMALPHA_FLAGS,
};

pub fn save_dds(images: &[Image], metadata: &TexMetadata, flags: DDS_FLAGS) -> Result<Blob> {
    let mut result = Blob::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_SaveToDDSMemory2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            flags,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Resize the image to width x height. Defaults to Fant filtering.
///
/// Note for a complex resize, the result will always have mipLevels == 1
pub fn resize(
    images: &[Image],
    metadata: &TexMetadata,
    width: usize,
    height: usize,
    filter: TEX_FILTER_FLAGS,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_Resize2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            width,
            height,
            filter,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Convert the image to a new format
pub fn convert(
    images: &[Image],
    metadata: &TexMetadata,
    format: DXGI_FORMAT,
    filter: TEX_FILTER_FLAGS,
    threshold: f32,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_Convert2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            format,
            filter,
            threshold,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Converts the image from a planar format to an equivalent non-planar format
pub fn convert_to_single_plane(images: &[Image], metadata: &TexMetadata) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_ConvertToSinglePlane2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// levels of '0' indicates a full mipchain, otherwise is generates that number of total levels (including the source base image)
///
/// Defaults to Fant filtering which is equivalent to a box filter
pub fn generate_mip_maps(
    images: &[Image],
    metadata: &TexMetadata,
    filter: TEX_FILTER_FLAGS,
    levels: usize,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_GenerateMipMaps2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            filter,
            levels,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// levels of '0' indicates a full mipchain, otherwise is generates that number of total levels (including the source base image)
///
/// Defaults to Fant filtering which is equivalent to a box filter
pub fn generate_mip_maps_3d(
    images: &[Image],
    metadata: Option<&TexMetadata>,
    filter: TEX_FILTER_FLAGS,
    levels: usize,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = if let Some(metadata) = metadata {
        unsafe {
            ffi::DirectXTexFFI_GenerateMipMaps3D2(
                images.as_ffi_ptr(),
                images.len(),
                metadata.into(),
                filter,
                levels,
                (&mut result).into(),
            )
        }
    } else {
        unsafe {
            ffi::DirectXTexFFI_GenerateMipMaps3D1(
                images.as_ffi_ptr(),
                images.len(),
                filter,
                levels,
                (&mut result).into(),
            )
        }
    };
    hr.success(result)
}

pub fn scale_mip_maps_alpha_for_coverage(
    images: &[Image],
    metadata: &TexMetadata,
    item: usize,
    alpha_reference: f32,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_ScaleMipMapsAlphaForCoverage(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            item,
            alpha_reference,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Converts to/from a premultiplied alpha version of the texture
pub fn premultiply_alpha(
    images: &[Image],
    metadata: &TexMetadata,
    flags: TEX_PMALPHA_FLAGS,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_PremultiplyAlpha2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            flags,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Note that threshold is only used by BC1. TEX_THRESHOLD_DEFAULT is a typical value to use
pub fn compress(
    images: &[Image],
    metadata: &TexMetadata,
    format: DXGI_FORMAT,
    compress: TEX_COMPRESS_FLAGS,
    threshold: f32,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_Compress2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            format,
            compress,
            threshold,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Compresses to BC6H or BC7 with DirectXTex's DirectCompute encoder, the D3D11
/// overload of [`Compress`](https://github.com/microsoft/DirectXTex/wiki/Compress).
///
/// `device` is an `ID3D11Device*`, as from the `windows` crate's
/// `ID3D11Device::as_raw`. A null device fails with `E_INVALIDARG`, and any other
/// format than BC6H or BC7 fails too; use [`compress`] for those. `alpha_weight`
/// is only used by BC7; [`TEX_ALPHA_WEIGHT_DEFAULT`](crate::TEX_ALPHA_WEIGHT_DEFAULT)
/// is a typical value to use.
///
/// # Safety
/// `device` must be null or a live `ID3D11Device` for the whole call. DirectXTex
/// neither takes nor releases a reference to it. It encodes through the device's
/// immediate context, which is not thread-safe, so nothing else may use that
/// context during the call.
#[cfg(windows)]
pub unsafe fn compress_gpu(
    device: *mut core::ffi::c_void,
    images: &[Image],
    metadata: &TexMetadata,
    format: DXGI_FORMAT,
    compress: TEX_COMPRESS_FLAGS,
    alpha_weight: f32,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_Compress3(
            device,
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            format,
            compress,
            alpha_weight,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

pub fn decompress(
    images: &[Image],
    metadata: &TexMetadata,
    format: DXGI_FORMAT,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_Decompress2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            format,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

pub fn compute_normal_map(
    images: &[Image],
    metadata: &TexMetadata,
    flags: CNMAP_FLAGS,
    amplitude: f32,
    format: DXGI_FORMAT,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_ComputeNormalMap2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            flags,
            amplitude,
            format,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

#[cfg(all(test, windows))]
mod tests {
    use crate::{
        compress_gpu, ScratchImage, CP_FLAGS_NONE, DXGI_FORMAT, TEX_ALPHA_WEIGHT_DEFAULT,
        TEX_COMPRESS_DEFAULT,
    };
    use core::ptr;

    /// The GPU encoder is compiled and linked; without a device DirectXTex
    /// rejects the call before touching D3D11.
    #[test]
    fn compress_gpu_rejects_a_null_device() {
        let mut source = ScratchImage::default();
        source
            .initialize_2d(DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM, 8, 8, 1, 1, CP_FLAGS_NONE)
            .unwrap();
        // SAFETY: a null device is allowed and only fails the call.
        let result = unsafe {
            compress_gpu(
                ptr::null_mut(),
                source.images(),
                source.metadata(),
                DXGI_FORMAT::DXGI_FORMAT_BC7_UNORM,
                TEX_COMPRESS_DEFAULT,
                TEX_ALPHA_WEIGHT_DEFAULT,
            )
        };
        // E_INVALIDARG
        assert_eq!(result.unwrap_err().into_underlying(), 0x8007_0057);
    }
}
