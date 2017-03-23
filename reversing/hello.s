	.section	__TEXT,__text,regular,pure_instructions
	.macosx_version_min 10, 12
	.intel_syntax noprefix
	.globl	_main
	.align	4, 0x90
_main:                                  ## @main
## BB#0:
	push	ebp
	mov	ebp, esp
	sub	esp, 24
	call	L0$pb
L0$pb:
	pop	eax
	lea	eax, [eax + L_.str-L0$pb]
	mov	dword ptr [ebp - 4], 0
	mov	dword ptr [esp], eax
	call	_printf
	mov	ecx, 4660
	mov	dword ptr [ebp - 8], eax ## 4-byte Spill
	mov	eax, ecx
	add	esp, 24
	pop	ebp
	ret

	.section	__TEXT,__cstring,cstring_literals
L_.str:                                 ## @.str
	.asciz	"Hello, World!\n"


.subsections_via_symbols
