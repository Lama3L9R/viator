# Implementation Proposal

## Purpose

This proposal translates the V2 specifications into an implementation sequence.
It is guidance rather than a stable public API. Data formats and plugin
protocols should receive schema versions before they become compatibility
commitments.

The implementation is divided into:

- [Rust implementation](rust/OVERVIEW.md): engine, validation, resolution,
  artifact model, scheduler, cache, repository, and mounting.
- [Lua implementation](lua/OVERVIEW.md): project DSL, plugin planning APIs,
  CLang language support, toolchain policy helpers, and templates.

## Responsibility Boundary

Rust owns every correctness-sensitive or concurrently mutated subsystem:

```text
filesystem and process capabilities
owned project and package models
dependency and provider resolution
pipeline and action normalization
typed artifact promises and sealed artifacts
job graph validation
concurrent scheduler
hashing, cache, and repository
mount transactions
structured diagnostics
```

Lua owns declarative policy and high-level planning:

```text
project descriptions
pipeline construction
action registration
language-specific semantic options
job-factory planning functions
toolchain policy helpers
project templates
plugin composition
```

Lua factories return owned, serializable declarations. Rust validates and
normalizes those declarations before execution. Lua functions and `mlua::Value`
instances never enter scheduler workers.

## End-to-End Flow

```text
1. CLI selects project, target, machine, and policy.
2. Rust creates a restricted Luau configuration session.
3. Rust loads pinned plugins and exposes the V host API.
4. Lua returns project, pipelines, actions, and dependency requests.
5. Rust converts persistent project declarations into owned models.
6. Rust and Lua action providers collect package and capability requirements.
7. Rust resolves and locks packages, representations, providers, and toolchains.
8. Pipeline planning calls native or Lua JobFactory implementations serially.
9. Factories append typed artifact promises and return Job specifications.
10. Rust validates the resulting job DAG and destroys the Lua session.
11. Rust executes ready jobs concurrently up to configured limits.
12. Successful jobs seal artifacts into the content-addressed store.
13. Explicit effect jobs publish or mount selected results.
```

Planning and execution are deliberately separate. The single-threaded Lua
coordinator can generate thousands of jobs without limiting concurrent process
execution afterward. Session-owned Lua factory functions remain alive through
serial planning and are released before worker execution.

## Primary Internal Models

The first stable internal model set should include:

```text
Project
PackageCoordinate and PackageRequirement
PipelineDefinition and PipelineInvocation
ActionDefinition and ActionInvocation
JobFactory and JobSpec
ArtifactType and ArtifactPromise
SealedArtifact
MachineRequest and MachineRequirements
CapabilityRequest and ResolvedProvider
RequirementSet
ResolutionGraph and Lockfile
JobGraph
PackagePlanSpec, SealedPackagePlan, and ActivationLock
Repository and Store
MountProfile and RuntimeClosure
DiagnosticEvent
```

These models use owned Rust strings, paths, IDs, and serialized payloads. Lua
tables are input syntax, not the engine's persistent data structures.

## Graph Boundaries

Do not combine the following graphs into one generic node structure initially:

- Package graph: versions, representations, capabilities, and provider edges.
- Pipeline call graph: reusable project planning composition.
- Job graph: concrete execution dependencies and resource constraints.

They can share ID and diagnostic utilities while retaining separate validation
and cycle semantics.

## Recommended Vertical Slice

The first useful end-to-end build should:

1. Load a Luau project containing one `build` target.
2. Load a bundled CLang plugin.
3. Resolve an LLVM toolchain provider with no standard library.
4. Plan one compile job per C source file.
5. Run compile jobs concurrently.
6. Link or archive the results for Cortex-M0 using ARMv6-M, Thumb instruction
   state, little endian, soft-float ABI, ELF format, no libc, and no loader.
7. Seal outputs into the local store.
8. Re-run without executing unchanged jobs.

This slice validates the central design before implementing remote repositories,
C++, system providers, complex mounting, or external plugin distribution.

## Compatibility Discipline

Before publishing any V2 package, assign independent versions to:

- Project DSL.
- Package metadata.
- Lockfile.
- Action factory protocol.
- Job specification.
- Artifact schemas.
- Plugin API.
- Repository protocol.
- Mount profile manifest.

Unknown optional extensions may be preserved. Unknown required schema versions
must fail with a diagnostic.
