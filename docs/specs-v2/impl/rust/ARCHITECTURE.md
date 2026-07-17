# Rust Architecture

## Owned IDs

Use explicit newtypes for persistent identities:

```rust
struct PipelineId(String);
struct PipelineCallSiteId(String);
struct PipelineInvocationId(String);
struct ActionTypeId(String);
struct ActionEntryId(String);
struct PlannedActionId(String);
struct JobId(String);
struct ArtifactTypeId(String);
struct ArtifactRef(String);
struct CapabilityId(String);
struct ProviderId(String);
struct ArchId(String);
```

IDs that enter metadata or lockfiles use stable canonical strings. Process-local
arena indexes may accelerate lookup but never become serialized identity.

## Project Loading

The Lua boundary should perform manual conversion and validation rather than
derive direct `FromLua` implementations for the complete project model.

Manual conversion provides:

- Field-path diagnostics.
- Unknown-field errors.
- Schema version checks.
- Target names derived from map keys.
- Stable table iteration rules.
- Conversion of handles into owned IDs.
- Rejection of functions in persistent configuration.

Frontmatter parsing should use `split_once`, trim keys and values, propagate I/O
errors, and recognize separate `dslVersion` and `minViatorVersion` fields before
evaluation. Plugin API versions belong to plugin manifests rather than the
project frontmatter.

The returned root model contains `group`, `name`, `version`, project dependency
requests, a target map, and project constraints. A target value resolves to a
`PipelineDefinition` handle.

Manual table conversion guarantees chunk names and field-path diagnostics.
Exact table-literal spans are optional unless DSL constructors record source
provenance explicitly.

## Registries

Use separate registries rather than one untyped string map:

```text
ActionRegistry
ArtifactSchemaRegistry
ArchitectureRegistry
CapabilityRegistry
ProviderRegistry
ToolchainRegistry
LuaModuleRegistry
```

Registration rejects duplicate IDs unless an explicit, versioned replacement
policy permits them. Registries preserve provider provenance for diagnostics and
cache identity.

Registries are mutable during plugin loading and frozen before project
resolution and planning. This avoids concurrent registration races.

## Pipeline Model

```rust
struct PipelineDefinition {
    id: PipelineId,
    entries: Vec<PipelineEntry>,
    parameters: ParameterSchema,
    constraints: PipelineConstraints,
}

enum PipelineEntry {
    Action(ActionInvocation),
    CallPipeline(CallPipeline),
}

struct TargetEntry {
    name: String,
    pipeline: PipelineId,
}
```

Definition IDs derive from module identity plus stable constructor provenance.
Each pipeline entry has an `ActionEntryId` or `PipelineCallSiteId`. A planned
action ID derives from the caller invocation plus entry ID. Factory-local job
keys derive stable `JobId` and artifact references. Moving an entry may
invalidate its identity, but two identical entries at distinct call sites never
collide.

`CallPipeline` expands the referenced definition with normalized arguments and a
new invocation scope. Shared calls append into the same context lineage.
Isolated calls plan against a child context and expose only validated exports.

Maintain an invocation stack during planning and report the complete call path
when recursion is detected.

## Action and JobFactory

Separate action definitions from invocations:

```rust
struct ActionDefinition {
    id: ActionTypeId,
    schema: ActionSchema,
    factory: JobFactoryHandle,
    implementation_digest: Digest,
}

struct ActionInvocation {
    entry_id: ActionEntryId,
    action_type: ActionTypeId,
    config: CanonicalValue,
}
```

The logical factory protocol is:

```rust
trait NativeJobFactory {
    fn collect_requirements(
        &self,
        request: &ActionRequirementRequest,
        context: &RequirementContext,
        services: &RequirementServices,
    ) -> Result<RequirementResult, Diagnostic>;

    fn create_jobs(
        &self,
        request: &ActionPlanRequest,
        context: &ArtifactSnapshot,
        services: &PlanningServices,
    ) -> Result<JobFactoryResult, Diagnostic>;
}

struct JobFactoryResult {
    jobs: Vec<JobSpec>,
    artifacts: Vec<ArtifactPromiseSpec>,
    diagnostics: Vec<Diagnostic>,
}

struct RequirementResult {
    requirements: RequirementSet,
    declared_artifacts: Vec<DeclaredArtifactShape>,
}
```

