# CLang Language Support Plugin

## Meaning

`CLang` means C/C++ Language Support. It is independent from the LLVM Clang
compiler brand.

The plugin defines semantic compilation and linking. Toolchain providers, such
as LLVM or GNU, translate those semantics into concrete jobs.

## Modules

The bundled implementation may expose:

```text
viator/CLang@1
viator/CLangStdLibs@1
viator/CLangToolchainLLVM@1
viator/CLangToolchainGNU@1
```

LLVM is loaded by default policy. GNU support may be incomplete without
weakening the semantic contract.

## Artifact Schemas

CLang should register or use:

```text
CLang:CSource
CLang:CxxSource
CLang:HeaderTree
CLang:CompileInterface
Native:Object
Native:StaticLibrary
Native:DynamicLibrary
Native:Executable
Native:RuntimeFile
Firmware:LinkedImage
Firmware:EncodedImage
```

`Native:Object` attributes include source provenance, object format, machine
requirements, PIC state, language, and compile interface identity.

## Requirement Collection

Each CLang action supplies a requirement collector. It derives language,
toolchain, compiler-runtime, linker, and explicitly requested standard-library
capabilities without creating jobs.

Requirement collectors receive a symbolic declared-artifact context. A
generator can declare that it will produce C or C++ sources before its worker
job exists. CLang derives requirements from those shapes. A generator whose
output language or ABI cannot be known until execution requires an explicit
declaration in project configuration.

The resolver selects and locks providers after this pass. Provider dependency
metadata may add related requirements such as CRT or unwind support.
`create_jobs` then receives the completed provider graph and must not discover a
new unresolved capability.

## CompileAll

`CLang:CompileAll` performs planning only:

1. Resolve source files using tracked glob APIs or typed source artifacts.
2. Normalize semantic compile options.
3. Read resolved headers, compile interfaces, toolchain, and runtime providers.
4. Create one object promise per source file.
5. Ask the selected toolchain provider to translate each semantic request.
6. Return one process job per translation unit.

Illustrative configuration:

```luau
V.action("CLang:CompileAll", {
    sourceRoot = "src",
    include = { "**/*.c", "**/*.cc" },
    standard = {
        c = "c23",
        cxx = "c++23",
    },
    optimization = "release",
})
```

The resulting jobs are independent unless generated headers or other declared
inputs create edges. The Rust scheduler controls actual concurrency.

The action must not compile every source in one subprocess merely to simplify
progress reporting. Per-translation-unit jobs are required for concurrency and
incremental caching.

## Header Dependencies

The toolchain provider declares its dependency discovery mechanism, such as
depfiles or structured scanning.

An initial implementation may conservatively hash all declared include trees.
Precise header dependency discovery should replace that behavior once cache-hit
validation is correct.

Generated headers are typed artifact inputs and create explicit graph edges.

## LinkObjects

`CLang:LinkObjects` selects compatible object and dependency link artifacts from
the typed context.

```luau
V.action("CLang:LinkObjects", {
    name = "example",
    kind = "library",
    static = true,
    dynamic = true,
})
```

The action may produce two jobs:

```text
archive example static representation
link example dynamic representation
```

Both jobs depend on the required object jobs but not necessarily on each other,
so they may execute concurrently.

The final product name is required because it is a public artifact boundary.
Individual object files do not need user-assigned names.

Link input order derives from the resolved component graph and explicit group
semantics. Context insertion order and job completion timing are never link
order.

Static and dynamic outputs are representations of the same logical component.
The plugin records representation-specific link and runtime dependencies.

Executable production uses the same semantic link planner with an explicit
kind:

```luau
V.action("CLang:LinkObjects", {
    name = "example-cli",
    kind = "executable",
    dynamic = true,
    command = "example",
})
```

This produces `Native:Executable` plus its runtime roots. Static archives,
shared libraries, and executables have distinct output schemas even when they
share object selection and linker option logic.

## Standard Library Helpers

`viator/CLangStdLibs` returns dependency request helpers, not concrete host
paths:

```luau
local std = V.require("viator/CLangStdLibs@1")

local dependencies = {
    std.c.auto(),
}
```

The request must be attached to the project or selected pipeline:

```luau
local build = V.pipeline {
    dependencies = dependencies,
    -- Build entries follow.
}
```

`auto()` creates a virtual capability request. Toolchain policy selects and
locks the provider later.

Explicit providers remain possible:

```luau
std.c.provider("llvm:libc", "20.1.0")
std.cxx.provider("llvm:libc++", "20.1.0")
```

Compiler runtime, libc, CRT, C++ ABI, C++ standard library, and unwind runtime
must remain separate internally even when a convenience helper requests a
compatible bundle.

A `nostd` project does not call these helpers.

## Toolchain Translation

CLang builds a semantic request containing:

```text
language and standard
source and output artifacts
include interfaces and definitions
optimization and debug policy
warnings
visibility and PIC
machine and ABI requirements
LTO and sanitizer settings
raw provider arguments
```

The selected provider translates this into a `V.job.process` declaration and
reports unsupported semantics explicitly.

LLVM is the conformance reference. GNU translation should reuse the same
semantic request and may report unsupported fields. Build files should not need
to branch on `compiler == "gcc"` for ordinary operations.

## Embedded Linking

Embedded linking consumes explicit provider artifacts:

- Compiler runtime.
- Optional C library.
- Startup implementation.
- BSP and memory layout.
- Linker script.
- Entry point.

The absence of libc is valid. No `none` OS target or fake libc package is
required.

Firmware transformations use separate actions or providers:

```text
Firmware:Link
Firmware:ToRawBinary
Firmware:ToIntelHex
```

These transformations preserve the linked ELF as a distinct artifact.

## Package Assembly

Before publication, `V:AssemblePackage` consumes selected final artifact
promises and creates a typed `V:PackagePlanSpec`:

```luau
V.action("V:AssemblePackage", {
    kind = "library",
    headers = V.select("CLang:HeaderTree"),
    static = V.select("Native:StaticLibrary"),
    dynamic = V.select("Native:DynamicLibrary"),
})

V.action("V:PublishLocal", {
    package = V.select("V:PackagePlanSpec"),
})
```

The plan spec includes resolved dependency edges and promise references. The
publish job waits for every promise, validates sealed files, computes final
digests and the executable activation lock, and produces a sealed package plan.
The repository does not infer component structure from file names.

## Protection and Post-Link Actions

Post-link plugins consume `Native:Executable` and produce another typed
executable. CLang does not assume that its first linked output is the published
one.

For example:

```luau
V.action("Vendor:DoVMProtect", {
    input = V.select("Native:Executable", { label = "release" }),
})
```

The protection action declares any runtime dependency changes. Publication and
mounting treat the result as opaque and do not rewrite it automatically.

## Tests

The CLang plugin needs:

- Pure planning tests using a fake toolchain provider.
- Command golden tests for LLVM.
- Per-source job-count tests.
- Object selector compatibility tests.
- Static and dynamic representation tests.
- Header dependency tests.
- Cortex-M0 ARMv6-M, Thumb, little-endian, soft-float, freestanding integration
  tests.
- x64 hosted integration tests.
- GNU provider conformance tests for supported semantics.

Tests should assert semantic requests before concrete command strings whenever
possible. This keeps language behavior separate from provider translation.
