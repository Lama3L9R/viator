# Vision

## Problem

C and C++ projects commonly depend on undocumented host state: globally
installed headers, linker search paths, compiler defaults, platform-specific
scripts, environment variables, package-manager conventions, and build-system
probing. Two machines can accept the same project description and build
different dependency graphs or incompatible binaries.

Existing tools often preserve these behaviors for compatibility. Viator is a
greenfield system and does not need to preserve a convention merely because it
is old or widespread.

## Goal

Viator's primary goal is to make native builds reproducible and predictable by
turning implicit environmental assumptions into explicit, resolvable data.

A Viator build should answer these questions before execution:

- Which source and generated inputs are used?
- Which exact package versions and representations are selected?
- Which toolchain and runtime providers are selected?
- Which machine capabilities does each artifact require?
- Which jobs will run, and what are their declared inputs and outputs?
- Why is an artifact compatible or incompatible with a consumer?
- Which runtime files are needed when a published command is mounted?

## Principles

### Dependencies over host state

Libraries, standard libraries, compiler runtimes, startup objects, loaders,
kernel interfaces, and embedded board support should be modeled as
dependencies or capabilities. Ambient system discovery is an explicit escape
hatch, not the default dependency mechanism.

### Semantic build descriptions

Projects should express intent such as C standard, optimization, visibility,
linkage, and required machine features. Toolchain providers translate that
intent into concrete compiler and linker arguments. A build file should not be
a portable-looking wrapper around opaque `CFLAGS`.

### Explicit incompatibility

Viator should reject unsupported or ambiguous combinations. It should not
silently downgrade a language standard, replace an explicitly selected runtime
provider, ignore an unknown machine feature, or choose an arbitrary package
variant.

### Isolation by default

Building and publishing a package must not modify the host environment. A
package becomes globally or user-profile accessible only through an explicit,
transactional mount.

### Extensibility without core pollution

Language support, architectures, toolchains, artifact types, actions, package
providers, and launch strategies can be extended by plugins. Unsupported
legacy behavior does not belong in core merely to make an extension possible.

### LLVM as the reference, not a prison

LLVM-based toolchains define the default and best-supported behavior. The
language model remains toolchain-independent, and another backend may implement
the same semantics. Unsupported semantic options must produce errors rather
than approximate behavior silently.

### Reproducibility is a contract

For a hermetic build, the same source graph, lockfile, toolchain, configuration,
and declared inputs must produce the same action graph and outputs. A build
that consumes undeclared host state must be identified as non-hermetic.

## Scope

Viator is responsible for:

- Project and workspace discovery.
- Luau configuration and plugin loading.
- Dependency and virtual-provider resolution.
- Machine and package variant matching.
- Build planning and concurrent job execution.
- Content hashing and local caching.
- Local package publication.
- Runtime closure construction and explicit mounting.

Remote repositories, remote caching, and remote execution may be added after
the local model is stable.

## Architecture Policy

Core supports x64 and selected ARM, RISC-V, and embedded architectures only
when each has a defined feature model and conformance coverage. ARMv6 remains a
first-class requirement because embedded support is a project goal.

Core intentionally excludes 32-bit x86. A plugin may register it with its own
architecture definition, feature validation, toolchain adapters, and tests.
There must be no IA-32-specific compatibility path in the Rust core.

The machine used to run Viator is separate from the machine for which Viator
produces artifacts. Running Viator on an x64 host does not imply that every
x64-host convention is available to a target build.

## Non-Goals

Viator is not intended to:

- Reproduce CMake's command language or implicit discovery behavior.
- Automatically consume arbitrary globally installed libraries.
- Preserve all compiler-specific flags behind a misleading portable API.
- Make output from different compilers byte-identical.
- Patch published binaries automatically to make mounting convenient.
- Add legacy target support to core without a maintained implementation.
- Execute package metadata as code during dependency resolution.

The design rule is simple: retain conventions that provide clear value, and
replace conventions that make native builds ambiguous, fragile, or difficult
to reproduce.