Requirement collection runs before provider resolution. Provider manifests may
expand the requirement graph through their own dependencies. `create_jobs` sees
the locked provider graph and may not introduce unresolved capabilities.

`RequirementContext` is a provider-independent symbolic preplan. Earlier
actions may declare artifact shapes such as "generated C source" without a
path, producer job, or selected toolchain. Later collectors can derive language
and runtime requirements from those shapes. If an action cannot know an
ABI-relevant output type until execution, the project must declare that type
explicitly; provider resolution cannot depend on runtime discovery.

Lua factories use a `LuaJobFactory` adapter evaluated by `LuaSession`. The
adapter serializes the request, calls the Lua function, and validates its plain
data result into the same `JobFactoryResult`.

`JobFactoryHandle` distinguishes native factories from session-local Lua
factory IDs. Lua registry keys remain owned by `LuaSession` through requirement
collection and planning. They are never stored in the final `JobGraph`.

## Provider Definitions

Rust validates versioned owned provider definitions:

```text
ToolchainProviderDefinition
  identity, execution requirements, target matcher, capability dependencies,
  semantic translator factory

CapabilityProviderDefinition
  provided capability, cardinality, compatibility attributes, dependencies,
  exported artifacts

ArchitectureDefinition
  canonical ID, aliases, CPU templates, feature implications, ABI validation

LoaderProviderDefinition
  capability identity, runtime constraints, supported isolation grades,
  launcher planning factory
```

Lua callbacks referenced by these definitions run only during requirement
collection or planning. A loader callback lowers a mount request into an owned
`LoaderPlan` embedded in the `MountJob`. Scheduler execution never calls back
into a destroyed Lua session.

A factory is side-effect-free except for tracked planning services such as
source globbing. It never runs compiler processes.

## Job Model

Begin with a small set of job implementations:

```rust
enum JobKind {
    Process(ProcessJob),
    Copy(CopyJob),
    Archive(ArchiveJob),
    Fetch(FetchJob),
    Publish(PublishJob),
    Mount(MountJob),
}
```

Do not require every job kind to be cacheable. Effect jobs explicitly disable
caching.

```rust
struct JobSpec {
    id: JobId,
    kind: JobKind,
    inputs: Vec<JobInput>,
    outputs: Vec<JobOutput>,
    control_dependencies: Vec<JobId>,
    resources: ResourceRequest,
    environment: EnvironmentSpec,
    sandbox: SandboxSpec,
    cache: CachePolicy,
    discovery: Option<DiscoverySpec>,
}
```

Artifact inputs create edges to their producer jobs. Explicit control edges are
used only when ordering has no data representation.

Validate before execution:

- Unique job IDs.
- Acyclic dependencies.
- Exactly one producer per output identity.
- No overlapping output paths inside one staging namespace or output tree.
- Satisfiable artifact selectors.
- Valid executor and target-machine roles.
- Declared resources and permissions.

## Typed Artifact Ledger

The planning ledger is a persistent append-only collection:

```rust
struct ArtifactRecord {
    reference: ArtifactRef,
    artifact_type: ArtifactTypeId,
    schema_version: u32,
    payload: CanonicalValue,
    producer: ProducerRef,
    scope: PipelineInvocationId,
    label: Option<String>,
    state: ArtifactState,
}

enum ArtifactState {
    Promised { producer_job: JobId },
    Sealed { digest: Digest, store_path: StoreRef },
}
```

The context API returns snapshots with deterministic iteration. Selector results
are fixed during planning and do not depend on worker completion order.

Artifact state transitions belong to the executor result store, not mutable Lua
payloads. An implementation may keep promised and sealed records in separate
maps while presenting one logical handle.

## Scheduler

The scheduler maintains:

- Remaining dependency count per job.
- Ready queues.
- CPU, memory, I/O, network, and named-resource limits.
- Running process handles.
- Cancellation state.
- Event channels.
- Result and failure records.

Stable job IDs determine diagnostic ordering, not execution priority. Scheduling
policy may optimize throughput without changing graph semantics.

`--jobs 1` applies a global active-job limit of one. CPU, memory, network, and
named-resource limits apply in addition. It does not use a different execution
path.

On failure, terminate the process tree, discard staging outputs, and cancel
dependent jobs. POSIX implementations use process groups. Windows
implementations use Job Objects where possible.

## Process Execution

```rust
struct ProcessJob {
    executable: ToolRef,
    arguments: Vec<ProcessArg>,
    working_directory: WorkingDirectorySpec,
    stdin: InputStreamSpec,
    stdout: OutputStreamSpec,
    stderr: OutputStreamSpec,
    timeout: Option<Duration>,
}

enum ProcessArg {
    Literal(OsString),
    InputPath(ArtifactRef),
    OutputPath(ArtifactRef),
    TemporaryPath(TempRef),
}
```

Lua factory wire values use factory-local promise tokens. Rust assigns stable
artifact references and materializes `InputPath`, `OutputPath`, and
`TemporaryPath` only when creating the private execution directory.

Commands are not shell strings. The executable is a declared tool artifact or a
tracked system provider. Environment starts empty or from a small engine base
and applies an allowlisted map.

Workers receive private staging and temporary directories. Successful outputs
are validated, hashed, and moved into the content-addressed store atomically.

## Machine and Provider Resolution

Use extensible IDs instead of a closed architecture enum. Built-in providers
register x64 and supported ARM/RISC-V definitions. A plugin can register a
namespaced 32-bit x86 architecture without modifying resolver code.

Resolution produces:

```text
canonical machine requirements
selected package representations
selected virtual providers
toolchain provider and policy
ABI key and diagnostics
lock graph
```

A requirement pass traverses the selected pipeline call graph before virtual
provider selection. Resolution then closes and locks provider dependencies.
Job factories subsequently see resolved headers, tools, runtime files, and
semantic capabilities.

## Store and Cache

Use tagged BLAKE3 digests and canonical serialization. Separate:

- Source and tree digests.
- Action cache keys.
- Artifact content digests.
- Package variant and ABI identities.

The store writes through temporary files, verifies content, and commits by
atomic rename. Per-digest locks deduplicate concurrent imports and downloads.

The action cache maps a validated action key to an output manifest. Cache hits
still verify referenced content and discovered-input manifests.

For initial C compilation, hashing complete declared include trees is correct
but conservative. Precise depfile caching requires either a deterministic
dependency scan before cache lookup or a discovery manifest that also proves
ordered search-root directory state and negative lookups. Rehashing only files
from an old depfile is insufficient because a newly created earlier header can
shadow a previously selected header.

## Package Repository

Package metadata parsing rejects duplicate keys, unsafe paths, unknown required
schemas, and digest mismatches. Publication is a staged transaction.

Static and dynamic representations are resolved within one logical component.
The resolver computes semantic link order and runtime closure independently.

Planning creates a `PackagePlanSpec` that refers to artifact promises and
resolved dependency edges. `PublishJob` waits for those promises, validates
sealed content, computes final digests and the executable activation lock, and
produces a `SealedPackagePlan`. Repository code accepts only the sealed form and
never infers component structure from basenames.

System providers are probe results with identities, not direct untracked path
strings.

## Mount Engine

The mount engine:

1. Resolves an exact executable variant.
2. Computes its metadata-declared runtime closure.
3. Detects loader-name and command collisions.
4. Asks the selected loader provider for a launch strategy.
5. Stages launchers and a private runtime view.
6. Writes an ownership and pin manifest.
7. Atomically activates a profile generation.

Binary patching is never implicit. A wrapper or native launcher operates on
opaque executable bytes, including obfuscated outputs.

For system mounts, import the closure into a root-owned store before activating
privileged launchers.

## Diagnostics

All layers emit structured events:

```rust
struct Diagnostic {
    code: String,
    severity: Severity,
    message: String,
    source_span: Option<SourceSpan>,
    package: Option<PackageId>,
    pipeline: Option<PipelineInvocationId>,
    action: Option<PlannedActionId>,
    job: Option<JobId>,
    notes: Vec<String>,
}
```

Renderers support interactive terminal output, plain output, and JSON events.
Compiler providers may translate native JSON diagnostics into this structure and
retain raw output as fallback.

## Testing Architecture

Provide deterministic fakes for:

- Filesystem planning views.
- Job executor.
- Toolchain provider.
- Package repository.
- System provider probes.
- Mount filesystem and launcher provider.

Tests must redirect every store, repository, config, and profile path into a
temporary directory. They must never access the developer's real Viator state.

Use golden tests for package metadata, lockfiles, canonical hashes, CLI events,
and Lua conversion diagnostics. Real LLVM integration tests should be separate
from fast unit and fake-executor tests.
