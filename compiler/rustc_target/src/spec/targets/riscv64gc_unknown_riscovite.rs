use std::borrow::Cow;

use crate::spec::{CodeModel, RelroLevel, SplitDebuginfo, Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "riscv64-unknown-riscovite".into(),
        metadata: crate::spec::TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: None,
        },
        pointer_width: 64,
        data_layout: "e-m:e-p:64:64-i64:64-i128:128-n32:64-S128".into(),
        arch: "riscv64".into(),
        options: TargetOptions {
            code_model: Some(CodeModel::Medium),
            cpu: "generic-rv64".into(),
            features: "+m,+a,+f,+d,+c".into(),
            llvm_abiname: "lp64d".into(),
            max_atomic_width: Some(64),
            supported_split_debuginfo: Cow::Borrowed(&[SplitDebuginfo::Off]),
            os: "riscovite".into(),
            dynamic_linking: false,
            relro_level: RelroLevel::Off,
            has_thread_local: false,
            crt_static_default: true,
            crt_static_respected: true,
            ..Default::default()
        },
    }
}
