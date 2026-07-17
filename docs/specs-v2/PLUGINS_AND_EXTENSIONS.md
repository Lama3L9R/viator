# Plugins and Extensions

## Extension Points

Plugins may provide:

- JobFactory implementations registered under action IDs.
- Typed artifact schemas.
- Language support.
- Toolchain providers.
- Runtime and virtual capability providers.
- Architecture definitions and CPU templates.
- Package repository protocols.
- Archive and artifact transformations.
- Loader and mount strategies.
- Project templates.

The Rust core owns validation, scheduling, storage, resolution, and security
boundaries. Plugins describe policy and produce declarations through versioned
interfaces.

## Identity and Versioning

Plugins have an immutable package identity, content digest, API version, and
implementation identity. These values participate in lockfiles and cache keys.

Names exposed by a plugin are stable and namespaced:

```text
CLang:CompileAll
Native:Object
org.example.vmp:Protect
org.example.x86:x86
```

Core short names are reserved. Unknown required action, artifact, architecture,
or capability IDs are errors.

Plugin APIs are versioned independently from package metadata, the project DSL,
lockfiles, and repository protocols.

## Lua Plugins

Lua plugins run during configuration and planning. They may:

- Register action factories.
- Register artifact schemas.
- Define semantic helper APIs.
- Read tracked project inputs through Viator APIs.
- Return structured job specifications.

Artifact schemas use a declarative, versioned format that Rust can validate and
encode canonically. The schema defines required and optional fields, tagged
values, compatibility rules, and evolution behavior. Lua callbacks do not
participate in payload validation after planning.

They may not place Lua closures into worker jobs. They do not receive ambient
process, filesystem, environment, or network access. Direct capabilities are
provided by the Rust host and tracked when used.

## Native and External Plugins

In-process native plugins are not required for the initial implementation.
Their ABI and trust risks are greater than Lua planning modules.

Future executable plugins should prefer a versioned subprocess or sandboxed
protocol that exchanges canonical declarations. Any in-process native plugin is
fully trusted and must match the Viator plugin ABI exactly.

## Architecture Plugins

Core supports x64 but not 32-bit x86. An external x86 plugin may register:

- A namespaced architecture ID.
- Canonical feature and implication rules.
- CPU templates.
- ABI requirement validation.
- Toolchain translations.
- Object-format compatibility.
- Conformance tests.

No fallback path in core interprets an unknown architecture as x86 or as a
generic machine.

## Trust

Package metadata is data and must never execute code during resolution.

Build plugins and project files are executable configuration code. Loading an
untrusted plugin requires an explicit trust decision and a restricted VM.
Remote packages do not receive process or network access merely because they
contain a build description.

Plugin manifests declare requested permissions. The engine records permissions
that affect reproducibility or cache safety.
