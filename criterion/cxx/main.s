	.file	"main.cpp"
	.intel_syntax noprefix
	.text
	.globl	_Z19mm256_fmadd_ps_testv
	.type	_Z19mm256_fmadd_ps_testv, @function
_Z19mm256_fmadd_ps_testv:
.LFB4047:
	push	rbp
	mov	rbp, rsp
	and	rsp, -32
	sub	rsp, 104
	vmovss	xmm0, DWORD PTR .LC0[rip]
	vmovss	DWORD PTR -92[rsp], xmm0
	vbroadcastss	ymm0, DWORD PTR -92[rsp]
	vmovaps	YMMWORD PTR -88[rsp], ymm0
	vmovss	xmm0, DWORD PTR .LC1[rip]
	vmovss	DWORD PTR -96[rsp], xmm0
	vbroadcastss	ymm0, DWORD PTR -96[rsp]
	vmovaps	YMMWORD PTR -56[rsp], ymm0
	vmovss	xmm0, DWORD PTR .LC2[rip]
	vmovss	DWORD PTR -100[rsp], xmm0
	vbroadcastss	ymm0, DWORD PTR -100[rsp]
	vmovaps	YMMWORD PTR -24[rsp], ymm0
	vmovaps	ymm0, YMMWORD PTR -88[rsp]
	vmovaps	YMMWORD PTR 8[rsp], ymm0
	vmovaps	ymm0, YMMWORD PTR -56[rsp]
	vmovaps	YMMWORD PTR 40[rsp], ymm0
	vmovaps	ymm0, YMMWORD PTR -24[rsp]
	vmovaps	YMMWORD PTR 72[rsp], ymm0
	vmovaps	ymm1, YMMWORD PTR 40[rsp]
	vmovaps	ymm0, YMMWORD PTR 72[rsp]
	vfmadd231ps	ymm0, ymm1, YMMWORD PTR 8[rsp]
	nop
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE4047:
	.size	_Z19mm256_fmadd_ps_testv, .-_Z19mm256_fmadd_ps_testv
	.globl	main
	.type	main, @function
main:
.LFB4048:
	.cfi_startproc
	endbr64
	push	rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	mov	rbp, rsp
	.cfi_def_cfa_register 6
	and	rsp, -32
	sub	rsp, 104
	vmovss	xmm0, DWORD PTR .LC0[rip]
	vmovss	DWORD PTR -92[rsp], xmm0
	vbroadcastss	ymm0, DWORD PTR -92[rsp]
	vmovaps	YMMWORD PTR -88[rsp], ymm0
	vmovss	xmm0, DWORD PTR .LC1[rip]
	vmovss	DWORD PTR -96[rsp], xmm0
	vbroadcastss	ymm0, DWORD PTR -96[rsp]
	vmovaps	YMMWORD PTR -56[rsp], ymm0
	vmovss	xmm0, DWORD PTR .LC2[rip]
	vmovss	DWORD PTR -100[rsp], xmm0
	vbroadcastss	ymm0, DWORD PTR -100[rsp]
	vmovaps	YMMWORD PTR -24[rsp], ymm0
	vmovaps	ymm0, YMMWORD PTR -88[rsp]
	vmovaps	YMMWORD PTR 8[rsp], ymm0
	vmovaps	ymm0, YMMWORD PTR -56[rsp]
	vmovaps	YMMWORD PTR 40[rsp], ymm0
	vmovaps	ymm0, YMMWORD PTR -24[rsp]
	vmovaps	YMMWORD PTR 72[rsp], ymm0
	vmovaps	ymm1, YMMWORD PTR 40[rsp]
	vmovaps	ymm0, YMMWORD PTR 72[rsp]
	vfmadd231ps	ymm0, ymm1, YMMWORD PTR 8[rsp]
	nop
	vmovaps	YMMWORD PTR -24[rsp], ymm0
	mov	eax, 0
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE4048:
	.size	main, .-main
	.section	.rodata
	.align 4
.LC0:
	.long	1065353216
	.align 4
.LC1:
	.long	1073741824
	.align 4
.LC2:
	.long	1077936128
	.ident	"GCC: (Ubuntu 9.4.0-1ubuntu1~20.04.2) 9.4.0"
	.section	.note.GNU-stack,"",@progbits
	.section	.note.gnu.property,"a"
	.align 8
	.long	 1f - 0f
	.long	 4f - 1f
	.long	 5
0:
	.string	 "GNU"
1:
	.align 8
	.long	 0xc0000002
	.long	 3f - 2f
2:
	.long	 0x3
3:
	.align 8
4:
