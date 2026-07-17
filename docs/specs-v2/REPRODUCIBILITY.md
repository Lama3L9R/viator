# Reproducibility

## Contract

For a hermetic build, the following inputs determine the result:

```text
source and generated input digests
project and plugin code
lockfile
toolchain and runtime providers
machine and package features
normalized action configuration
declared environment
sandbox policy
```

The same inputs must produce the same planned job graph. Output reproducibility
also requires every job implementation and selected provider to declare and
satisfy deterministic-output behavior.

Reproducibility is guaranteed for one locked toolchain and provider graph. It
does not require different compiler families to emit byte-identical artifacts.

## Hermeticity Levels

Viator distinguishes:

- `hermetic`: all inputs are immutable, declared, and digest-addressed.
- `tracked-system`: system providers are probed and fingerprinted.
- `uncontrolled`: jobs consume undeclared paths, environment, network, or time.

The level propagates through artifact dependencies. An artifact cannot claim a
stronger level than any input used to produce it.

Remote cache upload and reproducible publication should require `hermetic`
inputs by default. Overrides must be explicit and recorded.

## Action Identity

A cache key includes:

- Action and job implementation identity.
- Plugin and schema versions.
- Canonically encoded configuration.
- Every declared input digest.
- Tracked source-discovery manifests.
- Toolchain executable and policy digests.
- Runtime and virtual-provider selections.
- Architecture and canonical required machine features.
- ABI-relevant settings.
- Package feature and representation selections.
- Relevant environment values.
- Sandbox and executor semantics where they affect output.
- Ordered inputs when order is semantic.

Absolute workspace paths, repository response ordering, current time, ambient
locale, and undeclared environment values must not affect a hermetic identity.

## Filesystem Inputs

Planning glob results are sorted and hashed. Compiler depfiles or equivalent
dependency scans identify headers consumed by each translation unit.

Generated files are explicit artifact edges. A job may not create a header in a
shared source directory and rely on timing for another job to observe it.

Paths in cached manifests use normalized relative syntax. File mode, executable
state, symlink intent, and case-sensitivity assumptions are represented where
they affect behavior.

## Deterministic Execution

Providers claiming deterministic output MUST use or provide equivalent behavior
for:

- Stable archive member ordering.
- Deterministic archive metadata.
- Source and debug prefix mapping.
- Explicit locale and timezone.
- Defined timestamp policy such as `SOURCE_DATE_EPOCH`.
- Stable response-file content.
- Explicit linker build-ID policy.

Jobs run in private working directories and commit outputs atomically. Partially
written outputs are never cache entries.

## Environment and Network

Jobs receive an allowlisted environment. Host variables such as compiler flags,
include paths, library paths, preload settings, and SDK roots are not inherited
implicitly.

Network access is denied by default. Fetch jobs use declared repository inputs
and verify content digests. Jobs with uncontrolled network access are
uncacheable.

## Cache Safety

Cached outputs are verified by digest before use. Remote cache content is not a
source of package-resolution authority and cannot change the lockfile.

Digest verification detects corruption only when the expected action-result
record is trusted independently. Remote caches require a configured trust
policy, such as authenticated transport plus trusted service identity or signed
action-result records. An unauthenticated cache is trusted to the same degree as
remote execution and is disabled by default.

Publication, mounting, interactive prompts, hardware programming, and jobs that
consume secrets are effects and are uncacheable by default.

## Transformations and Obfuscation

An obfuscation or protection action is an ordinary declared transformation. It
consumes a sealed executable and produces another sealed executable with a new
digest and provenance.

The transformation implementation, configuration, license inputs, and declared
runtime requirements participate in job identity. Secret material should not
be written into shared cache metadata, and secret-dependent outputs should not
be uploaded automatically.

Viator does not need to understand the transformed section table to preserve
reproducibility. It relies on the action's declared outputs and runtime metadata
and validates only the properties required by the selected provider.
