# Package Model

## Component Identity

A Viator component is identified by:

```text
group:name:version
```

`version` MUST be an exact Semantic Versioning 2.0.0 version in published
metadata. Dependency constraints use one versioned Viator constraint grammar.
Resolution and lockfiles always select exact versions.

One component represents one logical native product. A library may have static
and dynamic representations, but those representations are not separate
components. A workspace that produces unrelated libraries should publish them
as separate components.

This model is inspired by Gradle module and variant resolution. Viator uses JSON
metadata rather than Maven POM XML and does not inherit Maven's classifier or
scope behavior where it does not fit native builds.

## Variants and Representations

A variant describes one compatible build of a component. Its identity includes:

- Architecture.
- Canonical required machine features.
- Object or executable format.
- Package features.
- ABI-relevant runtime and virtual-provider requirements.
- Other schema-defined compatibility attributes.

A representation is a consumable form of that variant. A library variant may
provide:

- Public headers shared by all representations.
- A non-PIC static archive.
- A PIC static archive.
- A dynamic link interface.
- Dynamic runtime files.
- Optional source or debug artifacts.

Static and dynamic representations can have different dependency edges. Static
link dependencies generally propagate to the final linker. Dynamic runtime
dependencies participate in the mounted runtime closure.

A consumer requesting an unavailable representation receives an error. Viator
must not silently build from source or switch linkage unless the request permits
that behavior explicitly.

## Artifact Kinds

The component declares one logical artifact kind:

```text
library
executable
firmware
header-only
```

An executable component may expose one or more command names that all belong to
the same logical product. A firmware component may expose related encodings,
such as linked ELF, raw binary, Intel HEX, debug data, and a map file.

An executable representation declares:

- A command-to-file mapping.
- Executable mode and target-machine requirements.
- Runtime dependency roots.
- Loader capability requirements.
- Auxiliary runtime files.
- The exact representation identity used by an activation lock.

Command names MUST be one normalized path component. Separators, absolute
paths, parent traversal, platform-reserved names, and collisions after case and
Unicode normalization are rejected before publication or mounting.

Libraries and executables should not be combined into one component merely
because they are built by the same source repository. They may remain in one
workspace while being published as separate component coordinates.

## Metadata Shape

The exact schema will be versioned. The following example is illustrative:

```json
{
  "schema": 2,
  "group": "icu.lama",
  "name": "libutils",
  "version": "1.2.0",
  "variant": {
    "arch": "armv7",
    "requiredMachineFeatures": ["neon", "arm-float-abi-hard"],
    "format": "elf",
    "packageFeatures": ["unicode"],
    "abiKey": "blake3:..."
  },
  "artifact": {
    "kind": "library",
    "headers": {
      "path": "headers",
      "digest": "blake3:..."
    },
    "representations": {
      "static": {
        "files": [
          { "path": "lib/libutils.a", "digest": "blake3:..." }
        ],
        "dependencies": []
      },
      "dynamic": {
        "linkFiles": [
          { "path": "lib/libutils.so", "digest": "blake3:..." }
        ],
        "runtimeFiles": [
          { "path": "lib/libutils.so.1", "digest": "blake3:..." }
        ],
        "dependencies": []
      }
    }
  },
  "requirements": [],
  "extensions": {}
}
```

The dynamic representation uses lists because a platform may require an import
library, linker stub, versioned runtime file, aliases, or debug companion files.

## Dependencies

A dependency edge contains at least:

```text
component or virtual capability
version constraint
role
visibility
requested package features
linkage preference
machine condition
provider constraints
```

Recommended roles are:

- `api`: exposed to consumers of this component.
- `link`: required to produce a final link unit.
- `runtime`: required when loading or executing a representation.
- `build`: executable used on the machine running Viator.
- `test`: used only by test pipelines.

Visibility is `public` or `private`. Public `api` requirements propagate compile
interfaces and compatibility constraints to consumers. Private `link`
requirements are used to create this component; they propagate through a
static representation when the final consumer must still link them. Dynamic
representation dependencies propagate through the runtime closure according to
their runtime role. `build` and `test` dependencies never become consumer
requirements unless another explicit published edge references them.

Linkage selection and virtual-provider selection are different operations.
`static` and `dynamic` are representation preferences, not dependency kinds.
Virtual dependencies are capability requirements, not symbolic linkages.

Within one final link unit, one component MUST resolve to one representation.
Conflicting hard static and dynamic requests are errors. Separate final link
units may select different representations.

Because C has no general symbol namespace, selecting multiple incompatible
versions of one component in a process closure is an error unless metadata
explicitly declares that the representations are isolated or co-installable.

## Virtual Dependencies

Virtual dependencies express required facilities without forcing one provider.
Their group name must begin with `@`.
Core capability families include:

```text
@cc:compiler-runtime
@cc:libc
@cc:crt
@cc:dynamic-loader
@cxx:abi
@cxx:stl
@cxx:unwind-runtime
@platform:linux-kernel-api
@embedded:bsp
```

A request may omit a provider. Toolchain policy supplies a default compatible
provider and records the exact choice in the lockfile. An explicit provider is
a hard request and must not be silently replaced.

No standard library is implicit. A `nostd` project simply does not request
`@cc:libc` or `@cxx:stl`. The compiler runtime remains separate because a
freestanding build may still need generated arithmetic or unwind helpers.

Language actions derive capability requirements before provider resolution.
For example, a link action may require a compiler runtime without requiring a C
library. Toolchain policy chooses providers only after this requirement pass and
MUST NOT introduce a standard-library capability that no selected pipeline or
dependency requested.

Provider metadata defines compatibility and cardinality. Facilities such as a
C library are normally exclusive within one runtime namespace. Additive
facilities, such as some instrumentation runtimes, may permit multiple
providers.

## System Providers and Search Paths

A library found in a system location is still represented by a provider node.
The resolver records enough information to identify and validate the selected
files. "Whatever is installed" is not a stable dependency identity.

An arbitrary include or linker search path is an escape hatch. Viator must do
one of the following:

- Import and hash the selected files into its store.
- Resolve the path through a declared system provider.
- Mark affected jobs and artifacts as non-hermetic.

Non-hermetic artifacts should not be uploaded to a shared cache or published as
reproducible packages without an explicit override.

## Lockfile

The lockfile records:

- Exact component versions and repository origins.
- Metadata and artifact digests.
- Selected variants and representations.
- Package features.
- Virtual capability providers.
- Toolchain and runtime provider identities.
- Target machine requirements.
- Plugin versions and implementation identities.

Resolution must be deterministic and independent of repository response order
or JSON object order. Ambiguous equally compatible variants are errors unless a
versioned policy defines a deterministic preference.

## Variant Matching

Variant matching uses a versioned algorithm:

1. Normalize architecture and feature aliases into canonical IDs.
2. Expand feature implication rules from the selected architecture provider.
3. Reject a variant whose architecture or object format is incompatible.
4. Require every variant machine requirement to be satisfied by the target.
5. Require requested package features and representation availability.
6. Validate virtual providers and structured ABI requirements.
7. Rank only by explicit, versioned policy preferences.
8. Report an error if more than one equally preferred compatible variant
   remains.

The ABI key protects identity but does not replace structured compatibility
checks. Diagnostics report incompatible requirements and their dependency
paths.

## Published Resolution

Published executable command roots include, or refer by digest to, an immutable
resolved runtime graph. This activation lock records exact component versions,
variants, representations, virtual providers, and artifact digests used to
construct the runtime closure.

Published dynamic libraries record exact build provenance plus structured
runtime and loader constraints. They do not independently pin the final process
loader graph. Resolving an executable combines those constraints and creates the
activation lock for that command root.

Mounting MUST use the activation lock by default. Re-resolving version
constraints is an explicit relock operation that produces a new activation
identity.

## Metadata Safety

Metadata paths are normalized relative paths. Absolute paths, parent traversal,
escaping symlinks, duplicate JSON keys, and platform-specific path aliases must
be rejected.

Digests include their algorithm. Identity is computed from canonical metadata
and verified content; a publisher-provided variant hash is never trusted by
itself.

Extension data belongs under namespaced, schema-versioned objects:

```json
{
  "extensions": {
    "org.example.plugin": {
      "schema": 1,
      "data": {}
    }
  }
}
```
