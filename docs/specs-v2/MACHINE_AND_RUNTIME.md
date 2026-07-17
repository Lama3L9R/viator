# Machine and Runtime Model

## No Primary OS Triplet

Viator does not use a traditional `architecture-vendor-os-environment` triplet
as its primary project or package selector. Triplets are useful compiler
implementation details, but they are a poor user model for embedded systems and
do not fully describe modern runtime dependencies.

The target environment is assembled from:

- A machine request.
- Code-generation and ABI requirements.
- A selected toolchain.
- Resolved runtime and platform capabilities.

An operating-system label is not required in package coordinates. If a binary
requires a Linux-compatible kernel API, a dynamic loader, or a particular C
library ABI, those requirements appear through provider dependencies.

## Machine Request

A machine request contains:

```text
architecture
optional CPU template
available machine features
requested machine features
```

The CPU template is a convenience profile. For example, `cortex-a8` expands to
a known set of available instructions and constraints. It does not select a C
library, kernel API, loader, board, or linker script.

A CPU template declares exactly one architecture or an explicit set of valid
architecture modes. When architecture is omitted, the CPU template supplies it.
An explicit conflicting architecture is an error.

Package metadata records required features, not every feature available on the
build machine. A binary compiled for Cortex-A8 without NEON instructions should
not unnecessarily require NEON merely because the CPU supports it.

## ABI-Relevant Requirements

Not every compatibility property is a CPU feature. ARM instruction state,
endianness, floating-point calling convention, and similar choices can make two
binaries incompatible even on the same processor.

Viator represents these as canonical machine requirements or ABI-provider
requirements. They participate in the computed ABI key even when they are not
shown as an OS triplet.

Examples include:

```text
arm-instruction-set-thumb
arm-float-abi-soft
arm-float-abi-softfp
arm-float-abi-hard
endian-little
endian-big
```

The exact names and implication rules belong to versioned architecture
providers. Unknown or contradictory requirements are errors.

## Runtime Composition

The runtime environment is a resolved dependency graph. Depending on the
project, it may include:

- Compiler runtime.
- C library.
- C startup objects.
- C++ ABI and standard library.
- Unwind runtime.
- Dynamic loader.
- Kernel or platform API.
- Embedded board support package.

Absence is meaningful. A project that does not request a C library is
freestanding; it does not need to select an OS named `none`.

Producing a shared library, producing a dynamically linked executable, and
mounting that executable have separate requirements. A shared library may carry
an unresolved loader capability requirement. A runnable dynamic executable and
a mount MUST resolve a concrete compatible loader strategy.

Toolchain policy may select default providers only for capabilities the project
requests. It does not add a standard library to a `nostd` project automatically.

## ABI Key

Architecture and CPU features are intentionally the primary visible variant
attributes. Viator also computes an ABI key from every ABI-relevant input:

- Canonical architecture requirements.
- ABI-affecting code-generation settings.
- Object format.
- Selected runtime provider ABI identities.
- Public dependency ABI requirements.
- Toolchain-defined compatibility information where required.

The ABI key is a compatibility guard, not a replacement for readable metadata.
When two variants fail to match, diagnostics should report the incompatible
requirements rather than only displaying different hashes.

## Object and Image Formats

Object format remains explicit because consumers and tools must know how to
read an artifact. Core formats may include ELF, PE, and Mach-O.

Raw firmware and Intel HEX are output encodings, not object formats. An embedded
pipeline commonly produces:

```text
linked ELF -> raw binary
linked ELF -> Intel HEX
linked ELF -> debug information
linked ELF -> map file
```

Each transform creates a new typed artifact with its own digest and provenance.

## Architecture Policy

Core includes x64 but excludes 32-bit x86. Core may include ARMv6, ARMv7,
ARMv8, ARMv9, RISC-V, and other architectures only with defined feature rules,
toolchain support, and conformance tests.

The canonical serialized core ID is `x64`. CLI aliases such as `x86_64` or
`amd64` may normalize to `x64`, but metadata and lockfiles contain only the
canonical ID. Names representing 32-bit x86 require an external provider.

ARMv6 support must account for distinct profiles. A Cortex-M0-class machine and
an ARM1176-class machine are not interchangeable merely because both contain
`armv6` in a compiler target name.

External plugins may register additional architecture IDs, including 32-bit
x86. Plugin architecture IDs are namespaced unless promoted into the core
registry. Unknown architecture IDs never fall back to a generic target.

## CPU, MCU, and Board Templates

CLI templates make detailed hardware knowledge convenient without putting that
knowledge into package coordinates:

```text
viator new example --cpu cortex-a8
viator new firmware --cpu cortex-m0 --template nostd
viator new firmware --mcu stm32f030
viator new firmware --board rp2040-pico
```

A CPU template defines processor capabilities. An MCU or board template may
also select a BSP dependency, memory layout, startup implementation, linker
script, and default output encodings.

Generated projects default to `nostd` unless another template explicitly
requests runtime facilities.

## Cross Compilation

Viator distinguishes the execution machine from the target machine:

- Build tools and compiler executables run on the execution machine.
- Produced objects and libraries target the requested machine.
- Code generators run on the execution machine but may produce target-specific
  files.
- Target binaries are never executed during configuration unless an explicit
  runner, emulator, or hardware provider is configured.

This distinction is internal and does not require exposing a traditional target
triplet as the primary user interface.
