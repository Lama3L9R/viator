--!nocheck
-- get rid of luau type warnings for now, full type def in future
-- Set minimum viator version to 1
-- @v:minVersion = 1

local std = V.require("viator/CLangStdLibs")

local dependencies = {
	V.dynamic("libc"), -- or stl depends on your need
	std.auto(), -- or let plugin decide
	V.static("icu.lama.libutils")
}

local buildProject = V.target {
	pipeline = {
		V.action("V:PrepareDependencies", dependencies),
		V.action("CLang:CompileAll", {
			sourceRoot = "src", -- Task write results to context["CLang:Objects"]: List<{ src: path, obj: path }>
		}),
		V.action("CLang:LinkObjects", { -- Task writes to context["Binaries"].push({ BinaryInfo })
			produce = {
				{
					name = "examplePlugin",
					objects = { }, -- Default to all dependency + all objects in "context["CLang:Objects"]"
								   -- To select objects, pass: { include: { src: path | regex, obj: path | regex }, exclude: {...} }
					               -- or pass: function(ctx) -> IncludedObjects
					dynamic = true,
					static = true
				}
			}
		})
	}
}

local publishProject = V.target {
	pipeline = {
		V.action("V:ExecutePipeline", {
			pipeline = buildProject,
			isolated = false -- let context merge into current context, so we can obtain linked ELFs
		}),
		V.action("V:CollectFiles", {
			rules = {
				{
					pattern = "src/*.h",
					action = "tar",    -- folder/tar/zip/7z/ar/targz/tarxz/wtf (a meme form: .iso.zstd.hex.br)
					out = "headers",   -- will be at: out/<target-triplet?>/<pipeline>/intemediates/headers
									   -- will also write above dir to context: context["Collected:headers"]
					binary = "headers" -- should we write to context["Binaries"], any non-falsy value will be used as binary indentifier
									   -- Binary format example: { type: "elf-dynamic", path: "...", identifier = ”examplePlugin“ }
				}
			}
		}),
		V.action("V:PublishLocal", {
			artifacts = {
				":elf",   -- matches all type starts with elf (which is linked products), though if multiple entries are found
					      -- e.g. two dynamic libraries, that's an illegal build, error will be thrown
				"headers" -- publish associated headers
			}
		})
	}
}

-- To support multiple platforms:
-- Your code, idealy, is expected to support multiplatform (#error for unsupported platform, or write docs to prevent people doing random shit)
-- or if you really want restrict this, see below
-- viator platform:arch:type build # executes build target with platform:arch:type
-- e.g. viator win:armv8:release build
-- built in platforms (w/o external plugins)
-- platforms: win/linux/mac/bare # bare for embedded   default host
-- arch: x64, armv8, armv7, riscv ...                  default host
-- type: release/debug                                 default release
-- default no x86/i386 support, fuck these old shit, but we do have armv7/v6 for embedded
-- for extensions, e.g. avx512, neon, etc.: bare:armv7+neon+hfpv2+extraProcessorFeature:release

return {
	group = "icu.lama",
	name = "example",
	version = "1.0",

	targets = {
		build = buildProject,
		publish = publishProject
	},

	platforms = { -- use this to restrict supported platforms
		os = { "bare" },
		arch = { "armv7" }
	}
}