#![feature(portable_simd)]
#![feature(test)]
#![feature(stdarch_x86_avx512)]
#![feature(avx512_target_feature)]
#![feature(iter_array_chunks)]
use std::simd::{self, i32x4, i8x32, num::SimdInt};

use half::f16;
use rand::{thread_rng, Rng};
use x86_64::{
    consts::{GGML_F16_ARR, GGML_F16_EPR, GGML_F16_STEP},
    ggml_f32_cx16_load, ggml_f32x16_reduce,
};
#[allow(dead_code)]
#[cfg(target_arch = "x86_64")]
mod x86_64;

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct BlockQ8_0 {
    pub d: f16,       // delta
    pub qs: [i8; 32], // quants
}

// #[link(name = "ggml")]
// extern "C" {
//     fn ggml_vec_dot_q8_0_q8_0(
//         n: i32,               // number of elements
//         s: *mut f32,          // result
//         bs: usize,            // not used?
//         vx: *const BlockQ8_0, // binary of quantized vec x
//         bx: usize,            // not used?
//         vy: *const BlockQ8_0, // binary of quantized vec y
//         by: usize,            // not used?
//         nrc: i32,             // always 1?
//     );
// }

// pub fn vec_dot_q8_ggml(n: usize, x: &[BlockQ8_0], y: &[BlockQ8_0]) -> f32 {
//     let mut result: f32 = 0.0;
//     unsafe {
//         ggml_vec_dot_q8_0_q8_0(
//             n as i32,
//             &mut result as *mut f32,
//             0,
//             x.as_ptr(),
//             0,
//             y.as_ptr(),
//             0,
//             1,
//         );
//     }
//     result
// }

pub fn vec_dot_q8_naive(n: usize, x: &[BlockQ8_0], y: &[BlockQ8_0]) -> f32 {
    let mut result: f32 = 0.0;
    for i in 0..(n / 32) {
        let mut tmp = 0.0;
        for j in 0..32 {
            tmp += (x[i].qs[j] as i32 * y[i].qs[j] as i32) as f32;
        }
        result += tmp * f16::to_f32(x[i as usize].d) * f16::to_f32(y[i as usize].d);
    }
    result
}

pub fn vec_dot_q8_stdsimd(n: usize, x: &[BlockQ8_0], y: &[BlockQ8_0]) -> f32 {
    let mut sumf: f32 = 0.0;
    for i in 0..n / 32 {
        let mut sumi: i32 = 0;
        for j in 0..8 {
            let ax = i32x4::from_array([
                x[i].qs[j * 4] as i32,
                x[i].qs[j * 4 + 1] as i32,
                x[i].qs[j * 4 + 2] as i32,
                x[i].qs[j * 4 + 3] as i32,
            ]);
            let bx = i32x4::from_array([
                y[i].qs[j * 4] as i32,
                y[i].qs[j * 4 + 1] as i32,
                y[i].qs[j * 4 + 2] as i32,
                y[i].qs[j * 4 + 3] as i32,
            ]);
            sumi += (ax * bx).reduce_sum();
        }
        sumf += sumi as f32 * x[i].d.to_f32() * y[i].d.to_f32();
    }

    sumf
}

