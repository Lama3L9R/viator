

export type Arch =
    'x64' |
    'armv9' | 'armv8' | 'armv7' | 'armv6' |
    'riscv32' | 'riscv64' |
    'xtensa' |
    'mips'

export type X64Extensions =
    'avx' | 'avx2' | 'avx512' |
    'sse' | 'sse2' | 'sse3' | 'sse4.1' | 'sse4.2' |
    'baseline' | 'v2' | 'v3' | 'v4' // should break down into compiler level exts when impl this

export type ArmExtensions =
    'vfpv2' | 'vfp' | 'software' | 'thumb' | // v6
    'vfpv3' | 'vfpv4' | 'neon' | // v7
    'v8' | 'v8.1' | 'v8.2' | 'v8.3' | 'v8.4' | 'v8.5' | 'v8.6' | 'v8.7' | 'v8.8' | 'v8.9' | 'sve' | 'sve2' | // v8
    'v9' | 'v9.1' | 'v9.2' | 'v9.3' | 'v9.4' | 'v9.5' | 'v9.6' | 'v9.7' // v9

export type ArchExtensions = X64Extensions | ArmExtensions

export type BinaryWrapperFormat = 'elf' | 'pe' | 'mach-o' | 'bare' | 'bare-hex'
