# CLI and Project Generation

## Command Shape

Commands put project target selection, machine selection, package features, and
parallelism in separate arguments.

Illustrative commands:

```text
viator build
viator build publish
viator build :subproject:build
viator build --arch armv7 --cpu cortex-a8
viator build --cpu cortex-a8 --machine-feature neon
viator build --cpu cortex-a8 --disable-machine-feature neon
viator build --arch armv7 --feature unicode
viator build --jobs 12
viator build --jobs 1
```

The positional target selects a named pipeline. `build` is the conventional
default target. Publication is never implicit; it requires an explicit target
or command with publication effects.

Package features and machine features use different flags and namespaces. A
future short syntax must not combine them into one ambiguous `+feature` list.

The CLI does not require an OS triplet. Verbose diagnostics may show the
compiler-specific target selected by a toolchain provider.

## Machine Selection

Users may select:

- Architecture.
- CPU template.
- Additional required or disabled machine features.
- Toolchain policy.
- Explicit runtime providers.

Host detection is explicit. A `native` machine policy records the detected
feature set and does not pretend to produce a portable baseline package.

When `--arch` is omitted, a CPU template supplies its architecture. A conflict
between an explicit architecture and the selected CPU is an error. Machine
feature flags are validated after CPU feature expansion. Package `--feature`
values never enter the machine-feature namespace.

## Parallelism

`--jobs N` limits concurrent scheduler work. A value of one disables parallel
execution without changing the planned graph.

Configuration may define separate limits for CPU-heavy, network, memory-heavy,
and exclusive-resource jobs. CLI flags override configuration for one run.

## Project Generation

`viator new` generates a project and build description from versioned templates:

```text
viator new example
viator new example --cpu cortex-a8
viator new firmware --cpu cortex-m0 --template nostd
viator new firmware --mcu stm32f030
viator new tool --template hosted-c
```

The default project template is `nostd`. It does not request a C library, C++
standard library, CRT, loader, or operating-system facility.

A hosted template adds explicit virtual runtime requirements. An MCU or board
template may add a BSP provider, startup files, memory layout, and linker script.

Templates generate ordinary project files. They do not create hidden global
configuration required for future builds.

## Package Commands

Expected package operations include:

```text
viator resolve
viator build --locked
viator build --frozen
viator publish local
viator mount <component-selector>
viator unmount <profile-or-command>
```

`--locked` rejects dependency graph changes. `--frozen` also rejects network
access and missing locked content.

Local publication adds content to the isolated Viator repository. Mounting is a
separate transactional operation.

## Output

The engine emits structured events. Renderers provide:

- Interactive terminal progress.
- Plain noninteractive output without color.
- Verbose commands and provider decisions.
- Machine-readable JSON events.

Diagnostics identify the package, pipeline invocation, action, and job. Variant
errors explain incompatible attributes and provider requirements instead of
reporting only that no variant was found.

## Configuration Locations

Configuration, mutable cache data, immutable package data, and mounted profiles
should use separate platform-appropriate directories. A configurable repository
root may present a unified view, but Viator should not assume that every kind of
state belongs directly under `~/.viator`.

Project-local configuration takes precedence only through documented fields.
Ambient compiler and linker environment variables are not imported by default.