#[cfg(all(target_arch = "x86_64"))]
// #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
pub fn vec_dot_q8_0_q8_0_avx2(abs: &[BlockQ8_0], bbs: &[BlockQ8_0]) -> f32 {
    use std::arch::x86_64::*;

    use crate::x86_64::*;

    debug_assert_eq!(abs.len(), bbs.len());

    unsafe {
        let mut acc0 = _mm256_setzero_ps();
        let mut acc1 = _mm256_setzero_ps();

        for [(abs0, bbs0), (abs1, bbs1)] in abs.iter().zip(bbs).array_chunks::<2>() {
            let d0 = _mm256_set1_ps(abs0.d.to_f32() * bbs0.d.to_f32());
            let d1 = _mm256_set1_ps(abs1.d.to_f32() * bbs1.d.to_f32());

            let qa0 = _mm256_loadu_si256(abs0.qs.as_ptr() as *const __m256i);
            let qb0 = _mm256_loadu_si256(bbs0.qs.as_ptr() as *const __m256i);

            let qa1 = _mm256_loadu_si256(abs1.qs.as_ptr() as *const __m256i);
            let qb1 = _mm256_loadu_si256(bbs1.qs.as_ptr() as *const __m256i);

            let q0 = mul_sum_i8_pairs_float(qa0, qb0);
            let q1 = mul_sum_i8_pairs_float(qa1, qb1);

            acc0 = _mm256_fmadd_ps(d0, q0, acc0);
            acc1 = _mm256_fmadd_ps(d1, q1, acc1);
        }

        if abs.len() % 2 == 1 {
            let a = abs.last().unwrap_unchecked();
            let b = bbs.last().unwrap_unchecked();

            let d = _mm256_set1_ps(a.d.to_f32() * b.d.to_f32());

            let qa = _mm256_loadu_si256(a.qs.as_ptr() as *const __m256i);
            let qb = _mm256_loadu_si256(b.qs.as_ptr() as *const __m256i);

            let q = mul_sum_i8_pairs_float(qa, qb);

            acc0 = _mm256_fmadd_ps(d, q, acc0);
        }

        hsum_float_8(_mm256_add_ps(acc0, acc1))
    }
}
pub fn vec_dot_q8(x: &[BlockQ8_0], y: &[BlockQ8_0]) -> f32 {
    unsafe {
        use std::arch::x86_64::*;
        // Initialize accumulator with zeros
        let mut acc = _mm256_setzero_ps();
        // Main loop
        (0..x.len()).into_iter().for_each(|i| {
            //  转换成查表，提升不明显
            let d = _mm256_set1_ps(x[i].d.to_f32() * (y[i].d.to_f32()));
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
        use std::arch::x86_64::*;
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

#[cfg(target_arch = "aarch64")]
pub fn vec_dot_q8_neon(n: usize, a: &[BlockQ8_0], b: &[BlockQ8_0]) -> f32 {
    unsafe {
        use std::arch::aarch64;

        let mut sumv = aarch64::vdupq_n_f32(0.0);
        let zerov = aarch64::vdupq_n_s32(0);

        for i in 0..n / 32 {
            let ab = a.get_unchecked(i);
            let bb = b.get_unchecked(i);

            let av0 = aarch64::vld1q_s8(ab.qs.as_ptr());
            let av1 = aarch64::vld1q_s8(ab.qs.as_ptr().add(16));
            let bv0 = aarch64::vld1q_s8(bb.qs.as_ptr());
            let bv1 = aarch64::vld1q_s8(bb.qs.as_ptr().add(16));

            let tmpv = aarch64::vcvtq_f32_s32(aarch64::vaddq_s32(
                aarch64::vdotq_s32(zerov, av0, bv0),
                aarch64::vdotq_s32(zerov, av1, bv1),
            ));
            sumv = aarch64::vmlaq_n_f32(sumv, tmpv, f16::to_f32(ab.d) * f16::to_f32(bb.d));
        }

        aarch64::vaddvq_f32(sumv)
    }
}

#[cfg(target_arch = "aarch64")]
pub fn vec_dot_q8_neon_unrolled(n: usize, a: &[BlockQ8_0], b: &[BlockQ8_0]) -> f32 {
    unsafe {
        use std::arch::aarch64;
        let mut sumv0 = aarch64::vdupq_n_f32(0.0);
        let mut sumv1 = aarch64::vdupq_n_f32(0.0);
        let zerov = aarch64::vdupq_n_s32(0);

        for i in (0..n / 32).step_by(2) {
            let ab0 = a.get_unchecked(i);
            let ab1 = a.get_unchecked(i + 1);
            let bb0 = b.get_unchecked(i);
            let bb1 = b.get_unchecked(i + 1);

            let av00 = aarch64::vld1q_s8(ab0.qs.as_ptr());
            let av01 = aarch64::vld1q_s8(ab0.qs.as_ptr().add(16));
            let av10 = aarch64::vld1q_s8(ab1.qs.as_ptr());
            let av11 = aarch64::vld1q_s8(ab1.qs.as_ptr().add(16));

            let bv00 = aarch64::vld1q_s8(bb0.qs.as_ptr());
            let bv01 = aarch64::vld1q_s8(bb0.qs.as_ptr().add(16));
            let bv10 = aarch64::vld1q_s8(bb1.qs.as_ptr());
            let bv11 = aarch64::vld1q_s8(bb1.qs.as_ptr().add(16));

            // vdotq_s32: dot product of two q registers (128 bit) of signed int 8, output is 4 int32 values in a q register
            // vcvtq_f32_s32: convert a q register (128 bit) from signed int 32 to f32
            // vmlaq_n_f32: multiply an scalar over a q register (128 bit) and accumulate. it seems the compiler will produce a fmul.4s and fadd.4s, not a single fmla.4s
            // vfmaq_f32: multiply and accumulate two q registers (128 bit) of f32, output is a q register (128 bit) of f32, it seems the compiler will produce a fmla.4s
            sumv0 = aarch64::vmlaq_n_f32(
                sumv0,
                aarch64::vcvtq_f32_s32(aarch64::vaddq_s32(
                    aarch64::vdotq_s32(zerov, av00, bv00),
                    aarch64::vdotq_s32(zerov, av01, bv01),
                )),
                f16::to_f32(ab0.d) * f16::to_f32(bb0.d),
            );

            sumv1 = aarch64::vmlaq_n_f32(
                sumv1,
                aarch64::vcvtq_f32_s32(aarch64::vaddq_s32(
                    aarch64::vdotq_s32(zerov, av10, bv10),
                    aarch64::vdotq_s32(zerov, av11, bv11),
                )),
                f16::to_f32(ab1.d) * f16::to_f32(bb1.d),
            );
        }

        aarch64::vaddvq_f32(sumv0) + aarch64::vaddvq_f32(sumv1)
    }
}

#[cfg(target_arch = "aarch64")]
pub fn vec_dot_q8_neon_unrolled_single_sum(n: usize, a: &[BlockQ8_0], b: &[BlockQ8_0]) -> f32 {
    unsafe {
        use std::arch::aarch64;
        let mut sumv0 = aarch64::vdupq_n_f32(0.0);
        let zerov = aarch64::vdupq_n_s32(0);

        for i in (0..n / 32).step_by(2) {
            let ab0 = a.get_unchecked(i);
            let ab1 = a.get_unchecked(i + 1);
            let bb0 = b.get_unchecked(i);
            let bb1 = b.get_unchecked(i + 1);

            let av00 = aarch64::vld1q_s8(ab0.qs.as_ptr());
            let av01 = aarch64::vld1q_s8(ab0.qs.as_ptr().add(16));
            let av10 = aarch64::vld1q_s8(ab1.qs.as_ptr());
            let av11 = aarch64::vld1q_s8(ab1.qs.as_ptr().add(16));

            let bv00 = aarch64::vld1q_s8(bb0.qs.as_ptr());
            let bv01 = aarch64::vld1q_s8(bb0.qs.as_ptr().add(16));
            let bv10 = aarch64::vld1q_s8(bb1.qs.as_ptr());
            let bv11 = aarch64::vld1q_s8(bb1.qs.as_ptr().add(16));

            // vdotq_s32: dot product of two q registers (128 bit) of signed int 8, output is 4 int32 values in a q register
            // vcvtq_f32_s32: convert a q register (128 bit) from signed int 32 to f32
            // vmlaq_n_f32: multiply an scalar over a q register (128 bit) and accumulate. it seems the compiler will produce a fmul.4s and fadd.4s, not a single fmla.4s
            // vfmaq_f32: multiply and accumulate two q registers (128 bit) of f32, output is a q register (128 bit) of f32, it seems the compiler will produce a fmla.4s
            sumv0 = aarch64::vmlaq_n_f32(
                sumv0,
                aarch64::vcvtq_f32_s32(aarch64::vaddq_s32(
                    aarch64::vdotq_s32(zerov, av00, bv00),
                    aarch64::vdotq_s32(zerov, av01, bv01),
                )),
                f16::to_f32(ab0.d) * f16::to_f32(bb0.d),
            );

            sumv0 = aarch64::vmlaq_n_f32(
                sumv0,
                aarch64::vcvtq_f32_s32(aarch64::vaddq_s32(
                    aarch64::vdotq_s32(zerov, av10, bv10),
                    aarch64::vdotq_s32(zerov, av11, bv11),
                )),
                f16::to_f32(ab1.d) * f16::to_f32(bb1.d),
            );
        }

        aarch64::vaddvq_f32(sumv0)
    }
}

#[cfg(target_arch = "aarch64")]
pub fn vec_dot_q8_neon_unrolled_vfma(n: usize, a: &[BlockQ8_0], b: &[BlockQ8_0]) -> f32 {
    unsafe {
        use std::arch::aarch64;
        let mut sumv0 = aarch64::vdupq_n_f32(0.0);
        let mut sumv1 = aarch64::vdupq_n_f32(0.0);
        let zerov = aarch64::vdupq_n_s32(0);

        for i in (0..n / 32).step_by(2) {
            let ab0 = a.get_unchecked(i);
            let ab1 = a.get_unchecked(i + 1);
            let bb0 = b.get_unchecked(i);
            let bb1 = b.get_unchecked(i + 1);

            let av00 = aarch64::vld1q_s8(ab0.qs.as_ptr());
            let av01 = aarch64::vld1q_s8(ab0.qs.as_ptr().add(16));
            let av10 = aarch64::vld1q_s8(ab1.qs.as_ptr());
            let av11 = aarch64::vld1q_s8(ab1.qs.as_ptr().add(16));

            let bv00 = aarch64::vld1q_s8(bb0.qs.as_ptr());
            let bv01 = aarch64::vld1q_s8(bb0.qs.as_ptr().add(16));
            let bv10 = aarch64::vld1q_s8(bb1.qs.as_ptr());
            let bv11 = aarch64::vld1q_s8(bb1.qs.as_ptr().add(16));

            // vdotq_s32: dot product of two q registers (128 bit) of signed int 8, output is 4 int32 values in a q register
            // vcvtq_f32_s32: convert a q register (128 bit) from signed int 32 to f32
            // vmlaq_n_f32: multiply an scalar over a q register (128 bit) and accumulate. it seems the compiler will produce a fmul.4s and fadd.4s, not a single fmla.4s
            // vfmaq_f32: multiply and accumulate two q registers (128 bit) of f32, output is a q register (128 bit) of f32, it seems the compiler will produce a fmla.4s
            sumv0 = aarch64::vfmaq_f32(
                sumv0,
                aarch64::vcvtq_f32_s32(aarch64::vaddq_s32(
                    aarch64::vdotq_s32(zerov, av00, bv00),
                    aarch64::vdotq_s32(zerov, av01, bv01),
                )),
                aarch64::vdupq_n_f32(f16::to_f32(ab0.d) * f16::to_f32(bb0.d)),
            );

            sumv1 = aarch64::vfmaq_f32(
                sumv1,
                aarch64::vcvtq_f32_s32(aarch64::vaddq_s32(
                    aarch64::vdotq_s32(zerov, av10, bv10),
                    aarch64::vdotq_s32(zerov, av11, bv11),
                )),
                aarch64::vdupq_n_f32(f16::to_f32(ab1.d) * f16::to_f32(bb1.d)),
            );
        }

        aarch64::vaddvq_f32(sumv0) + aarch64::vaddvq_f32(sumv1)
    }
}
// generate a random vector of BlockQ8_0
pub fn gen_rand_block_q8_0() -> BlockQ8_0 {
    let mut rng = thread_rng();
    let d: f32 = rng.gen_range(0.0..2.0);
    let mut qs: [i8; 32] = [0; 32];
    for i in 0..32 {
        qs[i] = rng.gen::<i8>();
    }
    BlockQ8_0 {
        d: f16::from_f32(d),
        qs,
    }
}

pub fn gen_rand_block_f16(n: usize) -> Vec<f16> {
    let mut rng = thread_rng();
    (0..n)
        .map(|_| {
            // 生成一个介于 0.0 到 1.0 之间的 f32 随机数
            let random_f32: f32 = rng.gen_range(0.0..1.0);
            // 将 f32 转换为 f16
            f16::from_f32(random_f32)
        })
        .collect()
}
pub fn gen_rand_block_q8_0_vec(n: usize) -> Vec<BlockQ8_0> {
    let mut v: Vec<BlockQ8_0> = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(gen_rand_block_q8_0());
    }
    v
}
#[cfg(test)]
mod tests {

    use super::*;
    use half::vec;
    use rand::{thread_rng, Rng};
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_vec_dot_q8() {
        let v1 = gen_rand_block_q8_0_vec(4);
        let v2 = gen_rand_block_q8_0_vec(4);

        let naive_result = vec_dot_q8_naive(128, &v1, &v2);
        // let result = vec_dot_q8_ggml(128, &v1, &v2);
        // assert!((result - naive_result).abs() < 1e-2);
        let result = vec_dot_q8_stdsimd(128,&v1, &v2);
        assert!((result - naive_result).abs() < 1e-2);
        let result = vec_dot_q8(&v1, &v2);
        assert!((result - naive_result).abs() < 1e-2);
        let result = vec_dot_q8_0_q8_0_avx2( &v1, &v2);
        assert!((result - naive_result).abs() < 1e-2);
        // let result = vec_dot_q8_neon_unrolled(64, &v1, &v2);
        // assert!((result - naive_result).abs() < 1e-2);
    }

    #[test]
    fn test_cpu_feature() {
        if is_x86_feature_detected!("avxvnni") {
            println!("supported.");
        } else {
            println!("not supported.");
        }
    }
}
