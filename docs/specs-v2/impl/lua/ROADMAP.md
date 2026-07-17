# Lua Implementation Roadmap

## Phase 0: Host API Mock and Type Definitions

Define Luau types for project declarations, pipeline handles, action
configurations, dependency requests, artifact handles, selectors, factory
requests, and job declarations.

Create a pure-Lua host mock for fast module tests. The Rust integration harness
remains authoritative for security and conversion behavior.

Deliverables:

- Versioned `V` API type definitions.
- Deterministic table and collection helpers.
- Structured diagnostic helper.
- Module-loading test fixtures.

## Phase 1: Core DSL

Implement:

```text
V.pipeline
V.callPipeline
V.action
V.dep
V.virtual
V.select
V.require
```

The DSL creates tagged handles and plain declarations. It does not execute
pipelines or resolve dependencies.

Deliverables:

- Canonical project example using one pipeline representation.
- Shared and isolated call declarations.
- Project and pipeline dependency ownership.
- Pipeline parameters and call arguments.
- Validation for common user errors.
- Generated project root tests.

## Phase 2: Registration and Factory Protocol

Implement Lua wrappers for:

```text
V.registry.action
V.registry.artifactType
V.registry.toolchainProvider
V.registry.capabilityProvider
V.registry.architecture
V.registry.loaderProvider
V.job.process
V.job.copy
V.job.archive
V.fs.glob
V.fs.readTracked
```

Factories must return serializable tables accepted by Rust validation. Add tests
that reject function values in returned persistent declarations, untracked
paths, duplicate output promises, and unknown artifact schemas.

Deliverables:

- Example plugin producing concurrent independent jobs.
- Typed context selection.
- Artifact promise helpers.
- Requirement collection callbacks.
- Tagged process argument placeholders.
- Rust/Lua protocol golden tests.

## Phase 3: CLang Artifact and Semantic Model

Register CLang and Native artifact schemas. Define normalized compile, archive,
link, and firmware semantic requests.

Deliverables:

- C and C++ source classification.
- Include and compile-interface model.
- Machine and ABI requirement propagation.
- Static and dynamic representation declarations.

## Phase 4: LLVM Provider and CompileAll

Implement the reference LLVM translation provider and
`CLang:CompileAll`.

Deliverables:

- One process job per source.
- Dependency discovery declarations.
- Structured diagnostic mode.
- Cortex-M0 ARMv6-M, Thumb, little-endian, soft-float, freestanding command
  generation.
- x64 hosted command generation.

## Phase 5: Linking and Runtime Providers

Implement `CLang:LinkObjects`, archive production, dynamic link interfaces,
runtime files, and standard-library helper modules.

Deliverables:

- Independent static and dynamic jobs.
- Executable link job and runtime roots.
- Link graph ordering.
- `std.c.auto()` and explicit provider requests.
- Compiler runtime and CRT separation.
- Embedded linker-script and startup inputs.
- Typed package-plan assembly.

## Phase 6: Project Templates

Implement versioned templates:

```text
nostd
hosted-c
hosted-cxx
embedded-cpu
embedded-mcu
embedded-board
```

The default template is `nostd`. Templates generate ordinary files and explicit
dependencies.

Register templates through `V.registry.template`; Rust owns destination
validation and transactional file creation.

Deliverables:

- `viator new` integration descriptors.
- Cortex-A8 CPU template.
- At least one ARMv6 MCU or board fixture.
- Template upgrade/version metadata.

## Phase 7: GNU Provider

Implement GNU translations for semantic options that can be supported
correctly. Unsupported options remain explicit diagnostics.

Do not delay the LLVM vertical slice while attempting complete GCC/binutils
coverage.

## Phase 8: External Plugin Examples

Create examples demonstrating:

- A custom generator action.
- A post-link protection action.
- A runtime provider.
- An external architecture provider.

External examples should use only the public versioned Lua API and must not
depend on internal Rust userdata layouts. Begin this phase only after the Rust
plugin manifests, permission model, and provider wire protocols are stable.
