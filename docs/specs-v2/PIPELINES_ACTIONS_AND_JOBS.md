# Greatly Rejected Document Warning
This document is greatly rejected due to overly elaboration to original prompt.

# Pipelines, Actions, and Jobs

## Three Graphs

Viator uses three related but distinct graphs:

- The package graph resolves components, representations, and virtual providers.
- The pipeline call graph composes project behavior.
- The job graph schedules concrete work.

These graphs have different identities and cycle rules. Package and pipeline
cycles are validated during planning. The final job graph must be acyclic.

## Pipelines and Targets

Targets and pipelines use the same internal `PipelineDefinition` representation.
A target is a pipeline exported under a project name and therefore addressable
from the CLI.

Conceptually:

```text
PipelineDefinition
  ordered pipeline entries
  parameters
  optional machine restrictions
  optional declared effects

Target
  project-visible name -> PipelineDefinition
```

There is no required target `depend` field. Pipeline composition uses
`CallPipeline`. A target dependency, if convenient syntax is added later, is
only syntax sugar for a pipeline call.

This allows anonymous reusable pipelines and CLI targets to share parsing,
validation, planning, caching, and cycle detection.

## CallPipeline

`CallPipeline` invokes another pipeline during planning. It is not a worker job
and does not start a nested scheduler.

The default mode shares the caller's typed append-only context. The callee sees
the current snapshot and appends artifact promises with its own invocation
provenance. Subsequent caller actions can consume those artifacts by type
without naming every output.

An isolated call creates a child context. It may export selected typed artifacts
back to the caller. Isolation is useful for helper builds whose intermediate
objects must not become visible to later actions.

Export selectors are evaluated after successful child planning and may select
only artifacts emitted by that child invocation. The resulting promises become
visible to subsequent caller planning while retaining child provenance.
Execution later seals those promises; export does not require runtime graph
expansion.

Every call receives a stable invocation ID derived from:

- The pipeline definition identity.
- A stable call-site identity within the caller definition.
- Normalized parameters.
- The caller invocation identity.
- Selected machine and toolchain policy.
- The explicit call-boundary input selection evaluated against the caller
  snapshot before child planning.

Recursive pipeline calls are rejected before job execution.

Repeated calls at different lexical call sites are distinct. Identical-call
memoization is not implicit; a future explicit memoization policy must include
the complete effective context selection in its identity.

Selections made inside child actions affect planned action and job identities,
not the already assigned pipeline invocation ID. A call that needs only a subset
of caller artifacts declares that boundary selector explicitly; otherwise the
canonical caller snapshot digest forms the boundary input identity.

## Requirement Collection

Selected pipelines are traversed once before provider resolution to collect
package and virtual capability requirements from project declarations, action
schemas, language semantics, and pipeline calls.

Toolchain and provider dependency metadata may expand this requirement graph.
Resolution continues deterministically until the provider graph is closed.
`create_jobs` runs only after that closure is locked and MUST NOT introduce a
new unresolved capability.

## Actions Are Job Factories

An action is a configured implementation of the `JobFactory` protocol. It does
not perform build work directly.

Conceptually:

```text
JobFactory.create_jobs(action, context_snapshot) -> JobFactoryResult

JobFactoryResult
  jobs: List<Job>
  emitted artifact promises
  diagnostics
```

The concrete Rust API may wrap the list in a `JobPlan` so output promises,
diagnostics, and planning metadata remain explicit.

A job factory must be deterministic for the same normalized configuration,
context snapshot, toolchain policy, and tracked filesystem inputs. Planning
must not spawn untracked processes, mutate source files, or perform hidden
network access.

Native Rust factories and Lua factories implement the same logical protocol.
Lua factories run on the coordinator thread and return serializable job
descriptions. Lua callbacks never execute concurrently inside worker threads.

## CompileAll Example

`CLang` means the C/C++ Language Support Plugin, not LLVM Clang.

For three source files, `CLang:CompileAll` can produce:

```text
Job cc:main.c   -> native/object(main.o)
Job cc:engine.c -> native/object(engine.o)
Job cc:addon.cc -> native/object(addon.o)
```

The jobs have no dependency edges between them and may execute concurrently.
A later link action selects the three object promises and creates one link job
with edges to all three producers.

```text
cc:main.c ----+
cc:engine.c --+--> link:example
cc:addon.cc --+
```

The user controls maximum parallelism through CLI or configuration. Setting the
job count to one executes the same graph sequentially without changing build
semantics or cache identity.

## Pipeline Order

Pipeline entries are planned in lexical order so each action sees artifact
promises emitted by earlier entries. Lexical order does not impose an execution
barrier by itself.

Execution edges come from:

- Consumed artifact promises.
- Explicit control dependencies.

An action that needs all earlier jobs to finish must declare a control or data
dependency. Relying on pipeline position alone is invalid.

This rule permits compile jobs from several actions to overlap when they do not
consume each other's outputs.

## Job Contract

Every job has:

```text
stable job ID
job implementation kind
declared input artifacts and files
declared output artifacts and paths
structured configuration or command arguments
environment allowlist
working directory policy
resource requirements
cache policy
sandbox and network policy
timeout and cancellation behavior
```

Process jobs use executable and argument arrays, never a shell command string by
default. Shell execution is a separate explicit job type and is non-portable
unless its shell provider is locked.

Each logical output has exactly one producer. Jobs write into private staging
directories. Outputs become visible as sealed artifacts only after successful
validation and atomic commit.

Failed or cancelled jobs publish no completed artifacts. Downstream jobs are
cancelled or skipped.

## Resource Scheduling

The scheduler supports at least:

- CPU weight.
- Memory estimate.
- I/O weight.
- Network permission.
- Named exclusive resources.
- External devices or hardware runners.

Independent jobs should run concurrently within configured limits. Cacheable
jobs MUST NOT communicate through shared mutable working directories. State
transitions are modeled as artifact inputs and outputs. Exclusive resources are
reserved for effectful devices and other noncacheable external facilities; they
do not replace semantic dependency edges.

Logs are tagged with stable job IDs. Human rendering may interleave progress,
but stored logs and machine-readable events preserve job identity and order.

## Dynamic Discovery

Ordinary source discovery happens through tracked planning APIs. Glob results
are sorted, normalized, and included in planning identity.

When a job must discover later inputs, it produces a typed discovery manifest.
A subsequent planned continuation consumes that manifest. Unrestricted runtime
mutation of the job graph is not allowed because it makes caching and remote
execution unsafe.

Initial implementations may reject runtime graph expansion and support only
planning-time source discovery.

## Effects

Jobs that publish packages, mount profiles, request secrets, interact with a
human, access uncontrolled networks, or program physical hardware are effectful.
They are not cacheable unless a more specific provider defines safe semantics.

Effectful jobs remain in the same job model so cancellation, diagnostics, and
resource ownership stay consistent.
