# Lua Implementation Overview

## Responsibilities

Luau provides the declarative project language and most high-level plugin
policy. It should be expressive enough to describe language support without
becoming a second scheduler or an unrestricted build-time shell.

Luau implements:

- Project and pipeline constructors.
- Action invocation helpers.
- Pipeline composition helpers.
- Dependency and virtual-capability requests.
- Typed artifact selectors.
- Lua action JobFactory registration.
- CLang C/C++ language support planning.
- Toolchain policy and semantic translation helpers.
- Project, CPU, MCU, and board templates.
- User-facing validation close to DSL calls.

Rust remains authoritative for final schema validation, dependency resolution,
job execution, hashing, storage, publication, and mounting.

## Proposed Module Layout

Bundled Luau modules may use a layout such as:

```text
lua/
  viator/
    bootstrap.luau
    dsl.luau
    artifact.luau
    jobs.luau
    dependencies.luau
    plugins/
      clang/
        init.luau
        actions.luau
        artifacts.luau
        stdlibs.luau
        toolchains/
          llvm.luau
          gnu.luau
    templates/
      nostd.luau
      hosted-c.luau
      embedded.luau
```

The final location may be embedded into the binary or installed as versioned
resources. Module identity and content digest must remain visible to the engine.

## Lua Lifecycle

Luau code runs in three coordinator phases:

1. Configuration loads modules and returns project declarations.
2. Requirement collection asks selected actions for package and capability
   requirements before Rust resolves providers.
3. Planning invokes registered Lua JobFactory functions against read-only,
   serializable requests and typed context snapshots.

Lua factories return plain tables. Rust converts them into owned jobs and
artifact promises immediately.

Lua code does not execute inside worker jobs and does not receive mutable shared
state. Concurrent work begins after job descriptions cross the Rust boundary.

## Host Capabilities

The Rust host exposes a small `V` API. Filesystem, environment, process, and
network operations are unavailable unless represented by tracked host methods.

Examples include:

```text
V.fs.glob
V.fs.readTracked
V.registry.action
V.registry.artifactType
V.registry.toolchainProvider
V.registry.capabilityProvider
V.registry.architecture
V.registry.loaderProvider
V.registry.template
V.job.process
V.select
V.require
V.dep
V.virtual
V.pipeline
V.callPipeline
V.action
```

Calling `V.job.process` declares a job. It does not run a process.

## Implementation Documents

- [DSL and Host API](DSL_AND_API.md)
- [CLang Plugin](CLANG_PLUGIN.md)
- [Roadmap](ROADMAP.md)
