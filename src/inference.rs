use crate::core::commit::CommitNodeInner;
use crate::core::redeem::NodeType;
use crate::core::types::{RcVar, Type, Variable, VariableFactory, VariableInner, VariableType};
use crate::jet::Jet;
use crate::{CommitNode, Error};
use std::convert::TryFrom;
use std::rc::Rc;
use std::sync::Arc;
use std::{cmp, fmt, mem};

/// Find the representative of the set of variable `x`
/// using _path halving_ of the union-find algorithm.
fn find_root(mut x: RcVar) -> RcVar {
    loop {
        // To allow moving out of `x` we clone its inner value. Because `VariableInner`s
        // are either trivial or contain more `Rc`s, this is still an O(1) shallow copy.
        //
        // While this does create a new variable which is independent of `x`, because it
        // clones the inner `Rc`s rather than deep-copying them, as soon as we follow
        // any of the interior references, it's as though we were using `x` itself. We
        // make sure to either do this or throw away the copy to ensure that this is
        // logically equivalent to directly using `x`.
        //
        // Ideally As of Rust 1.67 this appears to be necessary due to limitations of
        // the borrowck, and this pattern is used repeatedly throughout this module.
        let x_var = x.borrow().inner.clone();
        let parent = if let VariableInner::EqualTo(parent) = x_var {
            parent
        } else {
            // If there is no parent, then return the current node.
            return x;
        };

        // Clone inner to un-borrow `parent`; see above comment.
        let parent_var = parent.borrow().inner.clone();
        if let VariableInner::EqualTo(grandparent) = parent_var {
            // Update the parent pointer to the grandparent, and go to the grandparent.
            x.borrow_mut().inner = VariableInner::EqualTo(grandparent.clone());
            x = grandparent;
        } else {
            // If there is no grandparent, then return the parent.
            return parent;
        }
    }
}

/// Unify the sets of variables `x` and `y`
/// using _union by rank_ of the union-find algorithm.
fn unify(mut x: RcVar, mut y: RcVar, hint: &'static str) -> Result<(), Error> {
    x = find_root(x);
    y = find_root(y);

    // x and y are already in the same set
    if Rc::ptr_eq(&x, &y) {
        return Ok(());
    }

    // Adjust ranks for union-find path halving
    let rank_ord = x.borrow().rank.cmp(&y.borrow().rank);
    match rank_ord {
        // If x.rank < y.rank, then swap x and y to ensure x.rank ≥ y.rank
        // Always unify tree with smaller rank into tree with larger rank
        cmp::Ordering::Less => mem::swap(&mut x, &mut y),
        // If x.rank = y.rank, then increment the rank of x
        // The rank of x increases by making y its child
        cmp::Ordering::Equal => x.borrow_mut().rank += 1,
        _ => {}
    }

    // Make x the parent of y
    let old_y_var = mem::replace(&mut y.borrow_mut().inner, VariableInner::EqualTo(x.clone()));
    match old_y_var {
        VariableInner::Free(_) => Ok(()),
        // If y was already bound to a type, then x must be bound, too
        VariableInner::Bound(y_ty, _) => bind(&x, y_ty, hint),
        VariableInner::EqualTo(..) => unreachable!("A root node cannot have a parent"),
        VariableInner::Finalized(..) => unreachable!("No finalized types at this stage"),
    }
}

/// Bind variable `x` to type `ty`.
///
/// Fails if `x` is already bound to a type that is incompatible with `ty`.
fn bind(x: &RcVar, ty: VariableType, hint: &'static str) -> Result<(), Error> {
    // Clone inner to un-borrow the root; see comment inside `find_root` for more explanation
    let x_var = find_root(x.clone()).borrow().inner.clone(); //x.borrow().inner.clone();
    match x_var {
        VariableInner::Free(_) => {
            x.borrow_mut().inner = VariableInner::Bound(ty, false);
            Ok(())
        }
        VariableInner::Bound(self_ty, _) => match (self_ty, ty) {
            (VariableType::Unit, VariableType::Unit) => Ok(()),
            (VariableType::Sum(x1, x2), VariableType::Sum(y1, y2))
            | (VariableType::Product(x1, x2), VariableType::Product(y1, y2)) => {
                unify(x1, y1, hint)?;
                unify(x2, y2, hint)
            }
            _ => Err(Error::Unification(hint)),
        },
        VariableInner::EqualTo(..) => unreachable!("x_var is a root node"),
        VariableInner::Finalized(..) => unreachable!("No finalized types at this stage"),
    }
}

