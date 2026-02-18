
-- This is a design file that is used to
-- Show how this program will be like

-- Import plugin
local artifactory = V.require("SomeCoolGay/viator-artifactory")

local dependencies = {
    V.dynamic {
        name = "GayGayle:v114514GayEngine",
        version = { -- "Or just git reference name (hash/tag)"
            repository = "...", -- This object is received by build script directly
                                -- To support GN and depot_tool and other shit
        },
        flags = { -- Any extra flags

        }
    },
    V.dynamic("Tabi:Core:1.1.4-Release"),
    V.static("Tabi:viator:5.1.4-Beta") -- same as dynamic but linked statically
}

function handleExtraFlags(ctx, fl)
    if fl["UseExperimentalAPI"] then
        ctx.dependencies:override("Tabi:Core", {
            flags = {
                enableTestingAPI = true,
                vectorization = true,
                whateverTheThingIs = 114.514
            }
        })
    end

    if fl["UseSpecialCFlags"] then
        ctx.pipeline:override("doCompile", {
            ccFlags = {
                c = {
                    "-DoWhatEverYouWant"
                },
                cxx = {
                    "-DoSomeRandomStuff"
                },
                "-GeneralFlagsHere",
                "-YetAnotherFlag"
            }
        })
    end
end

function bin2obj(ctx, conf)
    local sources = conf:extractSources() -- kinda like a QOL helper func
    local intermediateDir = ctx.intermediateDir

    ctx.files.generatedAssemblies = V.utils.newArray()

    for k in sources do
        V.utils.touchAndChangeSuffix(k, intermediateDir .. "/generated-assemblies", ".S")
        -- Generate the assembly code
        ctx.files.generatedAssemblies:push("")

        V.action("CLang:CompileAll"):invoke(ctx, V.utils.merge(
                {
                    sourceRoot = intermediateDir .. "/generated-assemblies",
                    sources = { "..." },
                },
                conf.ccConfig
        )) -- This action will add generated obj files into ctx.files.objects
    end

end

V.registry:action("ExamplePlugin:Binary2Object", bin2obj)
--[[

Internal impl
    PipelineFiles {
        "c": [...]
        "cxx": [...]
        "object": [] // list of all objects produced
    }

--]]

local release = V.target {
    name = "release",
    dependencies,
    extraFlagsHandler = handleExtraFlags,
    pipeline = {
        V.action("V:PrepareDependencies"),
        V.action("CLang:CompileAll", {
            name = "doCompile", -- sure you can assign a name to it to distinguish between the same task name runed twice
                                -- but totally optional, as you can just use CLang:CompileObjects to represent this action
            sourceRoot = "src", -- default to src
            headerRoot = "src", -- default to sourceRoot
            sources = { "...", "...", {

            } }, -- default to all files under sourceRoot
            ccFlags = nil -- This object is universal to Clang:CompileC and Clang:CompileCXX
        }),
        V.action("ExamplePlugin:Binary2Object", {
            sources = { "help.bin" },
        }),
        V.action("CLang:LinkObjects", {
            produce = {
                {
                    name = "examplePlugin",
                    dynamic = true,
                    static = true
                }
            },
            ldFlags = {
                "llvm:-flags=xxx", -- to specify flag dedicated to that linker
                                   -- same syntax applies to ccFlags
                "llvm,gnu,ndk:-abc" -- to specify multiple linkers
            },
            linker = V.linker("llvm", "second-option here")
        }),                            -- You do not have to specify input files
                                       -- since it's a pipeline A -> B -> C
                                       -- A (source) -> B (intermediate, OBJ) -> C (elf)
        V.action("V:CopyCoreDependencies") -- Copy dynlib/so/dll into output folder
    }
}

local debug = release:from { -- default values are all from release
    name = "debug",
    debug = true,
    optimizations = false,
    sanitizer = {
        address = true,
    }
}

local libraryRelease = nil
local libraryDebug = nil -- Dummy targets

local publishRemote = artifactory.publishTarget {
    pipeline = {
        {
            parallel = true,
            V.action("V:ExecuteTarget", "release"),
            V.action("Artifactory:CollectArtifact"),
        },
        {
            parallel = true,
            V.action("V:ExecuteTarget", "debug"),
            V.action("Artifactory:CollectArtifact"),
        }, -- Yes you can do this to isolate context
           -- if you do this, ctx.parent will be set, so you are still able to
           -- interact with parent context
           -- This potentially makes parallel execution possible?
        V.action("Artifactory:Publish")
    }
}

local publishViatorLocal = V.target {
    pipeline = {
        V.action("V:ExecuteTarget", "release"),
        V.action("V:PublishLocal", {
            include = {
                products = true,
            },
        }),
    }
}

function checkCompiler()
    local cc = V.compiler("llvm")

    if cc == nil then
        error("Currently, only llvm clang is supported")
    end

    local linker = V.linker("llvm", "ndk", "gnu")
    if linker == nil then
        error("No supported linker")
    end

    if linker.type == 'ndk' then
        if not linker.version:requireMinimum("android24") then
            V.warning("Android support below sdk24 is experimental")
        end
    end
end

return {
    group = "icu.lama",
    name = "example",
    envCheck = {
        checkCompiler
    },
    targets = {
        release, debug, libraryDebug, libraryRelease, publishViatorLocal
    },
}