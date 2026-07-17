# Lua DSL and Host API

## Project Shape

V2 uses one pipeline constructor. A target is created by exporting a pipeline
under a name in the project root.

```luau
local build = V.pipeline {
    dependencies = {
        V.dep("icu.lama:libutils", {
            version = "^1.2.0",
            linkage = "static",
        }),
    },
    V.action("CLang:CompileAll", {
        sourceRoot = "src",
    }),
    V.action("CLang:LinkObjects", {
        name = "example",
        static = true,
        dynamic = true,
    }),
}

return {
    group = "icu.lama",
    name = "example",
    version = "1.0.0",

    targets = {
        build = build,
    },
}
```

Dependency requests must be attached to a project, pipeline, or schema-defined
action field. An unattached value returned by `V.dep` or `V.virtual` has no
effect and should produce a configuration warning or error.

Project dependencies apply to every selected target. Pipeline dependencies
apply only when that pipeline is reachable from the selected target. Action
requirement collectors may derive additional capabilities from semantics.

`V.target` is not required. If retained as a convenience, it is exactly an
alias for `V.pipeline` and has no separate runtime model.

The root target map supplies public names. Anonymous pipelines may be reused
without becoming CLI targets.

## Pipeline Composition

`V.callPipeline` creates a pipeline entry rather than a worker job:

```luau
local publish = V.pipeline {
    V.callPipeline(build),
    V.action("V:CollectFiles", {
        pattern = "include/**/*.h",
        artifactType = "CLang:HeaderTree",
    }),
    V.action("V:AssemblePackage", {
        kind = "library",
    }),
    V.action("V:PublishLocal", {
        package = V.select("V:PackagePlanSpec"),
    }),
}
```

The default call shares the typed append-only context. The called pipeline's
artifact promises become visible to later caller actions.

Isolation is explicit:

```luau
V.callPipeline(toolBuild, {
    arguments = {
        mode = "generator",
    },
    isolated = true,
    export = {
        V.select("Native:Executable", { label = "generator" }),
    },
})
```

Rust validates call cycles and normalizes invocation arguments.

Pipelines declare parameters when needed:

```luau
local toolBuild = V.pipeline {
    parameters = {
        mode = { type = "string", required = true },
    },
    -- Entries follow as numeric fields.
}
```

`V.param("mode")` creates a tagged parameter reference usable in action
configuration, dependency requests, selectors, and nested call arguments. Rust
binds and validates all parameter references before requirement collection and
uses the canonical bound map in pipeline and action identity.

Isolated export selectors are evaluated after successful child planning against
artifacts emitted by that child. Exported promises retain child provenance and
become available to subsequent caller planning.

## Action Invocation

`V.action` creates owned declarative configuration:

```luau
V.action("CLang:CompileAll", {
    sourceRoot = "src",
    standard = "c23",
    defines = {
        EXAMPLE = "1",
    },
})
```

The configuration may contain JSON-like values, registered handles, selectors,
and schema-defined tagged values. It may not contain arbitrary closures that
must survive configuration.

Unknown action IDs and invalid fields should fail at construction when the
schema is available. Rust repeats validation at the trust boundary.

## Dependencies

Concrete components use `V.dep`:

```luau
local utils = V.dep("icu.lama:libutils", {
    version = "^1.2.0",
    role = "link",
    visibility = "private",
    linkage = "static",
    features = { "unicode" },
    when = V.machineCondition { arch = "armv7" },
    provider = {
        repository = "official",
    },
})
```

Virtual facilities use `V.virtual`:

```luau
local libc = V.virtual("c.libc")

local explicitLibc = V.virtual("c.libc", {
    provider = "llvm:libc",
    version = "20.1.0",
    role = "link",
    visibility = "private",
})
```

Dependency requests support `version`, `role`, `visibility`, package features,
linkage preference, machine condition, and provider constraints. Virtual
requests also support capability cardinality constraints where the capability
schema permits them.

Project and pipeline requests default to `role = "link"`,
`visibility = "private"`, `linkage = "auto"`, and the current target-machine
condition. Public API dependencies and build tools must declare their different
role or visibility explicitly. Action schemas may define narrower defaults for
dependency-valued fields.

These constructors create requests. They do not perform resolution. Rust
resolves and locks requests before job factories consume dependency artifacts.

Omitting a standard-library request is the `nostd` behavior. The DSL does not
need `os = "none"` or an implicit `noStd = true` switch.

## Module Loading

`V.require` resolves a versioned plugin or bundled module through Rust:

```luau
local CLang = V.require("viator/CLang@1")
local std = V.require("viator/CLangStdLibs@1")
```

Module resolution is deterministic. The lockfile records external module
identity and digest. Modules do not search ambient Lua package paths.

