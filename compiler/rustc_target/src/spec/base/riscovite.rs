use crate::spec::{
    Cc, FramePointer, LinkOutputKind, LinkerFlavor, Lld, PanicStrategy, RelocModel, TargetOptions,
    TlsModel, crt_objects,
};

pub(crate) fn opts() -> TargetOptions {
    let pre_link_args =
        TargetOptions::link_args(LinkerFlavor::Gnu(Cc::No, Lld::No), &["-z", "max-page-size=4096"]);

    TargetOptions {
        os: "riscovite".into(),
        exe_suffix: "!".into(),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        linker: Some("clang".into()),
        dynamic_linking: false,
        dll_tls_export: false,
        relocation_model: RelocModel::Static,
        pre_link_args,
        pre_link_objects: crt_objects::new(&[(LinkOutputKind::StaticNoPicExe, &["crt1.o"])]),
        position_independent_executables: false,
        has_thread_local: true,
        tls_model: TlsModel::LocalExec,
        frame_pointer: FramePointer::NonLeaf,
        panic_strategy: PanicStrategy::Abort,
        ..Default::default()
    }
}
