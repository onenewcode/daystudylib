use consts::{GGML_F16_ARR, GGML_F16_EPR, GGML_F16_STEP, GGML_F32_ARR};
use core::f32;
use half::f16;
use std::arch::x86_64::*;
#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct BlockQ8_0 {
    d: f16,       // delta
    qs: [i8; 32], // quants
}

// 当目标支持 AVX2 时设置这些常量
// #[cfg(target_feature = "avx512f")]
mod consts {
    pub(crate) const GGML_F16_STEP: usize = 64;
    pub(crate) const GGML_F16_EPR: usize = 16;
    pub(crate) const GGML_F32_STEP: usize = 64;
    pub(crate) const GGML_F32_EPR: usize = 16;
    pub(crate) const GGML_F32_ARR: usize = GGML_F32_STEP / GGML_F32_EPR;
    pub(crate) const GGML_F16_ARR: usize = GGML_F16_STEP / GGML_F16_EPR;
}

/// TODO: Adding AVX-VNNI support so that we can use `_mm256_dpbssd_epi32`
#[inline]
#[allow(dead_code)]
pub unsafe fn mul_sum_i8_pairs_float(x: __m256i, y: __m256i) -> __m256 {
    // Get absolute values of x vectors
    let ax = _mm256_sign_epi8(x, x);
    // Sign the values of the y vectors
    let sy = _mm256_sign_epi8(y, x);
    mul_sum_us8_pairs_float(ax, sy)
}

#[inline]
pub unsafe fn mul_sum_us8_pairs_float(ax: __m256i, sy: __m256i) -> __m256 {
    if is_x86_feature_detected!("avx512vl") || is_x86_feature_detected!("avxvnni") {
        let zero = _mm256_setzero_si256();
        let summed_pairs = _mm256_dpbusd_epi32(zero, ax, sy);
        _mm256_cvtepi32_ps(summed_pairs)
    } else {
        // avx
        let axl = _mm256_castsi256_si128(ax);
        let axh = _mm256_extractf128_si256(ax, 1);
        let syl = _mm256_castsi256_si128(sy);
        let syh = _mm256_extractf128_si256(sy, 1);
        // Perform multiplication and create 16-bit values
        let dotl = _mm_maddubs_epi16(axl, syl);
        let doth = _mm_maddubs_epi16(axh, syh);
        sum_i16_pairs_float(doth, dotl)
    }
}

#[inline]
pub unsafe fn sum_i16_pairs_float(xh: __m128i, xl: __m128i) -> __m256 {
    let ones = _mm_set1_epi16(1);
    let summed_pairsl = _mm_madd_epi16(ones, xl);
    let summed_pairsh = _mm_madd_epi16(ones, xh);
    let summed_pairs = _mm256_set_m128i(summed_pairsh, summed_pairsl);
    _mm256_cvtepi32_ps(summed_pairs)
}

/// horizontally add 8 floats
#[inline]
pub unsafe fn hsum_float_8(x: __m256) -> f32 {
    let res = _mm256_extractf128_ps(x, 1);
    let res = _mm_add_ps(res, _mm256_castps256_ps128(x));
    let res = _mm_add_ps(res, _mm_movehl_ps(res, res));
    let res = _mm_add_ss(res, _mm_movehdup_ps(res));
    _mm_cvtss_f32(res)
}

// Unpack 32 4-bit fields into 32 bytes
// The output vector contains 32 bytes, each one in [ 0 .. 15 ] interval
#[inline]
pub unsafe fn bytes_from_nibbles_32(rsi: *const u8) -> __m256i {
    let tmp = _mm_loadu_si128(rsi as *const _);
    let bytes = _mm256_set_m128i(_mm_srli_epi16(tmp, 4), tmp);
    let low_mask = _mm256_set1_epi8(0xF);
    _mm256_and_si256(low_mask, bytes)
}
pub fn vec_dot_q8(n: usize, x: &[BlockQ8_0], y: &[BlockQ8_0]) -> f32 {
    // if is_x86_feature_detected!("avx2") {
    unsafe {
        // Initialize accumulator with zeros
        let mut acc = _mm256_setzero_ps();
        // Main loop
        (0..n / 32).into_iter().for_each(|i| {
            //  转换成查表，提升不明显
            let d = _mm256_set1_ps(x[i].d.to_f32() * (y[i].d.to_f32()));
            // let d = x86_64::_mm256_setzero_ps();

            let qx = _mm256_loadu_si256(x[i].qs.as_ptr() as *const __m256i);
            let qy = _mm256_loadu_si256(y[i].qs.as_ptr() as *const __m256i);

            let q = crate::x86_64::mul_sum_i8_pairs_float(qx, qy);

            // TODO 过慢 cpu Intel(R) Xeon(R) Gold 6330 CPU @ 2.00GHz rust 1.86.0-nightly
            // // Multiply q with scale and accumulate
            acc = _mm256_fmadd_ps(d, q, acc);
        });
        crate::x86_64::hsum_float_8(acc)
    }
}
pub fn vec_dot_f16(x: &[f16], y: &[f16]) -> f32 {
    unsafe {
        let mut sumf: f32 = 0.0;
        let n = x.len();

        let np = n & !(GGML_F16_STEP - 1);

        let mut sum: [__m512; GGML_F16_ARR] = [_mm512_setzero_ps(); GGML_F16_ARR];
        let mut ax: [__m512; GGML_F16_ARR] = [_mm512_setzero_ps(); GGML_F16_ARR];
        let mut ay: [__m512; GGML_F16_ARR] = [_mm512_setzero_ps(); GGML_F16_ARR];

        for i in (0..np).step_by(GGML_F16_STEP) {
            for j in 0..GGML_F16_ARR {
                let idx = i + j * GGML_F16_EPR;

                ax[j] = ggml_f32_cx16_load(x.as_ptr().add(idx) as *const u8);
                ay[j] = ggml_f32_cx16_load(y.as_ptr().add(idx) as *const u8);

                sum[j] = _mm512_fmadd_ps(ax[j], ay[j], sum[j]);
            }
        }

        ggml_f32x16_reduce(sumf, &mut sum);
        // 处理不能除尽的元素
        (np..n).into_iter().for_each(|i| {
            sumf += x[i].to_f32() * y[i].to_f32();
        });
        sumf
    }
}

#[target_feature(enable = "avx512f")]
unsafe fn ggml_f32x16_reduce(mut res: f32, x: &mut [__m512; GGML_F32_ARR]) {
    let mut offset = GGML_F32_ARR >> 1;
    // 第一轮归约
    for i in 0..offset {
        x[i] = _mm512_add_ps(x[i], x[offset + i]);
    }

    offset >>= 1;

    // 后续归约步骤
    while offset > 0 {
        for i in 0..offset {
            x[i] = _mm512_add_ps(x[i], x[offset + i]);
        }
        offset >>= 1;
    }

    // 最终归约
    res += _mm512_reduce_add_ps(x[0]);
}
#[inline]
unsafe fn ggml_f32_cx16_load(x: *const u8) -> __m512 {
    _mm512_cvtph_ps(_mm256_loadu_si256(x as *const __m256i))
}
// #define GGML_F32Cx16_LOAD(x)     _mm512_cvtph_ps(_mm256_loadu_si256((const __m256i *)(x)))
// #define GGML_F32Cx16_STORE(x, y) _mm256_storeu_si256((__m256i *)(x), _mm512_cvtps_ph(y, 0))
