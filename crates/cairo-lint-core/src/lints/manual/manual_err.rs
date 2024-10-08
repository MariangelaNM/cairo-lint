use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::Severity;
use cairo_lang_syntax::node::ast::{ExprIf, ExprMatch};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::TypedSyntaxNode;

use crate::lints::manual::{check_manual, check_manual_if, ManualLint};

pub const MANUAL_ERR: &str = "Manual match for `err` detected. Consider using `err()` instead";

pub fn check_manual_err(db: &dyn SyntaxGroup, expr_match: &ExprMatch, diagnostics: &mut Vec<PluginDiagnostic>) {
    if check_manual(db, expr_match, ManualLint::ManualErr) {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: expr_match.as_syntax_node().stable_ptr(),
            message: MANUAL_ERR.to_owned(),
            severity: Severity::Warning,
        });
    }
}

pub fn check_manual_if_err(db: &dyn SyntaxGroup, expr_if: &ExprIf, diagnostics: &mut Vec<PluginDiagnostic>) {
    if check_manual_if(db, expr_if, ManualLint::ManualErr) {
        diagnostics.push(PluginDiagnostic {
            stable_ptr: expr_if.as_syntax_node().stable_ptr(),
            message: MANUAL_ERR.to_owned(),
            severity: Severity::Warning,
        });
    }
}
