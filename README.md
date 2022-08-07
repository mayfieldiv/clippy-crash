https://github.com/rust-lang/rust-clippy/issues/9297

To induce an internal compiler error in clippy (as of build affe0d3a 2022-08-05), run `cargo clippy --tests` on this repo

```
$ RUST_BACKTRACE=1 cargo clippy --tests
    Checking clippy-crash v0.1.0 (/Users/mayfield/clippy-crash)
thread 'rustc' panicked at 'byte index 18446744073709551614 is out of bounds of `0`', library/core/src/str/mod.rs:107:9
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::str::slice_error_fail_rt
   3: core::ops::function::FnOnce::call_once
   4: core::intrinsics::const_eval_select
   5: core::str::slice_error_fail
   6: <clippy_utils::numeric_literal::NumericLiteral>::from_lit_kind
   7: <clippy_lints::literal_representation::LiteralDigitGrouping as rustc_lint::passes::EarlyLintPass>::check_expr
   8: <rustc_lint::early::EarlyLintPassObjects as rustc_lint::passes::EarlyLintPass>::check_expr
   9: <rustc_lint::early::EarlyContextAndPass<rustc_lint::early::EarlyLintPassObjects> as rustc_ast::visit::Visitor>::visit_expr
  10: rustc_ast::visit::walk_variant::<rustc_lint::early::EarlyContextAndPass<rustc_lint::early::EarlyLintPassObjects>>
  11: rustc_ast::visit::walk_enum_def::<rustc_lint::early::EarlyContextAndPass<rustc_lint::early::EarlyLintPassObjects>>
  12: rustc_ast::visit::walk_item::<rustc_lint::early::EarlyContextAndPass<rustc_lint::early::EarlyLintPassObjects>>
  13: rustc_ast::visit::walk_stmt::<rustc_lint::early::EarlyContextAndPass<rustc_lint::early::EarlyLintPassObjects>>
  14: rustc_ast::visit::walk_fn::<rustc_lint::early::EarlyContextAndPass<rustc_lint::early::EarlyLintPassObjects>>
  15: rustc_ast::visit::walk_assoc_item::<rustc_lint::early::EarlyContextAndPass<rustc_lint::early::EarlyLintPassObjects>>
  16: rustc_ast::visit::walk_item::<rustc_lint::early::EarlyContextAndPass<rustc_lint::early::EarlyLintPassObjects>>
  17: rustc_ast::visit::walk_crate::<rustc_lint::early::EarlyContextAndPass<rustc_lint::early::EarlyLintPassObjects>>
  18: rustc_lint::early::early_lint_node::<rustc_lint::early::EarlyLintPassObjects, &rustc_ast::ast::Crate>
  19: rustc_lint::early::check_ast_node::<rustc_lint::BuiltinCombinedEarlyLintPass, &rustc_ast::ast::Crate>
  20: <rustc_session::session::Session>::time::<(), rustc_interface::passes::configure_and_expand::{closure#8}>
  21: <rustc_interface::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface::queries::Queries>::expansion::{closure#0}::{closure#0}, core::result::Result<rustc_ast::ast::Crate, rustc_errors::ErrorGuaranteed>>
  22: <rustc_interface::queries::Queries>::expansion
  23: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorGuaranteed>>
  24: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  25: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
  26: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.64 (affe0d3a 2022-08-05)

query stack during panic:
end of query stack
error: could not compile `clippy-crash`
```
