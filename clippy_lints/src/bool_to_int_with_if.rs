use rustc_ast::{
    ast::{self, Block, ExprKind, LitKind, StmtKind},
    ptr::P,
};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// ### What it does
    /// This lint warns about if statements being used to convert a boolean value into an integer.
    ///
    /// ### Why is this bad?
    ///It is more concise to use the equivalent.
    ///
    /// ### Example
    /// ```rust
    /// // Bad
    /// if condition {
    ///     1_i64
    /// } else {
    ///     0
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// // Good
    /// i64::from(condition)
    /// condition as i64
    ///
    /// ```
    #[clippy::version = "1.59.0"]
    pub BOOL_TO_INT_WITH_IF,
    style,
    "Using an if statement to convert a boolean value into an integer"
}
declare_lint_pass!(BoolToIntWithIf => [BOOL_TO_INT_WITH_IF]);

impl EarlyLintPass for BoolToIntWithIf {
    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &ast::Expr) {
        if expr.span.from_expansion() {
            return;
        }
        if let ExprKind::If(_, block, else_expr) = &expr.kind {
            let if_block_matches = check_literal(block, 1);
            let else_block_matches = match else_expr {
                Some(expr) => {
                    if let ExprKind::Block(block, _) = &expr.kind {
                        check_literal(&block, 0)
                    } else {
                        false
                    }
                },
                None => false,
            };

            if if_block_matches && else_block_matches {
                //Lint here

                //span_lint(cx)
            }
        }
    }
}

fn check_literal(block: &P<Block>, expected: u128) -> bool {
    if_chain! {
        if block.stmts.len() == 1;
        if let StmtKind::Expr(inner) = &block.stmts[0].kind;
        if let ExprKind::Lit(literal) = &inner.kind;
        if let LitKind::Int(literal_value, _) = &literal.kind;
        then {
            *literal_value == expected
        } else {
            false
        }
    }
}
