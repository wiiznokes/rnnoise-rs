use std::ffi::CString;
use std::os::raw::{c_int, c_void};
use std::path::Path;
use std::ptr::NonNull;

use rnnoise2_sys::{
    DenoiseState, RNNModel, rnnoise_create, rnnoise_destroy, rnnoise_get_frame_size,
    rnnoise_model_free, rnnoise_model_from_buffer, rnnoise_model_from_filename,
    rnnoise_process_frame,
};

pub struct Model {
    ptr: NonNull<RNNModel>,
}

impl Model {
    /// Load model from memory buffer
    pub fn from_buffer(buf: &[u8]) -> Option<Self> {
        let ptr =
            unsafe { rnnoise_model_from_buffer(buf.as_ptr() as *const c_void, buf.len() as c_int) };
        NonNull::new(ptr).map(|p| Self { ptr: p })
    }

    /// Load model from a path
    pub fn from_path(path: &Path) -> Option<Self> {
        let c_path = CString::new(path.as_os_str().as_encoded_bytes()).ok()?;

        let ptr = unsafe { rnnoise_model_from_filename(c_path.as_ptr()) };
        NonNull::new(ptr).map(|p| Self { ptr: p })
    }

    fn as_ptr(&self) -> *mut RNNModel {
        self.ptr.as_ptr()
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            rnnoise_model_free(self.ptr.as_ptr());
        }
    }
}

pub struct Denoiser {
    ptr: NonNull<DenoiseState>,
    frame_size: usize,
}

impl Denoiser {
    /// Create using default or custom model
    pub fn new(model: Option<&Model>) -> Option<Self> {
        let ptr =
            unsafe { rnnoise_create(model.map(|m| m.as_ptr()).unwrap_or(std::ptr::null_mut())) };

        let ptr = NonNull::new(ptr)?;

        let frame_size = unsafe { rnnoise_get_frame_size() } as usize;

        Some(Self { ptr, frame_size })
    }

    pub fn frame_size(&self) -> usize {
        self.frame_size
    }

    /// Denoise a frame of samples
    pub fn process(&mut self, input: &[f32], output: &mut [f32]) -> f32 {
        assert!(input.len() >= self.frame_size);
        assert!(output.len() >= self.frame_size);

        unsafe { rnnoise_process_frame(self.ptr.as_ptr(), output.as_mut_ptr(), input.as_ptr()) }
    }
}

impl Drop for Denoiser {
    fn drop(&mut self) {
        unsafe {
            rnnoise_destroy(self.ptr.as_ptr());
        }
    }
}
