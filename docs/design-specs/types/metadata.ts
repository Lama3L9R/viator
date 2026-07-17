import { Arch, ArchExtensions, BinaryWrapperFormat } from "./built-in-ids"
import { Optional, WithCustomData } from "./utils"

type File = {
    path: string,
    hash: string,
}

type DependencyType = 'static' | 'dynamic' | 'symbolic' // Symbolic for virtual packages

// group:name:version[+feature] also virt pkgs like: @clang:libc
//                                                   @sys:openssl find library from system, not recommended
//                                                                should use prebuilt viator wraps
type Dependency = WithCustomData<{
    group: string,
    name: string,
    version: string,

    features: string[],
    type: DependencyType,
    linkPriority: number // Allow override link priority
}>

type Metadata = WithCustomData<{
    group: string,
    name: string,
    version: string,

    files: WithCustomData<{
        headers: Optional<File>,
        static: Optional<File>,
        staticPic: Optional<File>,
        dynamic: Optional<File>,
        sources: Optional<File & { partial: boolean }>,
    }>

    dependencies: Dependency[]

    variant: {
        arch: Arch
        extensions: ArchExtensions[]
        format: BinaryWrapperFormat,
        features: string[]
    }

    variantHash: string
}>