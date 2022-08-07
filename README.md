To induce an internal compiler error in clippy (as of build affe0d3a 2022-08-05), run `cargo clippy --tests` on this repo

```
$ RUST_BACKTRACE=full cargo clippy --tests
    Checking clippy-crash v0.1.0 (/Users/mayfield/clippy-crash)
thread 'rustc' panicked at 'byte index 18446744073709551614 is out of bounds of `0`', library/core/src/str/mod.rs:107:9
stack backtrace:
   0:        0x103252f14 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha126ba0792dd74da
   1:        0x1032a01dc - core::fmt::write::h8d774bd99c60cd5c
   2:        0x103245de0 - std::io::Write::write_fmt::hdf7b50b71460d456
   3:        0x103255a24 - std::panicking::default_hook::{{closure}}::h5bf4f1c0ad1cb9fc
   4:        0x103255744 - std::panicking::default_hook::hc84c31658e93785e
   5:        0x1020a0800 - clippy_driver[e76d520b3bfffe3a]::ICE_HOOK::{closure#0}::{closure#0}
   6:        0x1032560e8 - std::panicking::rust_panic_with_hook::he4637d05351524f5
   7:        0x103255f90 - std::panicking::begin_panic_handler::{{closure}}::h37354a056ff1aa8c
   8:        0x103253420 - std::sys_common::backtrace::__rust_end_short_backtrace::h7fe3a7a4e8db0e47
   9:        0x103255ce8 - _rust_begin_unwind
  10:        0x1032cb5d8 - core::panicking::panic_fmt::hff88c6af52b75a0d
  11:        0x1032a583c - core::str::slice_error_fail_rt::h14267998c211fb3c
  12:        0x103294fec - core::ops::function::FnOnce::call_once::hef3d12418070189c
  13:        0x10329b54c - core::intrinsics::const_eval_select::hb0f255b283cdb2ce
  14:        0x1032cb800 - core::str::slice_error_fail::h42952e610cb67f38
  15:        0x102466360 - <clippy_utils[367679a6d73bc7c9]::numeric_literal::NumericLiteral>::from_lit_kind
  16:        0x102257ce4 - <clippy_lints[6712ea8630f2c74e]::literal_representation::LiteralDigitGrouping as rustc_lint[260655b628567752]::passes::EarlyLintPass>::check_expr
  17:        0x10e43d780 - <rustc_lint[260655b628567752]::early::EarlyLintPassObjects as rustc_lint[260655b628567752]::passes::EarlyLintPass>::check_expr
  18:        0x10ab9a4bc - <rustc_lint[260655b628567752]::early::EarlyContextAndPass<rustc_lint[260655b628567752]::early::EarlyLintPassObjects> as rustc_ast[b3a3420db6da638e]::visit::Visitor>::visit_expr
  19:        0x10abb2e24 - rustc_ast[b3a3420db6da638e]::visit::walk_variant::<rustc_lint[260655b628567752]::early::EarlyContextAndPass<rustc_lint[260655b628567752]::early::EarlyLintPassObjects>>
  20:        0x10abb321c - rustc_ast[b3a3420db6da638e]::visit::walk_enum_def::<rustc_lint[260655b628567752]::early::EarlyContextAndPass<rustc_lint[260655b628567752]::early::EarlyLintPassObjects>>
  21:        0x10abbe030 - rustc_ast[b3a3420db6da638e]::visit::walk_item::<rustc_lint[260655b628567752]::early::EarlyContextAndPass<rustc_lint[260655b628567752]::early::EarlyLintPassObjects>>
  22:        0x10abc04e8 - rustc_ast[b3a3420db6da638e]::visit::walk_stmt::<rustc_lint[260655b628567752]::early::EarlyContextAndPass<rustc_lint[260655b628567752]::early::EarlyLintPassObjects>>
  23:        0x10abb8bd0 - rustc_ast[b3a3420db6da638e]::visit::walk_fn::<rustc_lint[260655b628567752]::early::EarlyContextAndPass<rustc_lint[260655b628567752]::early::EarlyLintPassObjects>>
  24:        0x10abb406c - rustc_ast[b3a3420db6da638e]::visit::walk_assoc_item::<rustc_lint[260655b628567752]::early::EarlyContextAndPass<rustc_lint[260655b628567752]::early::EarlyLintPassObjects>>
  25:        0x10abbe6bc - rustc_ast[b3a3420db6da638e]::visit::walk_item::<rustc_lint[260655b628567752]::early::EarlyContextAndPass<rustc_lint[260655b628567752]::early::EarlyLintPassObjects>>
  26:        0x10abb21ac - rustc_ast[b3a3420db6da638e]::visit::walk_crate::<rustc_lint[260655b628567752]::early::EarlyContextAndPass<rustc_lint[260655b628567752]::early::EarlyLintPassObjects>>
  27:        0x10ab99608 - rustc_lint[260655b628567752]::early::early_lint_node::<rustc_lint[260655b628567752]::early::EarlyLintPassObjects, &rustc_ast[b3a3420db6da638e]::ast::Crate>
  28:        0x10ab97fd4 - rustc_lint[260655b628567752]::early::check_ast_node::<rustc_lint[260655b628567752]::BuiltinCombinedEarlyLintPass, &rustc_ast[b3a3420db6da638e]::ast::Crate>
  29:        0x10ab82b5c - <rustc_session[ff6ccfb400bec4c9]::session::Session>::time::<(), rustc_interface[30d9b80919e68b2]::passes::configure_and_expand::{closure#8}>
  30:        0x10ab7935c - <rustc_interface[30d9b80919e68b2]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[30d9b80919e68b2]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[aebee68127987325]::result::Result<rustc_ast[b3a3420db6da638e]::ast::Crate, rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>>
  31:        0x10ab5d348 - <rustc_interface[30d9b80919e68b2]::queries::Queries>::expansion
  32:        0x10aa594ac - <rustc_interface[30d9b80919e68b2]::interface::Compiler>::enter::<rustc_driver[3a1d2be0e85867c]::run_compiler::{closure#1}::{closure#2}, core[aebee68127987325]::result::Result<core[aebee68127987325]::option::Option<rustc_interface[30d9b80919e68b2]::queries::Linker>, rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>>
  33:        0x10aa4fd90 - rustc_span[41db585a49df33a6]::with_source_map::<core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>, rustc_interface[30d9b80919e68b2]::interface::create_compiler_and_run<core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>, rustc_driver[3a1d2be0e85867c]::run_compiler::{closure#1}>::{closure#1}>
  34:        0x10aa768a8 - rustc_interface[30d9b80919e68b2]::interface::create_compiler_and_run::<core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>, rustc_driver[3a1d2be0e85867c]::run_compiler::{closure#1}>
  35:        0x10aac1778 - <scoped_tls[7ce985231f4fc1ba]::ScopedKey<rustc_span[41db585a49df33a6]::SessionGlobals>>::set::<rustc_interface[30d9b80919e68b2]::interface::run_compiler<core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>, rustc_driver[3a1d2be0e85867c]::run_compiler::{closure#1}>::{closure#0}, core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>>
  36:        0x10aa55b40 - std[261646b3aede1745]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[30d9b80919e68b2]::util::run_in_thread_pool_with_globals<rustc_interface[30d9b80919e68b2]::interface::run_compiler<core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>, rustc_driver[3a1d2be0e85867c]::run_compiler::{closure#1}>::{closure#0}, core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>>::{closure#0}, core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>>
  37:        0x10aa77164 - <<std[261646b3aede1745]::thread::Builder>::spawn_unchecked_<rustc_interface[30d9b80919e68b2]::util::run_in_thread_pool_with_globals<rustc_interface[30d9b80919e68b2]::interface::run_compiler<core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>, rustc_driver[3a1d2be0e85867c]::run_compiler::{closure#1}>::{closure#0}, core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>>::{closure#0}, core[aebee68127987325]::result::Result<(), rustc_errors[10c8ee89ef9a508d]::ErrorGuaranteed>>::{closure#1} as core[aebee68127987325]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  38:        0x10325e798 - std::sys::unix::thread::Thread::new::thread_start::hb2e089c87ccd2631
  39:        0x1bf17c26c - __pthread_deallocate

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.64 (affe0d3a 2022-08-05)

query stack during panic:
end of query stack
error: could not compile `clippy-crash`
```