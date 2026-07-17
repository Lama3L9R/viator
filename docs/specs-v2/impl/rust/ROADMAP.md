# Rust Implementation Roadmap

## Phase 0: Convert the Binary into an Engine Library

Create `src/lib.rs` and make `src/main.rs` a thin CLI adapter.

Replace the current `ViatorState` shape in `src/build/mod.rs`. The new engine
must not store CLI arguments or let a Lua global strongly own the state that
owns the Lua VM.

Remove production-unsafe prototype behavior, including the intentional crash
command and its dependency. Keep version reporting as a normal CLI service.

Replace the empty `Build` branch, empty plugin/project discovery methods, stub
`V.require`, and no-op action registration with explicit unsupported
diagnostics until their V2 implementations land. Fold `viator-utils` into the
main library unless a concrete cross-crate consumer justifies retaining it.

Deliverables:

- Reusable `Engine` API.
- `EngineRequest` and structured diagnostics.
- Temporary-directory test harness.
- No strong Lua/engine reference cycle.

## Phase 1: Owned V2 Models and Lua Loading

Implement owned project, pipeline, action, package, machine, and provider IDs.

Replace the current `autolua` project boundary in `src/build/lua`. Preserve
small generated bindings only where they improve a stable host API; manually
parse the project root and factory results.

Fix frontmatter parsing and enforce separate `dslVersion` and
`minViatorVersion` fields. Load the canonical V2 project shape with `group`,
project dependencies, and keyed targets.

Create a restricted VM profile with no ambient Lua module path, process,
network, environment, or unrestricted filesystem access. Remove
`ExecuteLua --env`, make it a development-only isolated command, or ensure it
cannot share a project/plugin session.

Deliverables:

- Project loader with chunk and field-path diagnostics.
- `PipelineDefinition` arena and target map.
- Frozen registries after plugin loading.
- Golden Lua loading tests.
- Restricted-VM integration tests.

## Phase 2: Typed Context and Planning

Replace the current `BuildContext.files: HashbrownMap<String, mlua::Value>` with
owned artifact schemas, promises, snapshots, selectors, scopes, and provenance.

Replace callback-style `Action { handler: Function }` with action definitions,
invocations, and native/Lua JobFactory adapters.

Implement shared and isolated `CallPipeline`, invocation IDs, and recursion
detection.

Implement the requirement-collection pass before provider resolution and the
versioned Lua factory wire protocol with promise tokens and path placeholders.

Deliverables:

- Artifact schema registry.
- Append-only planning ledger.
- Deterministic selectors.
- Stable call-site, planned-action, job, and artifact IDs.
- Requirement collection.
- JobFactory validation.
- Fake actions that create a nontrivial job DAG.

## Phase 3: Scheduler and Process Jobs

Enable the required asynchronous runtime features or select an equivalent
process runtime. Keep scheduler models independent from the runtime library.

Implement process jobs, resource limits, cancellation, event streaming, staging
directories, output validation, and dependency failure propagation.

Deliverables:

- DAG validator.
- `--jobs N` scheduling.
- Process-tree cancellation on supported platforms.
- Stable per-job logs.
- Fake-executor determinism tests.

## Phase 4: Store and Provisional Cache

Add canonical serialization and tagged BLAKE3 digests. Implement blob, tree, and
action-result storage with atomic commits and per-digest locking.

Deliverables:

- File and tree hashing.
- Action key framework for already resolved inputs.
- Cache hit and miss execution paths.
- Corruption detection.
- Concurrent import deduplication.

## Phase 5: Package and Provider Resolution

Replace the current closed package metadata model in
`src/build/package/metadata.rs` with versioned V2 metadata.

Implement component coordinates, static/dynamic representations, dependency
roles, package features, virtual capability providers, toolchain policy, and a
lockfile.

Deliverables:

- JSON metadata parser and canonical writer.
- Deterministic local resolver.
- Virtual provider selection.
- ABI key calculation.
- Lockfile creation, `--locked`, and `--frozen`.
- Final action keys including the locked package and provider graph.

## Phase 6: LLVM ARMv6 Vertical Slice

Integrate the Lua CLang plugin and LLVM toolchain provider with the scheduler.
Support Cortex-M0 with ARMv6-M, Thumb instruction state, little endian,
soft-float ABI, ELF format, no libc, and no dynamic loader.

Deliverables:

- One compile job per source.
- Conservative declared include-tree hashing.
- Static archive or linked ELF production.
- Optional raw and Intel HEX transforms.
- Incremental rebuild test.
- Target artifact inspection test.

Add x64 hosted support after the same semantic path works without special-case
logic in the scheduler.

## Phase 7: Local Publication

Implement immutable repository indexes and staged publication.

Add a typed `PackagePlanSpec` assembly step before publication. A publish job
resolves its promises into a `SealedPackagePlan` containing final digests,
representations, commands, resolved edges, and an immutable activation lock.

Deliverables:

- Publish exact component variant.
- Resolve the published component in another test project.
- Verify static dependency propagation.
- Verify dynamic runtime metadata.
- Verify package plans and activation locks.
- Reject coordinate/content replacement.

## Phase 8: Project Generation

Implement versioned template loading and the Rust side of `viator new`. Template
execution uses a restricted session and writes only through an explicit project
creation transaction.

Deliverables:

- `nostd` default template.
- CPU, MCU, and board template argument validation.
- No overwrite of unmanaged files.
- Template identity recorded in generated project metadata.

## Phase 9: Mounting

Implement runtime closure calculation, profile generations, collision preflight,
launcher-provider integration, pinning, rollback, and unmount.

Start with one well-defined user-profile strategy. Add privileged system mounts
only after root-owned store import and ownership rules are tested.

Deliverables:

- Wrapper or native launcher integration test.
- Private dynamic library closure.
- Command rename and collision behavior.
- Opaque obfuscated-executable fixture.
- Transaction rollback tests.

## Phase 10: External Extensions and Remote Services

Stabilize plugin manifests and permissions before external distribution.

Implement remote package repositories before remote cache and remote execution.
Repository authenticity, cache trust, and package resolution authority must
remain separate concerns.

## Suggested Dependencies

Evaluate these categories when their phases begin rather than adding all of them
immediately:

- BLAKE3 hashing.
- Semantic version parsing.
- Ordered maps and canonical JSON support.
- UTF-8 path handling or an explicit non-UTF-8 path policy.
- Glob matching.
- Temporary directories.
- Platform project-directory discovery.
- Asynchronous process execution.
- Archive creation.

Pin a Rust toolchain and define the minimum supported Rust version before the
plugin API becomes public.
