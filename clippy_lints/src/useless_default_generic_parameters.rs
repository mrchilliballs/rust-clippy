use rustc_ast::ast::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for the usage of a generic argument when the type already defines a default.
    ///
    /// ### Why is this bad?
    /// It is redundant and adds visual clutter.
    ///
    /// ### Example
    /// ```no_run
    /// type Result<T = ()> = core::result::Result<T, MyError>;
    /// fn foo() -> Result<()> {
    ///     Ok(())
    /// }
    /// ```
    /// Use instead:
    /// ```no_run
    /// type Result<T = ()> = core::result::Result<T, MyError>;
    /// fn foo() -> Result {
    ///     Ok(())
    ///}
    /// ```
    #[clippy::version = "1.89.0"]
    pub USELESS_DEFAULT_GENERIC_PARAMETERS,
    style,
    "default lint description"
}
declare_lint_pass!(UselessDefaultGenericParameters => [USELESS_DEFAULT_GENERIC_PARAMETERS]);

impl LateLintPass<'_> for UselessDefaultGenericParameters {}
