# Rust Implementation Overview

## Responsibilities

Rust implements the trusted engine:

- CLI parsing and project discovery.
- Restricted Luau VM hosting.
- Conversion from Lua declarations into owned models.
- Registries for actions, artifact schemas, architectures, capabilities, and
  toolchain providers.
- Dependency and virtual-provider resolution.
- Pipeline planning and `CallPipeline` expansion.
- Typed artifact context and provenance.
- Job graph construction, validation, and concurrent scheduling.
- Process execution and cancellation.
- Hashing, cache, and immutable local store.
- Package publication and metadata validation.
- Runtime closure calculation and mount transactions.
- Structured diagnostics and event rendering.

Rust does not hard-code every language action or compiler flag. Those policies
belong to language and toolchain providers, commonly implemented in Lua.

## Proposed Source Layout

Begin with one library crate and a thin binary:

```text
src/
  lib.rs
  main.rs
  cli/
  engine/
  lua/
  model/
  registry/
  resolution/
  pipeline/
  artifact/
  jobs/
  scheduler/
  process/
  machine/
  package/
  store/
  repository/
  mount/
  diagnostics/
```

Do not split these modules into workspace crates until stable interfaces or
compile-time pressure justify it. Small utility code should remain near its
consumer rather than accumulating in a generic utility crate.

## Engine Lifetime

The current prototype lets a Lua global retain a strong handle to the state that
owns the Lua VM. Replace that self-referential shape.

Use an explicit session lifetime:

```text
Engine
  immutable configuration
  registries
  resolver
  store
  scheduler

LuaSession<'engine>
  Lua VM
  weak or borrowed host interfaces
  temporary Lua registry keys
```

The engine creates a Lua session for configuration and planning. Host functions
borrow narrowly scoped services or use weak handles. Destroying the session
releases every Lua function and value before concurrent execution starts.

CLI arguments should not be stored inside the engine. The CLI builds an
`EngineRequest` and passes it to a reusable library API.

## Concurrency Boundary

Planning may remain single-threaded because `mlua` and Lua job factories belong
to one coordinator. Scheduler inputs are fully owned and `Send`.

The scheduler executes jobs concurrently. A runtime such as Tokio can provide:

- Asynchronous process and pipe handling.
- Cancellation.
- Resource semaphores.
- Network fetch support.
- Event channels.

The public job graph must not depend on Tokio-specific types. This keeps models
testable and permits a deterministic fake executor.

## Implementation Documents

- [Architecture](ARCHITECTURE.md)
- [Roadmap](ROADMAP.md)
