use prusti_specs::specifications::common;
use prusti_specs::specifications::json;
use rustc_hir::BodyId;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::{TyCtxt, TyKind};
use std::collections::HashMap;
use rustc_hir::hir_id::HirId;

pub use common::{ExpressionId, SpecType, SpecificationId};

/// A specification that has no types associated with it.
pub type Specification<'tcx> = common::Specification<ExpressionId, LocalDefId, TyKind<'tcx>>;
/// A set of untyped specifications associated with a single element.
pub type SpecificationSet<'tcx> = common::SpecificationSet<ExpressionId, LocalDefId, TyKind<'tcx>>;
/// A set of untyped specifications associated with a loop.
pub type LoopSpecification<'tcx> = common::LoopSpecification<ExpressionId, LocalDefId, TyKind<'tcx>>;
/// A set of untyped specifications associated with a procedure.
pub type ProcedureSpecification<'tcx> = common::ProcedureSpecification<ExpressionId, LocalDefId, TyKind<'tcx>>;
/// A map of untyped specifications for a specific crate.
pub type SpecificationMap<'tcx> = HashMap<common::SpecificationId, SpecificationSet<'tcx>>;
/// An assertion that has no types associated with it.
pub type Assertion<'tcx> = common::Assertion<ExpressionId, LocalDefId, TyKind<'tcx>>;
/// An assertion kind that has no types associated with it.
pub type AssertionKind<'tcx> = common::AssertionKind<ExpressionId, LocalDefId, TyKind<'tcx>>;
/// An expression that has no types associated with it.
pub type Expression = common::Expression<ExpressionId, LocalDefId>;
/// A trigger set that has no types associated with it.
pub type TriggerSet = common::TriggerSet<ExpressionId, LocalDefId>;
/// For all variables that have no types associated with it.
pub type ForAllVars<'tcx> = common::ForAllVars<ExpressionId, TyKind<'tcx>>;
/// A trigger that has no types associated with it.
pub type Trigger = common::Trigger<ExpressionId, LocalDefId>;

pub trait StructuralToTyped<'tcx, Target> {
    fn to_typed(self, typed_expressions: &HashMap<String, HirId>, tcx: TyCtxt<'tcx>) -> Target;
}

impl<'tcx> StructuralToTyped<'tcx, Expression> for json::Expression {
    fn to_typed(self, typed_expressions: &HashMap<String, HirId>, tcx: TyCtxt<'tcx>) -> Expression {
        let hir_id = typed_expressions[&format!("{}_{}", self.spec_id, self.expr_id)];
        let local_id = tcx.hir().local_def_id(hir_id);
        Expression {
            spec_id: self.spec_id,
            id: self.expr_id,
            expr: local_id,
        }
    }
}

impl<'tcx> StructuralToTyped<'tcx, TriggerSet> for json::TriggerSet {
    fn to_typed(self, typed_expressions: &HashMap<String, HirId>, tcx: TyCtxt<'tcx>) -> TriggerSet {
        common::TriggerSet(
            self.0
                .into_iter()
                .map(|x| x.to_typed(typed_expressions, tcx))
                .collect()
        )
    }
}

impl<'tcx> StructuralToTyped<'tcx, Trigger> for json::Trigger {
    fn to_typed(self, typed_expressions: &HashMap<String, HirId>, tcx: TyCtxt<'tcx>) -> Trigger {
        common::Trigger(
            self.0
                .into_iter()
                .map(|x| x.to_typed(typed_expressions, tcx))
                .collect()
        )
    }
}

impl<'tcx> StructuralToTyped<'tcx, ForAllVars<'tcx>> for json::ForAllVars {
    fn to_typed(self, typed_expressions: &HashMap<String, HirId>, tcx: TyCtxt<'tcx>) -> ForAllVars<'tcx> {
        let hir_id = typed_expressions[&format!("{}_{}", self.spec_id, self.expr_id)];
        let local_id = tcx.hir().local_def_id(hir_id);
        let (body, _) = tcx.mir_validated(local_id);
        let body = body.borrow();

        let vars: Vec<TyKind> = body
            .local_decls
            .iter()
            .filter(|x| x.is_user_variable())
            .map(|x| x.ty.kind.clone())
            .collect();

        assert_eq!(vars.len(), self.count);
        return ForAllVars {
            spec_id: self.spec_id,
            id: self.expr_id,
            vars
        }
    }
}

impl<'tcx> StructuralToTyped<'tcx, AssertionKind<'tcx>> for json::AssertionKind {
    fn to_typed(self, typed_expressions: &HashMap<String, HirId>, tcx: TyCtxt<'tcx>) -> AssertionKind<'tcx> {
        use json::AssertionKind::*;
        match self {
            Expr(expr) => AssertionKind::Expr(expr.to_typed(typed_expressions, tcx)),
            And(assertions) => AssertionKind::And(
                assertions.into_iter()
                          .map(|assertion| assertion.to_typed(typed_expressions, tcx))
                          .collect()
            ),
            Implies(lhs, rhs) => AssertionKind::Implies(
                lhs.to_typed(typed_expressions, tcx),
                rhs.to_typed(typed_expressions, tcx)
            ),
            ForAll(vars, body, triggers) => AssertionKind::ForAll(
                vars.to_typed(typed_expressions, tcx),
                triggers.to_typed(typed_expressions, tcx),
                body.to_typed(typed_expressions, tcx),
            )
        }
    }
}

impl<'tcx> StructuralToTyped<'tcx, Assertion<'tcx>> for json::Assertion {
    fn to_typed(self, typed_expressions: &HashMap<String, HirId>, tcx: TyCtxt<'tcx>) -> Assertion<'tcx> {
        Assertion {
            kind: box self.kind.to_typed(typed_expressions, tcx),
        }
    }
}
