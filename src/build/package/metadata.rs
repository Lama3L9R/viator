use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Arch {
    X64,
    Armv9,
    Armv8,
    Armv7,
    Armv6,
    Riscv32,
    Riscv64,
    Xtensa,
    Mips,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArchExtension {
    // X64
    Avx,
    Avx2,
    Avx512,
    Sse,
    Sse2,
    Sse3,
    #[serde(rename = "sse4.1")]
    Sse4_1,
    #[serde(rename = "sse4.2")]
    Sse4_2,
    Baseline,
    V2,
    V3,
    V4,

    // Arm v6 / v7
    Vfpv2,
    Vfp,
    Software,
    Thumb,
    Vfpv3,
    Vfpv4,
    Neon,

    // Arm v8
    V8,
    #[serde(rename = "v8.1")] V8_1,
    #[serde(rename = "v8.2")] V8_2,
    #[serde(rename = "v8.3")] V8_3,
    #[serde(rename = "v8.4")] V8_4,
    #[serde(rename = "v8.5")] V8_5,
    #[serde(rename = "v8.6")] V8_6,
    #[serde(rename = "v8.7")] V8_7,
    #[serde(rename = "v8.8")] V8_8,
    #[serde(rename = "v8.9")] V8_9,
    Sve,
    Sve2,

    // Arm v9
    V9,
    #[serde(rename = "v9.1")] V9_1,
    #[serde(rename = "v9.2")] V9_2,
    #[serde(rename = "v9.3")] V9_3,
    #[serde(rename = "v9.4")] V9_4,
    #[serde(rename = "v9.5")] V9_5,
    #[serde(rename = "v9.6")] V9_6,
    #[serde(rename = "v9.7")] V9_7,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BinaryWrapperFormat {
    Elf,
    Pe,
    #[serde(rename = "mach-o")]
    MachO,
    Bare,
    #[serde(rename = "bare-hex")]
    BareHex,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct File {
    pub path: String,
    pub hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFile {
    pub path: String,
    pub hash: String,
    pub partial: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DependencyType {
    Static,
    Dynamic,
    Symbolic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub group: String,
    pub name: String,
    pub version: String,
    pub features: Vec<String>,

    #[serde(rename = "type")]
    pub dependency_type: DependencyType,
    pub link_priority: i32,

    #[serde(flatten)]
    pub custom_data: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variant {
    pub arch: Arch,
    pub extensions: Vec<ArchExtension>,
    pub format: BinaryWrapperFormat,
    pub features: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetadataFiles {
    pub headers: Option<File>,

    #[serde(rename = "static")]
    pub static_file: Option<File>,
    pub static_pic: Option<File>,
    pub dynamic: Option<File>,
    pub sources: Option<SourceFile>,

    #[serde(flatten)]
    pub custom_data: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub group: String,
    pub name: String,
    pub version: String,

    pub files: MetadataFiles,
    pub dependencies: Vec<Dependency>,
    pub variant: Variant,
    pub variant_hash: String,

    #[serde(flatten)]
    pub custom_data: HashMap<String, String>,
}