/// Converts variable `x` into a finalized type.
///
/// Free variables are finalized as _unit_.
/// Bound variables are recursively finalized as the units, sums and products of their types.
///
/// Fails if a variable occurs recursively in the type it is bound to _(occurs check)_.
fn finalize(x: RcVar) -> Result<Arc<Type>, Error> {
    let x = find_root(x);

    let variable_type = match x.borrow_mut().inner {
        VariableInner::Free(_) => VariableType::Unit,
        VariableInner::Bound(ref ty, ref mut occurs_check) => {
            if *occurs_check {
                return Err(Error::OccursCheck);
            }
            *occurs_check = true;
            ty.clone()
        }
        VariableInner::EqualTo(..) => unreachable!("A root cannot have a parent"),
        VariableInner::Finalized(ref final_type) => return Ok(final_type.clone()),
    };

    let final_type = match variable_type {
        VariableType::Unit => Type::unit(),
        VariableType::Sum(y, z) => Type::sum(finalize(y)?, finalize(z)?),
        VariableType::Product(y, z) => Type::product(finalize(y)?, finalize(z)?),
    };

    x.borrow_mut().inner = VariableInner::Finalized(final_type.clone());
    Ok(final_type)
}

/// Source and target type of a node during type inference
#[derive(Clone, Debug)]
pub struct UnificationArrow {
    /// Source type of the node
    source: RcVar,
    /// Target type of the node
    target: RcVar,
}

impl fmt::Display for UnificationArrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} → {}", self.source.borrow(), self.target.borrow())
    }
}

impl<J: Jet> TryFrom<&CommitNode<J>> for NodeType {
    type Error = Error;

    /// Return the finalized type of the given `node`.
    /// To work, this method must be called on nodes in post order!
    fn try_from(node: &CommitNode<J>) -> Result<Self, Self::Error> {
        let source_ty = match finalize(node.arrow.source.clone()) {
            Ok(ty) => ty,
            Err(error) => return Err(error),
        };
        let target_ty = match finalize(node.arrow.target.clone()) {
            Ok(ty) => ty,
            Err(error) => return Err(error),
        };

        Ok(NodeType {
            source: source_ty,
            target: target_ty,
        })
    }
}

impl UnificationArrow {
    /// Create a unification arrow for a fresh `unit` combinator
    pub(crate) fn for_unit(naming: &mut VariableFactory) -> Self {
        UnificationArrow {
            source: naming.free_variable(),
            target: Variable::bound(VariableType::Unit),
        }
    }

    /// Create a unification arrow for a fresh `iden` combinator
    pub(crate) fn for_iden(naming: &mut VariableFactory) -> Self {
        // We cheat and make the source and target of iden literally be the same
        // variable, rather than unifying them. This is slightly more efficient
        // and shouldn't result in any user-visible consequences.
        let name = naming.free_variable();
        UnificationArrow {
            source: name.clone(),
            target: name,
        }
    }

    /// Create a unification arrow for a fresh `witness` combinator
    pub(crate) fn for_witness(naming: &mut VariableFactory) -> Self {
        UnificationArrow {
            source: naming.free_variable(),
            target: naming.free_variable(),
        }
    }

    /// Create a unification arrow for a fresh `fail` combinator
    pub(crate) fn for_fail(naming: &mut VariableFactory) -> Self {
        UnificationArrow {
            source: naming.free_variable(),
            target: naming.free_variable(),
        }
    }