Repeated requests for one identity return the same module instance inside a
configuration session.

## Action Registration

A Lua plugin registers an action schema and a planning factory:

```luau
V.registry.action {
    id = "Example:Generate",
    schema = {
        input = "string",
        outputType = "string",
    },
    requirements = function(request)
        return {}
    end,
    factory = function(request)
        local output = request:promise(request.config.outputType, {
            label = "generated",
        })

        return {
            jobs = {
                V.job.process {
                    tool = request.tools.generator,
                    args = {
                        "--input", request.config.input,
                        "--output", V.job.outputPath(output),
                    },
                    outputs = { output },
                },
            },
            artifacts = { output },
        }
    end,
}
```

This syntax is illustrative. The stable API should minimize direct path
handling and should let output handles generate their private staging paths.

The factory executes serially during planning. `V.job.process` returns a plain
job description and never spawns the process.

`requirements` executes before provider resolution and may return package,
toolchain, or virtual capability requests. `factory` executes after resolution
and may not add an unresolved requirement.

## Factory Wire Values

Factory results use a versioned tagged wire format. Promise handles become
factory-local tokens, not process paths. Process arguments support:

```text
literal string or byte value
input artifact path placeholder
output artifact path placeholder
temporary path placeholder
```

Rust assigns stable artifact IDs, validates token ownership, and materializes
private paths only in the executor. Returned persistent declarations must not
contain Lua functions, userdata without a registered encoder, or ambient paths.

## Artifact Type Registration

Plugins register serializable schemas:

```luau
V.registry.artifactType {
    id = "Example:GeneratedData",
    version = 1,
    schema = {
        path = "file",
        format = "string",
    },
}
```

Built-in schemas may be implemented in Rust and exposed as generated Luau type
definitions. Plugin schemas remain namespaced and versioned.

## Provider Registration

Lua modules may register declarative providers through versioned host APIs:

```text
V.registry.toolchainProvider
V.registry.capabilityProvider
V.registry.architecture
V.registry.loaderProvider
V.registry.template
```

Rust validates provider schemas and owns resolution. Provider planning
callbacks remain session-local Lua factory IDs and are released before worker
execution.

Provider registrations contain owned compatibility data and phase-scoped
callbacks. At minimum:

```text
toolchain provider
  id, version, execution requirements, target matcher, capability dependencies,
  semantic translator factory

capability provider
  id, provided capability, cardinality, attributes, dependencies, artifacts

architecture provider
  id, aliases, CPU templates, feature implications, ABI validator

loader provider
  id, runtime constraints, isolation grades, launcher planner
```

Launcher planners return owned `LoaderPlan` data while the Lua session is alive.
Mount workers execute that plan without invoking Lua.

## Context and Selectors

Factories receive a read-only context snapshot:

```luau
local objects = request.context:select("Native:Object", {
    compatibleWith = request.machine,
})
```

Selectors are deterministic against the lexical planning snapshot. They may
filter by type, scope, label, machine requirements, and schema-defined
attributes.

Action schemas should supply common defaults. A user should not need to write a
selector for the ordinary compile-then-link pipeline.

Labels are optional and intended for ambiguity or public outputs, not every
intermediate artifact.

## Job Declarations

Core job constructors include:

```text
V.job.process
V.job.copy
V.job.archive
V.job.fetch
```

Publication and mounting are privileged engine effects and should normally be
invoked through core actions instead of arbitrary plugin job declarations.

Every job constructor requires declared inputs, outputs, environment, and
resource policy. Defaults are restrictive.

Jobs may refer to tool and artifact handles, but not ambient executable names
whose resolution depends on `PATH`.

## Tracked Planning APIs

Source discovery uses host APIs:

```luau
local sources = V.fs.glob("src/**/*.c")
```

The host records the root, pattern, sorted result, and relevant file identity.
Direct `io`, `os`, process, and network libraries are unavailable in the
restricted environment.

Tracked filesystem methods accept normalized project-relative `ProjectPath`
values. Absolute paths and `..` are rejected. Symlinks may not escape the
declared project or dependency root. Case-folding and normalization collisions
are diagnosed according to the selected filesystem policy.

## Diagnostics

Lua code reports structured diagnostics through the request or `V` host API:

```luau
request:error("CLANG001", "no source files matched", {
    field = "sourceRoot",
})
```

Throwing a Lua error is reserved for plugin defects or syntax errors. Expected
configuration failures should carry stable codes and source context.

## Determinism Rules

Lua planning code must not depend on:

- Hash-table iteration order.
- Wall-clock time.
- Random values without declared seeds.
- Ambient environment variables.
- Untracked filesystem state.
- Repository response order.
- Job completion timing.

Host APIs return sorted collections where ordering is otherwise undefined.
