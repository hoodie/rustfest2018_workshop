#![feature(rustc_private)]
#![feature(macro_vis_matcher)]

extern crate clippy_lints;
extern crate rustfest2018_workshop;
extern crate syntax_pos;

#[macro_use]
extern crate rustc;
extern crate syntax;

mod no_flags {
    use rustc::hir::intravisit::FnKind as HirFnKind;
    use rustc::hir::Body;
    use rustc::hir::FnDecl as HirFnDecl;
    use rustc::lint::*;
    use syntax::ast::{FnDecl as AstFnDecl, NodeId};
    use syntax::visit;
    use rustc::ty;
    use syntax_pos::Span;

    declare_lint! {
        pub BOOLARGS,
        Warn,
        "bool flags in function calls are a sign of a bad api"
    }

    pub struct NoFlags;
    impl LintPass for NoFlags {
        fn get_lints(&self) -> LintArray {
            lint_array!(BOOLARGS)
        }
    }

    impl EarlyLintPass for NoFlags {
        fn check_fn(&mut self, cx: &EarlyContext, _: visit::FnKind, decl: &AstFnDecl, fnspan: Span, _: NodeId) {
            for input in &decl.inputs {
                let typ = format!("{:?}", input.ty);
                if typ.contains("type(bool)") {
                    cx.span_lint(
                        BOOLARGS,
                        fnspan,
                        "using boolean flags, please try to design your API differently",
                    )
                }
            }
        }
    }

    impl<'a, 'tcx> LateLintPass<'a, 'tcx> for NoFlags {
        fn check_fn(
            &mut self,
            cx: &LateContext<'a, 'tcx>,
            _: HirFnKind<'tcx>,
            decl: &'tcx HirFnDecl,
            _: &'tcx Body,
            fn_span: Span,
            fn_id: NodeId,
        ) {
            let fn_def_id = cx.tcx.hir.local_def_id(fn_id);
            let sig = cx.tcx.fn_sig(fn_def_id);
            let fn_ty = sig.skip_binder();

            for (_idx, (_arg, ty)) in decl.inputs.iter().zip(fn_ty.inputs()).enumerate() {
                if let ty::TyBool = ty.sty {
                    cx.span_lint(
                        BOOLARGS,
                        fn_span,
                        "using boolean flags, please try to design your API differently",
                    )
                }
            }
        }
    }
}

mod no_transmute {
    use rustc::lint::*;
    use syntax::ast::*;
    declare_lint! {
        pub TRANSMUTE,
        Forbid,
        "the interns keep taking shortcuts that bite us later"
    }

    pub struct NoTransmute;

    impl LintPass for NoTransmute {
        fn get_lints(&self) -> LintArray {
            lint_array!(TRANSMUTE)
        }
    }

    impl EarlyLintPass for NoTransmute {
        fn check_ident(&mut self, cx: &EarlyContext, ident: Ident) {
            if ident.to_string().contains("transmute") {
                cx.span_lint(
                    TRANSMUTE,
                    ident.span,
                    "no. No. NO. NOOOOOO!!!! Like seriously, doesn't anyone read our coding guidelines?",
                );
            }
        }
    }
}

use no_transmute::NoTransmute;
use no_flags::NoFlags;

fn main() {
    rustfest2018_workshop::run_lints(|ls| {
        ls.register_early_pass(None, false, Box::new(NoTransmute));
        ls.register_late_pass(None, false, Box::new(NoFlags));
    });
}
