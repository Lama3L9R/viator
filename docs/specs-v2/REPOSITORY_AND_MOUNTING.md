# Greatly Rejected Document Warning
This document is greatly rejected due to overly elaboration to original prompt.

# Repository and Mounting

## Isolation Rule

Building or publishing a package never makes it globally available. Local
publication only makes an immutable component available to Viator dependency
resolution.

Publication must not modify:

- `PATH`.
- Global loader configuration.
- System include or library directories.
- The current shell environment.
- Existing mounted profiles.

Host visibility requires an explicit `viator mount` operation.

## Local Repository

The local repository presents a component-oriented view:

```text
repo/<group>/<name>/<version>/<variant-id>/metadata.json
```

Artifact contents should be backed by a content-addressed store:

```text
store/blobs/<digest>
store/trees/<digest>
store/actions/<digest>
```

Repository entries are immutable indexes into verified content. Publishing the
same coordinate and variant with different content is an error.

Publication stages metadata and files in a temporary sibling directory,
verifies all digests, and commits atomically. A failed publication leaves no
visible partial package.

Concurrent publishers use a coordinate lock or compare-and-swap operation.
Crash recovery removes stale staging directories only after proving that no
live transaction owns them. Repository durability requirements are explicit for
each supported filesystem.

## Runtime Closure

Mounting starts from an exact executable representation and its published
activation lock. It computes the runtime closure without re-resolving version
constraints or provider choices. Dynamic library packages contribute runtime
constraints to that executable lock rather than carrying independent final
process locks.

The closure includes:

- Selected executable files.
- Dynamic runtime representations.
- Runtime dependencies propagated by linked components.
- Virtual runtime providers.
- Required loader aliases, SONAMEs, install names, or DLL basenames.
- Declared runtime-loaded libraries.

Metadata is authoritative. Binary scanning may validate metadata but is not the
primary dependency discovery mechanism. Computed `dlopen` or `LoadLibrary`
targets must be declared by the producer because no scanner can discover every
runtime name reliably, especially after obfuscation.

Conflicting files or loader identifiers in one runtime namespace cause mount
preflight failure unless a provider supplies a proven isolation strategy.

## Activation View

A mount creates a private, transactional activation view containing:

```text
profile generation manifest
command launchers
private runtime closure
required aliases or links
ownership and content records
```

The view is staged and activated atomically. Existing unmanaged files are never
overwritten. Unmount removes only entries that still match the recorded owner
and content identity.

Profile updates use a generation lock and compare the expected active
generation before commit. Activation writes the new manifest durably before
switching the current-generation pointer. Interrupted staging is recoverable
without modifying the previous active generation.

Mounted generations pin their package and store closure against garbage
collection. Profile generations should support rollback.

Libraries remain private to mounted commands. Mounting does not run `ldconfig`,
add global DLL directories, or publish general linker search paths by default.
Libraries used for builds are resolved from the Viator repository, not from the
mounted command profile.

## Launch Strategy

Launch behavior is supplied by the selected loader or runtime provider. Viator
does not infer it solely from an OS triplet.

Possible strategies include:

- A small shell wrapper using a private loader search path.
- Direct invocation of a known dynamic loader with a private library path.
- A generated native launcher.
- A private application directory with runtime libraries beside the command.

The strategy must preserve arguments, working directory, standard handles,
exit status, and normal signal or console behavior.

Launchers use absolute paths to the selected executable and closure. Inherited
loader search, preload, and injection variables MUST be sanitized unless an
explicit host-dependent mount policy permits them.

Mount strategies declare an isolation grade. A `strict-loader` strategy controls
initial loader resolution and every permitted initial host fallback through
resolved system providers. A `sandboxed-runtime` strategy additionally controls
later absolute-path and computed dynamic loads through a provider-defined
runtime boundary. A `host-dependent` strategy is a convenience activation and
does not claim loader confinement or reproducibility.

Metadata completeness is a trust requirement. Without a sandboxed runtime,
Viator cannot prevent an opaque program from requesting an undeclared absolute
library path after launch; it can only ensure that declared and ordinary initial
loader resolution uses the locked closure.

### POSIX wrappers

A simple POSIX wrapper may set a provider-defined search variable and finish
with `exec`:

```sh
#!/bin/sh
LD_LIBRARY_PATH="/absolute/viator/profile/lib" \
exec "/absolute/viator/store/program" "$@"
```

The real implementation must quote generated paths safely and should avoid
appending uncontrolled inherited paths by default. Direct loader invocation or
a native launcher is preferable when it provides stronger isolation.

A shell wrapper cannot sanitize variables before its own interpreter starts and
therefore is normally `host-dependent`. A `strict-loader` POSIX mount uses a
suitable native or static launcher, direct loader invocation, or another
provider-defined mechanism that controls preload and initial fallback behavior
before loading the target.

### Windows launchers

Windows support should use a native launcher or private application directory
instead of relying on a batch script. The launcher configures an exact DLL
search scope, implements Windows command-line quoting correctly, limits
inherited handles, and forwards the child exit code.

### Restricted binaries

Setuid binaries, file capabilities, code-signing policies, macOS hardened
runtime restrictions, and similar platform rules may prevent environment-based
launching. Mount preflight must reject an unsupported combination instead of
modifying the binary silently.

## No Automatic Binary Patching

Mounting does not run `patchelf` or another binary rewriter automatically.
Published bytes remain immutable, signatures remain meaningful, and unusual or
intentionally damaged section tables do not need to be understood by the mount
implementation.

A plugin may expose an explicit patching action. Such an action is a normal
build transformation that produces a new content-addressed artifact and records
its effect on signatures and runtime metadata.

## Obfuscated Products

An action such as `DoVMProtectAction` consumes an executable and produces a new
protected executable for publication.

Mounting treats the protected output as opaque. It does not inspect, rewrite,
or repair its section table. The action is responsible for declaring:

- The produced command artifact.
- Runtime dependencies retained or added by protection.
- Loader restrictions.
- Additional files required at runtime.

If the host loader can execute the protected binary using the declared private
runtime closure, the wrapper strategy remains valid regardless of symbol-table
or section-table obfuscation.

## User and System Mounts

A user mount writes only to a user-owned profile. A system mount requires an
explicit privileged operation and must not create privileged launchers that
refer to a user-writable store.

System mounts import or copy the exact closure into a root-owned immutable
store before activation. This prevents a user from replacing content after a
privileged launcher has been created.

## Command Collisions

Packages declare command exposure names. A mount collision is resolved before
activation by:

- Failing.
- Explicitly renaming a command.
- Explicitly skipping a command.

Runtime library identifiers cannot be renamed arbitrarily because the loader
may refer to them by embedded identity.
