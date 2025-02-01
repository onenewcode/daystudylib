# 操作
需要导入共享库 
>export LD_LIBRARY_PATH=/home/ztf/llama.cpp/build/bin:$LD_LIBRARY_PATH

# asm
>https://github.com/pacak/cargo-show-asm

>cargo asm -p zcriterion --lib   zcriterion::vec_dot_q8 0 --intel
>cargo asm --lib   --release --intel --full-name 
>cargo flamegraph --unit-test zcriterion  -- test::test_vec_dot_q8_simdx86
**linux**
>objdump -d --disassemble=ggml_vec_dot_q8_0_q8_0 ./libggml-cpu.so

> objdump -d -S --disassemble=ggml_vec_dot_q8_0_q8_0 --no-show-raw-insn ./libggml-cpu.so

>cmake -B build -DCMAKE_BUILD_TYPE=Debug -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ && cmake --build build 
>cmake -B build -DCMAKE_BUILD_TYPE=Debug  && cmake --build build 
```asm
void ggml_vec_dot_q8_0_q8_0(int n, float * restrict s, size_t bs, const void * restrict vx, size_t bx, const void * restrict vy, size_t by, int nrc) {
   657d0:       endbr64 
   657d4:       lea    0x8(%rsp),%r10
   657d9:       and    $0xffffffffffffffe0,%rsp
   657dd:       pushq  -0x8(%r10)
   657e1:       push   %rbp
   657e2:       mov    %rsp,%rbp
   657e5:       push   %r10
   657e7:       sub    $0x188,%rsp
   657ee:       mov    %edi,-0x154(%rbp)
   657f4:       mov    %rsi,-0x160(%rbp)
   657fb:       mov    %rdx,-0x168(%rbp)
   65802:       mov    %rcx,-0x170(%rbp)
   65809:       mov    %r8,-0x178(%rbp)
   65810:       mov    %r9,-0x180(%rbp)
    const int qk = QK8_0;
   65817:       movl   $0x20,-0x13c(%rbp)
    const int nb = n / qk;
   65821:       mov    -0x154(%rbp),%eax
   65827:       cltd   
   65828:       idivl  -0x13c(%rbp)
   6582e:       mov    %eax,-0x138(%rbp)

    assert(n % qk == 0);
   65834:       mov    -0x154(%rbp),%eax
   6583a:       cltd   
   6583b:       idivl  -0x13c(%rbp)
   65841:       mov    %edx,%eax
   65843:       test   %eax,%eax
   65845:       je     65866 <ggml_vec_dot_q8_0_q8_0+0x96>
   65847:       lea    0x6e482(%rip),%rcx        # d3cd0 <__PRETTY_FUNCTION__.32367>
   6584e:       mov    $0xd13,%edx
   65853:       lea    0x6dd76(%rip),%rsi        # d35d0 <GGML_TENSOR_SIZE+0xf0>
   6585a:       lea    0x6ddff(%rip),%rdi        # d3660 <kvalues_iq4nl+0x10>
   65861:       callq  ff50 <__assert_fail@plt>
#if defined(__ARM_FEATURE_MATMUL_INT8)
    assert((nrc == 2) || (nrc == 1));
#else
    assert(nrc == 1);
   65866:       cmpl   $0x1,0x8(%r10)
   6586b:       je     6588c <ggml_vec_dot_q8_0_q8_0+0xbc>
   6586d:       lea    0x6e45c(%rip),%rcx        # d3cd0 <__PRETTY_FUNCTION__.32367>
   65874:       mov    $0xd17,%edx
   65879:       lea    0x6dd50(%rip),%rsi        # d35d0 <GGML_TENSOR_SIZE+0xf0>
   65880:       lea    0x6dde5(%rip),%rdi        # d366c <kvalues_iq4nl+0x1c>
   65887:       callq  ff50 <__assert_fail@plt>
    UNUSED(nrc);
    UNUSED(bx);
    UNUSED(by);
    UNUSED(bs);

    const block_q8_0 * restrict x = vx;
   6588c:       mov    -0x170(%rbp),%rax
   65893:       mov    %rax,-0x130(%rbp)
    const block_q8_0 * restrict y = vy;
   6589a:       mov    -0x180(%rbp),%rax
   658a1:       mov    %rax,-0x128(%rbp)

        return;
    }
#endif

    int ib = 0;
   658a8:       movl   $0x0,-0x14c(%rbp)
    float sumf = 0;
   658b2:       vxorps %xmm0,%xmm0,%xmm0
   658b6:       vmovss %xmm0,-0x148(%rbp)
}

extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_setzero_ps (void)
{
  return __extension__ (__m256){ 0.0, 0.0, 0.0, 0.0,
   658be:       vxorps %xmm0,%xmm0,%xmm0
    }

    sumf = vaddvq_f32(sumv0) + vaddvq_f32(sumv1);
#elif defined(__AVX2__)
    // Initialize accumulator with zeros
    __m256 acc = _mm256_setzero_ps();
   658c2:       vmovaps %ymm0,-0x110(%rbp)

    // Main loop
    for (; ib < nb; ++ib) {
   658ca:       jmpq   65a15 <ggml_vec_dot_q8_0_q8_0+0x245>
        // Compute combined scale for the block
        const __m256 d = _mm256_set1_ps(GGML_FP16_TO_FP32(x[ib].d) * GGML_FP16_TO_FP32(y[ib].d));
   658cf:       mov    -0x14c(%rbp),%eax
   658d5:       cltq   
   658d7:       imul   $0x22,%rax,%rdx
   658db:       mov    -0x130(%rbp),%rax
   658e2:       add    %rdx,%rax
   658e5:       movzwl (%rax),%eax
   658e8:       movzwl %ax,%eax
   658eb:       mov    %eax,%edi
   658ed:       callq  5f611 <ggml_lookup_fp16_to_fp32>
   658f2:       vmovss %xmm0,-0x184(%rbp)
   658fa:       mov    -0x14c(%rbp),%eax
   65900:       cltq   
   65902:       imul   $0x22,%rax,%rdx
   65906:       mov    -0x128(%rbp),%rax
   6590d:       add    %rdx,%rax
   65910:       movzwl (%rax),%eax
   65913:       movzwl %ax,%eax
   65916:       mov    %eax,%edi
   65918:       callq  5f611 <ggml_lookup_fp16_to_fp32>
   6591d:       vmulss -0x184(%rbp),%xmm0,%xmm0
   65925:       vmovss %xmm0,-0x134(%rbp)

/* Create a vector with all elements equal to A.  */
extern __inline __m256 __attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_set1_ps (float __A)
{
  return __extension__ (__m256){ __A, __A, __A, __A,
   6592d:       vbroadcastss -0x134(%rbp),%ymm0
   65936:       vmovaps %ymm0,-0xf0(%rbp)
        __m256i qx = _mm256_loadu_si256((const __m256i *)x[ib].qs);
   6593e:       mov    -0x14c(%rbp),%eax
   65944:       cltq   
   65946:       imul   $0x22,%rax,%rdx
   6594a:       mov    -0x130(%rbp),%rax
   65951:       add    %rdx,%rax
   65954:       add    $0x2,%rax
   65958:       mov    %rax,-0x118(%rbp)
  return *__P;
   6595f:       mov    -0x118(%rbp),%rax
   65966:       vmovdqu64 (%rax),%ymm0
   6596c:       vmovdqa64 %ymm0,-0xd0(%rbp)
        __m256i qy = _mm256_loadu_si256((const __m256i *)y[ib].qs);
   65976:       mov    -0x14c(%rbp),%eax
   6597c:       cltq   
   6597e:       imul   $0x22,%rax,%rdx
   65982:       mov    -0x128(%rbp),%rax
   65989:       add    %rdx,%rax
   6598c:       add    $0x2,%rax
   65990:       mov    %rax,-0x120(%rbp)
   65997:       mov    -0x120(%rbp),%rax
   6599e:       vmovdqu64 (%rax),%ymm0
   659a4:       vmovdqa64 %ymm0,-0xb0(%rbp)

        const __m256 q = mul_sum_i8_pairs_float(qx, qy);
   659ae:       vmovdqa64 -0xb0(%rbp),%ymm1
   659b8:       vmovdqa64 -0xd0(%rbp),%ymm0
   659c2:       callq  6008e <mul_sum_i8_pairs_float>
   659c7:       vmovaps %ymm0,-0x90(%rbp)
   659cf:       vmovaps -0xf0(%rbp),%ymm0
   659d7:       vmovaps %ymm0,-0x70(%rbp)
   659dc:       vmovaps -0x90(%rbp),%ymm0
   659e4:       vmovaps %ymm0,-0x50(%rbp)
   659e9:       vmovaps -0x110(%rbp),%ymm0
   659f1:       vmovaps %ymm0,-0x30(%rbp)

extern __inline __m256
__attribute__((__gnu_inline__, __always_inline__, __artificial__))
_mm256_fmadd_ps (__m256 __A, __m256 __B, __m256 __C)
{
  return (__m256)__builtin_ia32_vfmaddps256 ((__v8sf)__A, (__v8sf)__B,
   659f6:       vmovaps -0x50(%rbp),%ymm1
   659fb:       vmovaps -0x30(%rbp),%ymm0
   65a00:       vfmadd231ps -0x70(%rbp),%ymm1,%ymm0
   65a06:       nop

        // Multiply q with scale and accumulate
        acc = _mm256_fmadd_ps( d, q, acc );
   65a07:       vmovaps %ymm0,-0x110(%rbp)
    for (; ib < nb; ++ib) {
   65a0f:       incl   -0x14c(%rbp)
   65a15:       mov    -0x14c(%rbp),%eax
   65a1b:       cmp    -0x138(%rbp),%eax
   65a21:       jl     658cf <ggml_vec_dot_q8_0_q8_0+0xff>
    }

    sumf = hsum_float_8(acc);
   65a27:       vmovaps -0x110(%rbp),%ymm0
   65a2f:       callq  5f662 <hsum_float_8>
   65a34:       vmovd  %xmm0,%eax
   65a38:       mov    %eax,-0x148(%rbp)
        acc = __lasx_xvfmadd_s( d, q, acc );
    }

    sumf = hsum_float_8(acc);
#endif
    for (; ib < nb; ++ib) {
   65a3e:       jmpq   65b4a <ggml_vec_dot_q8_0_q8_0+0x37a>
        int sumi = 0;
   65a43:       movl   $0x0,-0x144(%rbp)

        for (int j = 0; j < qk; j++) {
   65a4d:       movl   $0x0,-0x140(%rbp)
   65a57:       jmp    65ab4 <ggml_vec_dot_q8_0_q8_0+0x2e4>
            sumi += x[ib].qs[j]*y[ib].qs[j];
   65a59:       mov    -0x14c(%rbp),%eax
   65a5f:       cltq   
   65a61:       imul   $0x22,%rax,%rdx
   65a65:       mov    -0x130(%rbp),%rax
   65a6c:       add    %rax,%rdx
   65a6f:       mov    -0x140(%rbp),%eax
   65a75:       cltq   
   65a77:       movzbl 0x2(%rdx,%rax,1),%eax
   65a7c:       movsbl %al,%edx
   65a7f:       mov    -0x14c(%rbp),%eax
   65a85:       cltq   
   65a87:       imul   $0x22,%rax,%rcx
   65a8b:       mov    -0x128(%rbp),%rax
   65a92:       add    %rax,%rcx
   65a95:       mov    -0x140(%rbp),%eax
   65a9b:       cltq   
   65a9d:       movzbl 0x2(%rcx,%rax,1),%eax
   65aa2:       movsbl %al,%eax
   65aa5:       imul   %edx,%eax
   65aa8:       add    %eax,-0x144(%rbp)
        for (int j = 0; j < qk; j++) {
   65aae:       incl   -0x140(%rbp)
   65ab4:       mov    -0x140(%rbp),%eax
   65aba:       cmp    -0x13c(%rbp),%eax
   65ac0:       jl     65a59 <ggml_vec_dot_q8_0_q8_0+0x289>
        }

        sumf += sumi*(GGML_FP16_TO_FP32(x[ib].d)*GGML_FP16_TO_FP32(y[ib].d));
   65ac2:       vcvtsi2ssl -0x144(%rbp),%xmm3,%xmm3
   65aca:       vmovss %xmm3,-0x184(%rbp)
   65ad2:       mov    -0x14c(%rbp),%eax
   65ad8:       cltq   
   65ada:       imul   $0x22,%rax,%rdx
   65ade:       mov    -0x130(%rbp),%rax
   65ae5:       add    %rdx,%rax
   65ae8:       movzwl (%rax),%eax
   65aeb:       movzwl %ax,%eax
   65aee:       mov    %eax,%edi
   65af0:       callq  5f611 <ggml_lookup_fp16_to_fp32>
   65af5:       vmovss %xmm0,-0x188(%rbp)
   65afd:       mov    -0x14c(%rbp),%eax
   65b03:       cltq   
   65b05:       imul   $0x22,%rax,%rdx
   65b09:       mov    -0x128(%rbp),%rax
   65b10:       add    %rdx,%rax
   65b13:       movzwl (%rax),%eax
   65b16:       movzwl %ax,%eax
   65b19:       mov    %eax,%edi
   65b1b:       callq  5f611 <ggml_lookup_fp16_to_fp32>
   65b20:       vmulss -0x188(%rbp),%xmm0,%xmm0
   65b28:       vmulss -0x184(%rbp),%xmm0,%xmm0
   65b30:       vmovss -0x148(%rbp),%xmm1
   65b38:       vaddss %xmm0,%xmm1,%xmm0
   65b3c:       vmovss %xmm0,-0x148(%rbp)
    for (; ib < nb; ++ib) {
   65b44:       incl   -0x14c(%rbp)
   65b4a:       mov    -0x14c(%rbp),%eax
   65b50:       cmp    -0x138(%rbp),%eax
   65b56:       jl     65a43 <ggml_vec_dot_q8_0_q8_0+0x273>
    }

    *s = sumf;
   65b5c:       mov    -0x160(%rbp),%rax
   65b63:       vmovss -0x148(%rbp),%xmm0
   65b6b:       vmovss %xmm0,(%rax)
}
   65b6f:       nop
   65b70:       add    $0x188,%rsp
   65b77:       pop    %r10
   65b79:       pop    %rbp
   65b7a:       lea    -0x8(%r10),%rsp
   65b7e:       retq   
```
##
```asm
zcriterion::vec_dot_q8_simdx86:
                // /home/ztf/daystudylib/criterion/src/lib.rs : 153
                pub fn vec_dot_q8_simdx86(n: usize, x: &[BlockQ8_0], y: &[BlockQ8_0]) -> f32 {
        push rbp
        mov rbp, rsp
        push r15
        push r14
        push r13
        push r12
        push rbx
        and rsp, -32
        sub rsp, 544
        mov qword ptr [rsp + 48], r8
        mov qword ptr [rsp + 16], rcx
        mov qword ptr [rsp + 56], rdx
        mov r12, rsi
        mov r14, rdi
        lea rdi, [rsp + 224]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 158
                let mut acc = x86_64::_mm256_setzero_ps();
        call core::core_arch::x86::avx::_mm256_setzero_ps
        movdqa xmm0, xmmword ptr [rsp + 224]
        movaps xmm1, xmmword ptr [rsp + 240]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs : 1720
                fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
        cmp r14, 32
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs : 764
                if self.start < self.end {
        jae .LBB17_1
.LBB17_38:
        movaps xmmword ptr [rsp + 128], xmm1
        movdqa xmmword ptr [rsp + 144], xmm0
                // /home/ztf/daystudylib/criterion/src/lib.rs : 144
                let mut res = x86_64::_mm256_extractf128_ps(x, 1);
        movaps xmmword ptr [rsp + 112], xmm1
        movdqa xmmword ptr [rsp + 96], xmm0
        lea rdi, [rsp + 352]
        lea rsi, [rsp + 96]
        call core::core_arch::x86::avx::_mm256_extractf128_ps
                // /home/ztf/daystudylib/criterion/src/lib.rs : 145
                res = x86_64::_mm_add_ps(res, x86_64::_mm256_castps256_ps128(x));
        movaps xmm0, xmmword ptr [rsp + 352]
        movaps xmmword ptr [rsp + 16], xmm0
        movaps xmm0, xmmword ptr [rsp + 128]
        movaps xmmword ptr [rsp + 80], xmm0
        movaps xmm0, xmmword ptr [rsp + 144]
        movaps xmmword ptr [rsp + 64], xmm0
        lea rdi, [rsp + 192]
        lea rsi, [rsp + 64]
        call core::core_arch::x86::avx::_mm256_castps256_ps128
        movaps xmm0, xmmword ptr [rsp + 16]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs : 34
                simd_add(a, b)
        addps xmm0, xmmword ptr [rsp + 192]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/macros.rs : 124
                $crate::intrinsics::simd::simd_shuffle(
        movaps xmm1, xmm0
        unpckhpd xmm1, xmm0
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs : 34
                simd_add(a, b)
        addps xmm1, xmm0
        movaps xmmword ptr [rsp + 16], xmm1
                // /home/ztf/daystudylib/criterion/src/lib.rs : 147
                res = x86_64::_mm_add_ss(res, x86_64::_mm_movehdup_ps(res));
        movaps xmmword ptr [rsp + 192], xmm1
        lea rdi, [rsp + 384]
        lea rsi, [rsp + 192]
        call core::core_arch::x86::sse3::_mm_movehdup_ps
        movaps xmm0, xmmword ptr [rsp + 16]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/../../stdarch/crates/core_arch/src/x86/sse.rs : 22
                simd_insert!(a, 0, _mm_cvtss_f32(a) + _mm_cvtss_f32(b))
        addss xmm0, dword ptr [rsp + 384]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 178
                }
        lea rsp, [rbp - 40]
        pop rbx
        pop r12
        pop r13
        pop r14
        pop r15
        pop rbp
        ret
.LBB17_1:
        shr r14, 5
                // /home/ztf/daystudylib/criterion/src/lib.rs : 162
                let d = x86_64::_mm256_set1_ps(x[i].d.to_f32() * (y[i].d.to_f32()));
        add qword ptr [rsp + 16], 2
        add r12, 2
        xor ebx, ebx
        mov qword ptr [rsp + 168], r14
        jmp .LBB17_2
.LBB17_35:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 93
                unsafe { x86::f16_to_f32_x86_f16c(i) }
        mov edi, r13d
        call half::binary16::arch::x86::f16_to_f32_x86_f16c
.LBB17_36:
        movss xmm1, dword ptr [rsp + 32]
.LBB17_37:
                // /home/ztf/daystudylib/criterion/src/lib.rs : 162
                let d = x86_64::_mm256_set1_ps(x[i].d.to_f32() * (y[i].d.to_f32()));
        mulss xmm1, xmm0
        inc rbx
        lea rdi, [rsp + 416]
        movaps xmm0, xmm1
        call core::core_arch::x86::avx::_mm256_set1_ps
        lea rdi, [rsp + 64]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 164
                let qx = x86_64::_mm256_loadu_si256(x[i].qs.as_ptr() as *const x86_64::__m256i);
        mov rsi, r12
        call core::core_arch::x86::avx::_mm256_loadu_si256
        movaps xmm0, xmmword ptr [rsp + 64]
        movaps xmmword ptr [rsp + 32], xmm0
        movaps xmm0, xmmword ptr [rsp + 80]
        movaps xmmword ptr [rsp + 176], xmm0
        lea rdi, [rsp + 448]
        mov r15, qword ptr [rsp + 16]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 165
                let qy = x86_64::_mm256_loadu_si256(y[i].qs.as_ptr() as *const x86_64::__m256i);
        mov rsi, r15
        call core::core_arch::x86::avx::_mm256_loadu_si256
        movaps xmm1, xmmword ptr [rsp + 176]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 132
                let ax = x86_64::_mm256_sign_epi8(x, x);
        movaps xmmword ptr [rsp + 272], xmm1
        movaps xmm0, xmmword ptr [rsp + 32]
        movaps xmmword ptr [rsp + 256], xmm0
        movaps xmmword ptr [rsp + 304], xmm1
        movaps xmmword ptr [rsp + 288], xmm0
        mov r14, rbx
        mov rbx, r12
        mov r12, r15
        lea r15, [rsp + 352]
        mov rdi, r15
        lea rsi, [rsp + 256]
        lea rdx, [rsp + 288]
        call core::core_arch::x86::avx2::_mm256_sign_epi8
        movaps xmm0, xmmword ptr [rsp + 176]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 134
                let sy = x86_64::_mm256_sign_epi8(y, x);
        movaps xmmword ptr [rsp + 336], xmm0
        movaps xmm0, xmmword ptr [rsp + 32]
        movaps xmmword ptr [rsp + 320], xmm0
        lea r13, [rsp + 384]
        mov rdi, r13
        lea rsi, [rsp + 448]
        lea rdx, [rsp + 320]
        call core::core_arch::x86::avx2::_mm256_sign_epi8
        lea rdi, [rsp + 96]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 94
                let dot = x86_64::_mm256_maddubs_epi16(ax, sy);
        mov rsi, r15
        mov r15, r14
        mov rdx, r13
        call core::core_arch::x86::avx2::_mm256_maddubs_epi16
        lea r14, [rsp + 192]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 115
                let ones = x86_64::_mm256_set1_epi16(1);
        mov rdi, r14
        call core::core_arch::x86::avx::_mm256_set1_epi16
        lea r13, [rsp + 64]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 116
                let summed_pairs = x86_64::_mm256_madd_epi16(ones, x);
        mov rdi, r13
        mov rsi, r14
        lea rdx, [rsp + 96]
        call core::core_arch::x86::avx2::_mm256_madd_epi16
        lea r14, [rsp + 480]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 118
                x86_64::_mm256_cvtepi32_ps(summed_pairs)
        mov rdi, r14
        mov rsi, r13
        call core::core_arch::x86::avx::_mm256_cvtepi32_ps
        movaps xmm0, xmmword ptr [rsp + 128]
                // /home/ztf/daystudylib/criterion/src/lib.rs : 171
                acc = x86_64::_mm256_fmadd_ps(d, q, acc);
        movaps xmmword ptr [rsp + 80], xmm0
        movaps xmm0, xmmword ptr [rsp + 144]
        movaps xmmword ptr [rsp + 64], xmm0
        lea rdi, [rsp + 96]
        lea rsi, [rsp + 416]
        mov rdx, r14
        mov rcx, r13
        call core::core_arch::x86::fma::_mm256_fmadd_ps
        movdqa xmm0, xmmword ptr [rsp + 96]
        movaps xmm1, xmmword ptr [rsp + 112]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/cmp.rs : 1720
                fn lt(&self, other: &$t) -> bool { (*self) < (*other) }
        add r12, 34
        mov qword ptr [rsp + 16], r12
        add rbx, 34
        mov r12, rbx
        mov r14, qword ptr [rsp + 168]
        mov rbx, r15
        cmp r14, r15
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/range.rs : 764
                if self.start < self.end {
        je .LBB17_38
.LBB17_2:
                // /home/ztf/daystudylib/criterion/src/lib.rs : 162
                let d = x86_64::_mm256_set1_ps(x[i].d.to_f32() * (y[i].d.to_f32()));
        cmp qword ptr [rsp + 56], rbx
        je .LBB17_39
        movzx r13d, word ptr [r12 - 2]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs : 3342
                Relaxed => intrinsics::atomic_load_relaxed(dst),
        mov rax, qword ptr [rip + std_detect::detect::cache::CACHE@GOTPCREL]
        mov rax, qword ptr [rax]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect/src/detect/cache.rs : 107
                if cached == 0 {
        test rax, rax
        movdqa xmmword ptr [rsp + 144], xmm0
        movaps xmmword ptr [rsp + 128], xmm1
        je .LBB17_4
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect/src/detect/cache.rs : 110
                Some(test_bit(cached as u128, bit))
        movabs rcx, 281474976710656
        test rax, rcx
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 37
                if is_x86_feature_detected!("f16c") {
        jne .LBB17_18
.LBB17_5:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 614
                if i & 0x7FFFu16 == 0 {
        test r13d, 32767
        je .LBB17_6
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 618
                let half_sign = (i & 0x8000u16) as u32;
        mov eax, r13d
        and eax, 32768
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 619
                let half_exp = (i & 0x7C00u16) as u32;
        mov ecx, r13d
        and ecx, 31744
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 620
                let half_man = (i & 0x03FFu16) as u32;
        and r13d, 1023
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 636
                let sign = half_sign << 16;
        shl eax, 16
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 623
                if half_exp == 0x7C00u32 {
        cmp ecx, 31744
        jne .LBB17_15
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 625
                if half_man == 0 {
        test r13d, r13d
        je .LBB17_9
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 630
                mem::transmute::<u32, f32>((half_sign << 16) | 0x7FC0_0000u32 | (half_man << 13))
        shl r13d, 13
        or eax, r13d
        or eax, 2143289344
        movd xmm0, eax
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 97
                f16_to_f32_fallback(i)
        jmp .LBB17_19
.LBB17_6:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 615
                return unsafe { mem::transmute::<u32, f32>((i as u32) << 16) };
        shl r13d, 16
        movd xmm0, r13d
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 97
                f16_to_f32_fallback(i)
        jmp .LBB17_19
.LBB17_15:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 641
                if half_exp == 0 {
        test cx, cx
        je .LBB17_11
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 638
                let unbiased_exp = ((half_exp as i32) >> 10) - 15;
        movzx ecx, cx
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 652
                let exp = ((unbiased_exp + 127) as u32) << 23;
        shl ecx, 13
        and ecx, 260046848
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 653
                let man = (half_man & 0x03FFu32) << 13;
        shl r13d, 13
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 654
                unsafe { mem::transmute::<u32, f32>(sign | exp | man) }
        add ecx, r13d
        add ecx, 939524096
        or ecx, eax
        movd xmm0, ecx
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 97
                f16_to_f32_fallback(i)
        jmp .LBB17_19
.LBB17_9:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 626
                return unsafe { mem::transmute::<u32, f32>((half_sign << 16) | 0x7F80_0000u32) };
        or eax, 2139095040
        movd xmm0, eax
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 97
                f16_to_f32_fallback(i)
        jmp .LBB17_19
.LBB17_11:
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs : 136
                return intrinsics::ctlz(self as $ActualT);
        test r13d, r13d
        je .LBB17_12
        bsr dx, r13w
        xor edx, 15
        jmp .LBB17_14
.LBB17_12:
        mov dx, 16
.LBB17_14:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 647
                let man = (half_man << (14 + e)) & 0x7F_FF_FFu32;
        mov ecx, edx
        add cl, 8
        shl r13d, cl
        and r13d, 8388607
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 648
                return unsafe { mem::transmute::<u32, f32>(sign | exp | man) };
        or eax, 989855744
        shl edx, 23
        sub eax, edx
        or eax, r13d
        movd xmm0, eax
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 97
                f16_to_f32_fallback(i)
        jmp .LBB17_19
.LBB17_4:
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect/src/detect/cache.rs : 222
                .unwrap_or_else(|| detect_and_initialize().test(bit))
        call qword ptr [rip + std_detect::detect::cache::detect_and_initialize@GOTPCREL]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect/src/detect/cache.rs : 19
                x & (1 << bit) != 0
        bt rax, 48
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 37
                if is_x86_feature_detected!("f16c") {
        jae .LBB17_5
.LBB17_18:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 93
                unsafe { x86::f16_to_f32_x86_f16c(i) }
        mov edi, r13d
        call half::binary16::arch::x86::f16_to_f32_x86_f16c
.LBB17_19:
                // /home/ztf/daystudylib/criterion/src/lib.rs : 162
                let d = x86_64::_mm256_set1_ps(x[i].d.to_f32() * (y[i].d.to_f32()));
        cmp qword ptr [rsp + 48], rbx
        je .LBB17_40
        movd dword ptr [rsp + 32], xmm0
        mov rax, qword ptr [rsp + 16]
        movzx r13d, word ptr [rax - 2]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs : 3342
                Relaxed => intrinsics::atomic_load_relaxed(dst),
        mov rax, qword ptr [rip + std_detect::detect::cache::CACHE@GOTPCREL]
        mov rax, qword ptr [rax]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect/src/detect/cache.rs : 107
                if cached == 0 {
        test rax, rax
        je .LBB17_21
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect/src/detect/cache.rs : 110
                Some(test_bit(cached as u128, bit))
        movabs rcx, 281474976710656
        test rax, rcx
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 37
                if is_x86_feature_detected!("f16c") {
        jne .LBB17_35
.LBB17_22:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 614
                if i & 0x7FFFu16 == 0 {
        test r13d, 32767
        je .LBB17_23
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 618
                let half_sign = (i & 0x8000u16) as u32;
        mov eax, r13d
        and eax, 32768
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 619
                let half_exp = (i & 0x7C00u16) as u32;
        mov ecx, r13d
        and ecx, 31744
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 620
                let half_man = (i & 0x03FFu16) as u32;
        and r13d, 1023
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 636
                let sign = half_sign << 16;
        shl eax, 16
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 623
                if half_exp == 0x7C00u32 {
        cmp ecx, 31744
        jne .LBB17_32
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 625
                if half_man == 0 {
        test r13d, r13d
        movss xmm1, dword ptr [rsp + 32]
        je .LBB17_26
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 630
                mem::transmute::<u32, f32>((half_sign << 16) | 0x7FC0_0000u32 | (half_man << 13))
        shl r13d, 13
        or eax, r13d
        or eax, 2143289344
        movd xmm0, eax
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 97
                f16_to_f32_fallback(i)
        jmp .LBB17_37
.LBB17_23:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 615
                return unsafe { mem::transmute::<u32, f32>((i as u32) << 16) };
        shl r13d, 16
        movd xmm0, r13d
        jmp .LBB17_36
.LBB17_32:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 641
                if half_exp == 0 {
        test cx, cx
        movss xmm1, dword ptr [rsp + 32]
        je .LBB17_28
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 638
                let unbiased_exp = ((half_exp as i32) >> 10) - 15;
        movzx ecx, cx
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 652
                let exp = ((unbiased_exp + 127) as u32) << 23;
        shl ecx, 13
        and ecx, 260046848
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 653
                let man = (half_man & 0x03FFu32) << 13;
        shl r13d, 13
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 654
                unsafe { mem::transmute::<u32, f32>(sign | exp | man) }
        add ecx, r13d
        add ecx, 939524096
        or ecx, eax
        movd xmm0, ecx
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 97
                f16_to_f32_fallback(i)
        jmp .LBB17_37
.LBB17_26:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 626
                return unsafe { mem::transmute::<u32, f32>((half_sign << 16) | 0x7F80_0000u32) };
        or eax, 2139095040
        movd xmm0, eax
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 97
                f16_to_f32_fallback(i)
        jmp .LBB17_37
.LBB17_28:
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/uint_macros.rs : 136
                return intrinsics::ctlz(self as $ActualT);
        test r13d, r13d
        je .LBB17_29
        bsr dx, r13w
        xor edx, 15
        jmp .LBB17_31
.LBB17_29:
        mov dx, 16
.LBB17_31:
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 647
                let man = (half_man << (14 + e)) & 0x7F_FF_FFu32;
        mov ecx, edx
        add cl, 8
        shl r13d, cl
        and r13d, 8388607
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 648
                return unsafe { mem::transmute::<u32, f32>(sign | exp | man) };
        or eax, 989855744
        shl edx, 23
        sub eax, edx
        or eax, r13d
        movd xmm0, eax
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 97
                f16_to_f32_fallback(i)
        jmp .LBB17_37
.LBB17_21:
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect/src/detect/cache.rs : 222
                .unwrap_or_else(|| detect_and_initialize().test(bit))
        call qword ptr [rip + std_detect::detect::cache::detect_and_initialize@GOTPCREL]
                // /home/ztf/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/stdarch/crates/std_detect/src/detect/cache.rs : 19
                x & (1 << bit) != 0
        bt rax, 48
                // /home/ztf/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/half-2.4.1/src/binary16/arch.rs : 37
                if is_x86_feature_detected!("f16c") {
        jb .LBB17_35
        jmp .LBB17_22
.LBB17_39:
                // /home/ztf/daystudylib/criterion/src/lib.rs : 162
                let d = x86_64::_mm256_set1_ps(x[i].d.to_f32() * (y[i].d.to_f32()));
        lea rdx, [rip + .L__unnamed_5]
        mov rdi, qword ptr [rsp + 56]
        mov rsi, rdi
        call qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
.LBB17_40:
        lea rdx, [rip + .L__unnamed_6]
        mov rdi, qword ptr [rsp + 48]
        mov rsi, rdi
        call qword ptr [rip + core::panicking::panic_bounds_check@GOTPCREL]
```