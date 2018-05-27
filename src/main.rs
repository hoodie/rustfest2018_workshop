#![feature(rustc_private)]
#![feature(macro_vis_matcher)]

extern crate clippy_lints;
extern crate rustfest2018_workshop;
extern crate syntax_pos;

#[macro_use]
extern crate rustc;
extern crate syntax;



mod no_flags {
    use rustc::lint::*;
    use syntax::ast::{FnDecl, NodeId};
    use syntax::visit::*;
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
        fn check_fn(&mut self, cx: &EarlyContext, _: FnKind, decl: &FnDecl, fnspan: Span, _: NodeId) {
            for input in &decl.inputs {
                let typ = format!("{:?}", input.ty);
                if typ.contains("type(bool)") {
                    cx.span_lint(
                        BOOLARGS,
                        fnspan,
                        "using boolean flags, please try to design your API differently"
                        )
                }
            }
        }
    }

}

mod no_transmute {
    use rustc::lint::*;
    use syntax::ast::Ident;
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
        ls.register_early_pass(None, false, Box::new(NoFlags));
    });
}
