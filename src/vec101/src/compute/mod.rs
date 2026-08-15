use crate::core::vec101_context;
extern crate alloc;
#[cfg(feature = "std")]
pub mod batch;
#[doc = " Main compute dispatcher."]
#[doc = " # Safety"]
#[doc = " Caller must ensure that the provided context contains valid, aligned memory pointers."]
pub unsafe fn vec101_compute(ctx: &vec101_context) {
    if ctx.batch_size == 0 || ctx.num_rows == 0 {
        return;
    }
    use crate::hal::Vec101Backend;
    #[cfg(feature = "cuda")]
    {
        if !ctx.hardware_handle.is_null() {
            let device = unsafe {
                &*(ctx.hardware_handle as *const std::sync::Arc<cudarc::driver::CudaDevice>)
            };
            let backend = crate::gpu::cuda::CudaBackend::new(device.clone());
            backend.compute(ctx);
            return;
        }
    }
    #[cfg(feature = "gpu-metal")]
    {
        let backend = crate::gpu::metal::MetalBackend::new();
        backend.compute(ctx);
        return;
    }
    let backend = crate::hal::cpu::CpuBackend::new(ctx.num_threads);
    backend.compute(ctx);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec101_compute_early_exit() {
        let ctx = vec101_context {
            batch_size: 0,
            num_rows: 0,
            blocks_per_row: 0,
            num_threads: 0,
            quant_type: crate::core::QuantType::Bit1_58,
            w_stream: core::ptr::null(),
            x_stream: core::ptr::null(),
            s_stream: core::ptr::null(),
            out_buffer: core::ptr::null_mut(),
            tree_mask: core::ptr::null(),
            tree_size: 0,
            block_size: 16,
            kv_blocks: core::ptr::null(),
            num_blocks: 0,
            hardware_handle: core::ptr::null_mut(),
            enable_liquid: false,
            dt: 0.0,
            liquid_state: core::ptr::null_mut(),
            liquid_tau: core::ptr::null(),
            liquid_out_buffer: core::ptr::null_mut(),
            scratch_buffer: core::ptr::null_mut(),
            scratch_size: 0,
        };
        unsafe {
            vec101_compute(&ctx);
        }
    }
}

pub mod simd;
pub mod types;

pub use types::{
    QuantType, Vec101Block, Vec101Context, Vec101SuperBlock, Vector101__Block__Descriptor,
    Vector101__Computation__Context,
};

#[doc = " Safe abstraction for processing a single row in GEMV mode"]
#[inline(always)]
pub fn process_row_gemv_safe(
    row: usize,
    context: &Vector101__Computation__Context,
    x_mask: &[u64],
) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        simd::avx2::process_row_avx2_gemv(row, context, x_mask)
    };
    #[cfg(target_arch = "aarch64")]
    unsafe {
        simd::neon::process_row_neon_gemv(row, context, x_mask)
    };
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    unsafe {
        simd::scalar::process_row_scalar_gemv(row, context, x_mask)
    };
}

#[doc = " Safe abstraction for processing a single row in GEMM mode"]
#[inline(always)]
pub fn process_row_gemm_safe(
    row: usize,
    context: &Vector101__Computation__Context,
    x_t_ref: &[i8],
    x_mask: &[u64],
    padded_batch: usize,
    row_sums: &mut [i32],
) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        simd::avx2::process_row_avx2_gemm(row, context, x_t_ref, x_mask, padded_batch, row_sums)
    };
    #[cfg(target_arch = "aarch64")]
    unsafe {
        simd::neon::process_row_neon_gemm(row, context, x_t_ref, x_mask, padded_batch, row_sums)
    };
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    unsafe {
        simd::scalar::process_row_scalar_gemm(row, context, x_t_ref, x_mask, padded_batch, row_sums)
    };
}

#[doc = " Liquid Time-Constant (LTC) ODE integration step for Liquid Neural Networks."]
#[doc = " Integrates the `dot_product` with the current `state` using `tau_scaled` time constant."]
#[doc = " Returns the quantized INT8 activation."]
#[inline(always)]
pub fn liquid_step_i8(dot_product: i32, dt: f32, state: &mut f32, tau_scaled: i32) -> i8 {
    let input_f32 = dot_product as f32 / 128.0;
    let f_input = input_f32.abs();
    let dx_dt = -(1.0 / (tau_scaled as f32) + f_input) * (*state) + input_f32;
    *state += dx_dt * dt;
    let mut out = (*state * 127.0) as i32;
    out = out.clamp(-128, 127);
    out as i8
}