    /// Create a unification arrow for a fresh `hidden` combinator
    pub(crate) fn for_hidden(naming: &mut VariableFactory) -> Self {
        UnificationArrow {
            source: naming.free_variable(),
            target: naming.free_variable(),
        }
    }

    /// Create a unification arrow for a fresh jet combinator
    pub(crate) fn for_jet<J: Jet>(jet: &J) -> Self {
        let pow2s = Variable::powers_of_two();
        UnificationArrow {
            source: Variable::bound(jet.source_ty().to_variable_type(&pow2s)),
            target: Variable::bound(jet.target_ty().to_variable_type(&pow2s)),
        }
    }

    /// Create a unification arrow for a fresh jet combinator
    pub(crate) fn for_injl<J: Jet>(child: &CommitNode<J>, naming: &mut VariableFactory) -> Self {
        // Again, we "unify" by just cloning Rcs
        let source = child.arrow.source.clone();
        let target = Variable::bound(VariableType::Sum(
            child.arrow.target.clone(),
            naming.free_variable(),
        ));
        UnificationArrow { source, target }
    }

    /// Create a unification arrow for a fresh jet combinator
    pub(crate) fn for_injr<J: Jet>(child: &CommitNode<J>, naming: &mut VariableFactory) -> Self {
        // Again, we "unify" by just cloning Rcs
        let source = child.arrow.source.clone();
        let target = Variable::bound(VariableType::Sum(
            naming.free_variable(),
            child.arrow.target.clone(),
        ));
        UnificationArrow { source, target }
    }

    /// Create a unification arrow for a fresh jet combinator
    pub(crate) fn for_take<J: Jet>(child: &CommitNode<J>, naming: &mut VariableFactory) -> Self {
        // Again, we "unify" by just cloning Rcs
        let source = Variable::bound(VariableType::Product(
            child.arrow.source.clone(),
            naming.free_variable(),
        ));
        let target = child.arrow.target.clone();
        UnificationArrow { source, target }
    }

    /// Create a unification arrow for a fresh jet combinator
    pub(crate) fn for_drop<J: Jet>(child: &CommitNode<J>, naming: &mut VariableFactory) -> Self {
        // Again, we "unify" by just cloning Rcs
        let source = Variable::bound(VariableType::Product(
            naming.free_variable(),
            child.arrow.source.clone(),
        ));
        let target = child.arrow.target.clone();
        UnificationArrow { source, target }
    }

