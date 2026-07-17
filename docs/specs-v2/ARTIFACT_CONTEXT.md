# Typed Artifact Context

## Purpose

Pipelines need to connect actions without forcing users to invent names for
every object file, generated header, archive, executable, and other intermediate
result. An unrestricted mutable context solves the naming problem but makes
concurrency, caching, validation, and plugin interoperability unreliable.

Viator uses a typed, append-only artifact context. It behaves as a persistent
multimap from stable artifact type IDs to immutable artifact records.

```text
ArtifactContext = Multimap<ArtifactTypeId, ArtifactRecord>
```

## Artifact Type IDs

Artifact types use stable namespaced IDs and schema versions. They are not Rust
`TypeId` values and do not depend on process-local addresses or Lua table
identity.

Illustrative built-in and plugin types include:

```text
V:File
V:FileTree
V:CollectedFiles
CLang:CSource
CLang:CxxSource
CLang:HeaderTree
Native:Object
Native:StaticLibrary
Native:DynamicLibrary
Native:Executable
Firmware:LinkedImage
Firmware:EncodedImage
```

Object format, machine requirements, PIC state, source provenance, and similar
properties are artifact attributes. They should not create unrelated artifact
types such as `ELFObject` and `PEObject` unless their payload schemas are truly
different.

Plugins may register additional namespaced artifact schemas. Consumers must
reject an unknown required schema instead of treating its payload as an
arbitrary Lua value.

## Artifact Record

An artifact record contains:

```text
artifact reference ID
artifact type ID and schema version
immutable typed payload
producer action and job provenance
pipeline invocation scope
optional user label
machine and variant attributes
semantic ordering information
content digest when sealed
```

Payloads are owned, serializable data validated by Rust. Scheduler workers do
not retain `mlua::Value`, Lua functions, or references into a Lua VM.

## Promises and Sealed Artifacts

Planning creates artifact promises. A promise identifies the future output of
one job and is immediately available to later job factories for dependency
construction.

Execution turns a promise into a sealed artifact only after:

- The producer succeeds.
- Declared outputs exist.
- Output paths and types validate.
- Content digests are computed.
- Files are atomically imported into the build store.

Failed or cancelled jobs never produce sealed artifacts. Consumers of their
promises are cancelled or skipped.

The planning context contains promises and immutable metadata. The completed
build result contains only sealed artifacts and explicit effect records.

## Append-Only Semantics

Actions may append records. They may not mutate, replace, or remove records
already visible in a context snapshot.

A transformation creates another artifact:

```text
Native:Executable
    -> DoVMProtectAction
    -> Native:Executable { protected = true }
```

Both records retain separate provenance and content identity. Publication can
select the transformed artifact without altering the original result.

Repeating an artifact reference with identical identity may deduplicate.
Producing different content for the same declared reference is an error.

## Selection

Job factories consume artifacts through typed selectors. A selector may filter
by:

- Artifact type.
- Machine compatibility.
- Representation or format.
- Producer invocation scope.
- User label.
- Source path or other schema-defined attributes.

Examples:

```luau
V.select("Native:Object")

V.select("Native:Object", {
    label = "plugin",
    pic = true,
})
```

The common case needs no selector in the build file. An action schema can define
the default selection, such as all compatible `Native:Object` records visible
at that lexical pipeline position.

Selection operates on a fixed planning snapshot. "All objects" never changes
according to which concurrent job finishes first.

## Ordering

Context insertion order is not linker order. Semantic order is represented by
dependency edges and explicit ordered collections in artifact payloads.

Static library ordering is derived from the component dependency graph.
Strongly connected static dependency groups may be represented explicitly for
toolchains that support linker groups.

Concurrent artifact merges use stable producer invocation IDs and
producer-local emission order. This makes diagnostics and serialization stable
without turning completion timing into semantics.

## Scopes

Every pipeline invocation has a scope.

In a shared `CallPipeline`, the callee reads the caller snapshot and appends
records into the shared lineage. The records still carry the callee invocation
ID, so a later selector may distinguish them.

In an isolated `CallPipeline`, the callee appends into a child context. Only
artifacts emitted by that child and selected for export become visible to the
caller. Export promises appear atomically after successful child planning and
retain child provenance. Job execution later seals them.

Package dependencies seed the context with typed headers, link inputs, runtime
files, tools, and provider information before project actions are planned.

## Naming Policy

Intermediate names are optional. Labels are required only when type and scope
are insufficient, such as:

- Two independent products use the same artifact type.
- A link action should consume one subset of compatible objects.
- A target exports several artifacts of one type.
- Publication or mounting needs a stable public identifier.

File names are not artifact identities. A staged path may change without
changing the logical role of an artifact, and two scopes may safely contain the
same basename.

## Lua View

Lua receives read-only artifact handles and selector APIs. It does not receive a
mutable table containing arbitrary file paths.

Artifact handles may expose schema-defined fields for planning:

```luau
for object in context:select("Native:Object") do
    V.info(`link input: {object.source}`)
end
```

The Rust boundary validates all values returned by a Lua job factory before
they become jobs or artifact promises.
