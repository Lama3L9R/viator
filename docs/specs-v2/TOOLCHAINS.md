# Toolchains

## Language Support and Toolchains

`CLang` is the C/C++ Language Support Plugin. The name does not refer to LLVM
Clang.

CLang defines compiler-independent semantics for:

- C and C++ source discovery.
- Language standard selection.
- Include trees and definitions.
- Warning and diagnostic policies.
- Optimization and debug information.
- Position-independent code.
- Visibility.
- CPU and ABI requirements.
- Static and dynamic linking.
- Link-time optimization.
- Sanitizers and runtime instrumentation.
- Generated dependency information.

A language requirement pass derives runtime capabilities from selected
semantics before provider resolution. A toolchain provider contributes its own
provider dependencies and later translates resolved semantics into commands.
Command translation MUST NOT discover a new unresolved capability.

## Default Policy

LLVM-based toolchains are the global default and reference implementation.
Projects that use only portable CLang semantics are expected to work with the
reference LLVM provider.

GNU-based compilers are supported through lower-priority providers. They do not
define core behavior. A GNU provider must either implement a requested semantic
option correctly or report that it is unsupported. It must not silently replace
the option with an approximate flag.

Other compiler families may be supplied by plugins.

## Toolchain Provider

A provider describes:

```text
provider identity and version
execution-machine requirements
supported target architectures and features
C and C++ compiler commands
assembler
linker or compiler-driver link mode
archiver and ranlib
object inspection and conversion tools
resource directory
default capability-provider policy
supported diagnostic and dependency formats
deterministic-output capabilities
```

Toolchains should be resolvable packages with immutable manifests and digests.
A locally discovered system toolchain is represented by a probed provider and
is less reproducible than a locked toolchain package.

## Standard Libraries and Runtimes

The toolchain does not own the standard library permanently. It supplies a
default policy for unresolved virtual requests.

For example, a project may request `@cc:libc` without a provider and receive the
LLVM libc provider under the default LLVM policy. The project may explicitly
request LLVM libc while selecting a GNU compiler provider. Viator accepts the
combination only if all providers declare compatible target and ABI contracts.

Compiler runtime, C library, CRT, C++ ABI, C++ standard library, and unwind
runtime remain separate dependencies. This separation is required for
freestanding builds and mixed provider configurations.

## Semantic Command Translation

CLang job factories produce semantic compile and link requests. Toolchain
providers translate them into structured executable and argument arrays.

The normalized semantic request participates in cache identity. The translated
command also participates when backend differences can affect output.

Raw compiler or linker arguments are an explicit escape hatch:

```luau
V.action("CLang:CompileAll", {
    rawCompilerArgs = { "-fexample" },
})
```

Raw arguments:

- Are never interpreted by Viator as portable semantics.
- Participate verbatim in action identity.
- May restrict the action to one toolchain provider.
- Are shown as a portability warning in diagnostics.

Providers MUST reject raw arguments that override managed outputs or bypass the
declared tool, input, environment, and staging model. Files referenced by raw
arguments, including response files, plugins, linker scripts, include paths,
and library paths, require explicit input declarations.

An argument the provider cannot classify is rejected by default. An explicit
unsafe mode may permit it only inside an enforced sandbox with read-only inputs,
managed output allowlisting, and tool confinement. The resulting job is
`uncontrolled`, uncacheable, and nonpublishable unless the user applies a
separate recorded override.

Viator should not become a generic carrier for `CFLAGS`, `CXXFLAGS`, and
`LDFLAGS` inherited from the ambient environment.

## Compiler Portability and Reproducibility

Compiler portability and byte reproducibility are separate guarantees.

Portable semantics mean the same build description can be translated by every
provider that claims support. They do not mean GCC and Clang will emit identical
bytes.

Byte reproducibility means the same locked provider, source graph, dependency
graph, semantic options, and declared environment produce the same outputs.

Changing the compiler, linker, archiver, standard library, or relevant provider
policy produces a different action identity and may produce a different package
variant or ABI key.

## Target Translation

Users select architecture, CPU templates, machine features, and runtime
capabilities. A toolchain provider may internally translate this model into a
compiler target triple, CPU argument, feature switches, sysroot, and linker
emulation.

That internal translation must be deterministic and versioned. Compiler
triplets may appear in verbose diagnostics but are not the primary package or
project interface.

## Conformance

A toolchain provider should pass language-plugin conformance tests covering:

- Compile and dependency discovery.
- Static archive production.
- Dynamic link production where a loader exists.
- Feature and ABI flag translation.
- Deterministic archive and debug path behavior.
- Structured diagnostics.
- Runtime-provider compatibility checks.

Core architecture support is complete only when at least one reference
toolchain provider passes the relevant target conformance suite.