    /// Return a unification arrow that is initialized for the given `node`.
    pub(crate) fn for_node<J: Jet>(
        node: &CommitNodeInner<J>,
        naming: &mut VariableFactory,
    ) -> Result<UnificationArrow, Error> {
        match node {
            // No children
            CommitNodeInner::Unit => Ok(Self::for_unit(naming)),
            CommitNodeInner::Iden => Ok(Self::for_iden(naming)),
            CommitNodeInner::Witness => Ok(Self::for_witness(naming)),
            CommitNodeInner::Fail(_, _) => Ok(Self::for_fail(naming)),
            CommitNodeInner::Hidden(_) => Ok(Self::for_hidden(naming)),
            CommitNodeInner::Jet(ref j) => Ok(Self::for_jet(j)),
            // Single children
            CommitNodeInner::InjL(ref child) => Ok(Self::for_injl(child, naming)),
            CommitNodeInner::InjR(ref child) => Ok(Self::for_injr(child, naming)),
            CommitNodeInner::Take(ref child) => Ok(Self::for_take(child, naming)),
            CommitNodeInner::Drop(ref child) => Ok(Self::for_drop(child, naming)),
            // Multiple children -- now we have nontrivial unification and may fail.
            CommitNodeInner::Comp(ref lchild, ref rchild) => {
                let arrow = UnificationArrow {
                    source: lchild.arrow.source.clone(),
                    target: rchild.arrow.target.clone(),
                };
                unify(
                    lchild.arrow.target.clone(),
                    rchild.arrow.source.clone(),
                    "Composition: Left target = right source",
                )?;
                Ok(arrow)
            }
            CommitNodeInner::Case(ref lchild, ref rchild)
            | CommitNodeInner::AssertL(ref lchild, ref rchild)
            | CommitNodeInner::AssertR(ref lchild, ref rchild) => {
                let a = naming.free_variable();
                let b = naming.free_variable();
                let c = naming.free_variable();

                let sum_a_b = VariableType::Sum(a.clone(), b.clone());
                let prod_sum_a_b_c = VariableType::Product(Variable::bound(sum_a_b), c.clone());
                let source = Variable::bound(prod_sum_a_b_c);

                let target = match node {
                    CommitNodeInner::AssertL(..) => {
                        bind(
                            &find_root(lchild.arrow.source.clone()),
                            VariableType::Product(a, c),
                            "Case: Left source = A × C",
                        )?;
                        lchild.arrow.target.clone()
                    }
                    CommitNodeInner::AssertR(..) => {
                        bind(
                            &find_root(rchild.arrow.source.clone()),
                            VariableType::Product(b, c),
                            "Case: Right source = B × C",
                        )?;
                        rchild.arrow.target.clone()
                    }
                    CommitNodeInner::Case(..) => {
                        bind(
                            &find_root(lchild.arrow.source.clone()),
                            VariableType::Product(a, c.clone()),
                            "Case: Left source = A × C",
                        )?;
                        bind(
                            &find_root(rchild.arrow.source.clone()),
                            VariableType::Product(b, c),
                            "Case: Right source = B × C",
                        )?;
                        unify(
                            lchild.arrow.target.clone(),
                            rchild.arrow.target.clone(),
                            "Case: Left target = right target",
                        )?;
                        rchild.arrow.target.clone()
                    }
                    _ => unreachable!(),
                };

                Ok(UnificationArrow { source, target })
            }
            CommitNodeInner::Pair(ref lchild, ref rchild) => {
                unify(
                    lchild.arrow.source.clone(),
                    rchild.arrow.source.clone(),
                    "Pair: Left source = Right source",
                )?;

                Ok(UnificationArrow {
                    source: lchild.arrow.source.clone(),
                    target: Variable::bound(VariableType::Product(
                        lchild.arrow.target.clone(),
                        rchild.arrow.target.clone(),
                    )),
                })
            }
            CommitNodeInner::Disconnect(ref lchild, ref rchild) => {
                let a = naming.free_variable();
                let b = naming.free_variable();
                let c = rchild.arrow.source.clone();
                let d = rchild.arrow.target.clone();

                let pow2s = Variable::powers_of_two();
                let prod_256_a = VariableType::Product(pow2s[8].clone(), a.clone());
                let prod_b_c = VariableType::Product(b.clone(), c);
                let prod_b_d = VariableType::Product(b, d);

                bind(
                    &lchild.arrow.source,
                    prod_256_a,
                    "Disconnect: Left source = 2^256 × A",
                )?;
                bind(
                    &lchild.arrow.target,
                    prod_b_c,
                    "Disconnect: Left target = B × C",
                )?;

                Ok(UnificationArrow {
                    source: a,
                    target: Variable::bound(prod_b_d),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::types::Variable;

    #[test]
    fn type_error() {
        let mut naming = VariableFactory::new();
        let pow2s = Variable::powers_of_two();
        let x = naming.free_variable();
        let y = naming.free_variable();

        let x1 = naming.free_variable();
        let x2 = naming.free_variable();
        bind(&x, VariableType::Sum(x1, x2), "Cannot fail").unwrap();
        bind(
            &y,
            VariableType::Product(pow2s[8].clone(), naming.free_variable()),
            "Cannot fail",
        )
        .unwrap();

        unify(x, y, "Always fails").unwrap_err();
    }
}
