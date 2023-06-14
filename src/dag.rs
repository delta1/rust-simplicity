// Rust Simplicity Library
// Written in 2020 by
//   Andrew Poelstra <apoelstra@blockstream.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! General DAG iteration utilities

use std::collections::HashMap;
use std::rc::Rc;
use std::{fmt, marker};

use crate::core::commit::{CommitNode, CommitNodeInner};
use crate::core::redeem::{RedeemNode, RedeemNodeInner};
use crate::core::Value;
use crate::jet;
use crate::Imr;

/// Generic container for Simplicity DAGs
pub enum Dag<T> {
    /// Identity
    Iden,
    /// Unit constant
    Unit,
    /// Left injection of some child
    InjL(T),
    /// Right injection of some child
    InjR(T),
    /// Take of some child
    Take(T),
    /// Drop of some child
    Drop(T),
    /// Composition of a left and right child
    Comp(T, T),
    /// Case of a left and right child
    Case(T, T),
    /// Left-assertion of a child
    AssertL(T),
    /// Right-assertion of a child
    AssertR(T),
    /// Pair of a left and right child
    Pair(T, T),
    /// Disconnect of a left and right child
    Disconnect(T, T),
    /// Witness data (missing during commitment, inserted during redemption)
    Witness,
    /// Universal fail
    Fail,
    /// Hidden CMR
    Hidden,
    /// Application jet
    Jet,
    /// Constant word
    Word,
}

/// How much sharing/expansion to do when running an iterator over a DAG
///
/// This object works by recording and looking up nodes in a DAG as they are
/// being iterated over. If the tracker says that an element has been seen
/// before, it will not be yielded again; so for example, a tracker which
/// records nodes by their IMR will implement full sharing, while a tracker
/// which claims to have never seen any node before will implement no
/// sharing at all.
pub trait SharingTracker<D: DagLike> {
    /// Marks an object as having been seen, and record the index
    /// when it was seen.
    fn record(
        &mut self,
        object: &D,
        index: usize,
        left_child: Option<usize>,
        right_child: Option<usize>,
    );

    /// Check whether an object has been seen before; if so, return
    /// the index it was recorded at.
    fn seen_before(&self, object: &D) -> Option<usize>;
}

// Annoyingly we need to implement this explicitly
impl<D: DagLike> SharingTracker<D> for &mut dyn SharingTracker<D> {
    fn record(
        &mut self,
        object: &D,
        index: usize,
        left_child: Option<usize>,
        right_child: Option<usize>,
    ) {
        (**self).record(object, index, left_child, right_child)
    }
    fn seen_before(&self, object: &D) -> Option<usize> {
        (**self).seen_before(object)
    }
}

/// Do not share at all; yield every node in the expanded DAG
#[derive(Clone, Debug, Default)]
pub struct NoSharing;

impl<D: DagLike> SharingTracker<D> for NoSharing {
    fn record(&mut self, _: &D, _: usize, _: Option<usize>, _: Option<usize>) {}
    fn seen_before(&self, _: &D) -> Option<usize> {
        None
    }
}

/// Share using pointer identity, i.e. yield each node only once, where two
/// nodes are the same iff they both point to the same object.
#[derive(Clone, Debug, Default)]
pub struct InternalSharing {
    map: HashMap<PointerId, usize>,
}

impl<D: DagLike> SharingTracker<D> for InternalSharing {
    fn record(&mut self, d: &D, index: usize, _: Option<usize>, _: Option<usize>) {
        self.map.insert(PointerId::from(d), index);
    }
    fn seen_before(&self, d: &D) -> Option<usize> {
        self.map.get(&PointerId::from(d)).copied()
    }
}

/// Full sharing: yield a node with each IMR only once. This can only be
/// used with `RedeemNode`s, since `CommitNode`s do not have enough
/// information to determine their IMRs.
#[derive(Clone, Debug, Default)]
pub struct FullSharing {
    map: HashMap<Imr, usize>,
}

impl<J: jet::Jet> SharingTracker<&RedeemNode<J>> for FullSharing {
    fn record(&mut self, d: &&RedeemNode<J>, index: usize, _: Option<usize>, _: Option<usize>) {
        self.map.insert(d.imr, index);
    }
    fn seen_before(&self, d: &&RedeemNode<J>) -> Option<usize> {
        self.map.get(&d.imr).copied()
    }
}

/// A wrapper around any other sharing tracker which forces `witness` and
/// `disconnect` combinators, and their ancestors, to be unshared
///
/// This is useful when providing tools for users to manipulate non-final
/// programs (programs without witnesses), they should always be presented
/// with a view in which witness and disconnect nodes are unshared, since
/// these nodes take input data which is not committed in the CMR.
///
/// If these nodes were shared, it may create the impression that data
/// attached to the node along one path is automatically attached to the
/// node along every other path, which is dangerously untrue. If the user
/// actually wants to access the same witness data from multiple places
/// in the tree, they must write explicit logic which enforces that both
/// locations have the same data.
#[derive(Clone, Debug, Default)]
pub struct UnshareWitnessDisconnect<D: DagLike, T: SharingTracker<D>> {
    /// The underlying sharing tracker
    inner: T,
    /// Map from a node's pointer ID to whether it is a witness node
    ptrid_witness: HashMap<PointerId, bool>,
    /// Map from a node's index to whether it is a witness node (needed to
    /// determine whether a nodes' children are witnesses
    index_witness: HashMap<usize, bool>,
    phantom: marker::PhantomData<D>,
}

impl<D: DagLike, T: SharingTracker<D>> UnshareWitnessDisconnect<D, T> {
    /// Wrap an existing sharing tracker in the witness-unsharing tracker
    pub fn new(inner: T) -> Self {
        UnshareWitnessDisconnect {
            inner,
            ptrid_witness: HashMap::new(),
            index_witness: HashMap::new(),
            phantom: marker::PhantomData,
        }
    }
}

impl<D: DagLike, T: SharingTracker<D>> SharingTracker<D> for UnshareWitnessDisconnect<D, T> {
    fn record(
        &mut self,
        d: &D,
        index: usize,
        left_child: Option<usize>,
        right_child: Option<usize>,
    ) {
        self.inner.record(d, index, left_child, right_child);

        let mut is_witness = false;
        is_witness |= left_child
            .map(|idx| self.index_witness[&idx])
            .unwrap_or(false);
        is_witness |= right_child
            .map(|idx| self.index_witness[&idx])
            .unwrap_or(false);
        is_witness |= match d.as_dag_node() {
            Dag::Disconnect(..) => true,
            Dag::Witness => true,
            _ => false,
        };
        self.ptrid_witness.insert(PointerId::from(d), is_witness);
        self.index_witness.insert(index, is_witness);
    }

    fn seen_before(&self, d: &D) -> Option<usize> {
        match self.ptrid_witness.get(&PointerId::from(d)) {
            Some(true) => None,
            _ => self.inner.seen_before(d),
        }
    }
}

/// Object representing pointer identity of a DAG node
///
/// Used to populate a hashset to determine whether or not a given node has
/// already been tracker by an iterator.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct PointerId(usize);

impl<D: DagLike> From<&D> for PointerId {
    fn from(dag: &D) -> Self {
        PointerId(dag.data() as *const _ as usize)
    }
}

/// A trait for any structure which has the shape of a Simplicity DAG
///
/// This should be implemented on any reference type for `CommitNode` and `RedeemNode`;
/// it cannot be implemented on these structures themselves because they depend on
pub trait DagLike: Sized {
    /// The type of the DAG node, with no references or wrappers
    type Node;

    /// A pointer to the underlying data
    fn data(&self) -> &Self::Node;

    /// Interpret the node as a DAG node
    fn as_dag_node(&self) -> Dag<Self>;

    /// Accessor for the left child of the node, if one exists
    fn left_child(&self) -> Option<Self> {
        match self.as_dag_node() {
            Dag::Iden => None,
            Dag::Unit => None,
            Dag::InjL(sub) => Some(sub),
            Dag::InjR(sub) => Some(sub),
            Dag::Take(sub) => Some(sub),
            Dag::Drop(sub) => Some(sub),
            Dag::Comp(left, _) => Some(left),
            Dag::Case(left, _) => Some(left),
            Dag::AssertL(left) => Some(left),
            Dag::AssertR(right) => Some(right), // note that we treat the child of an assertR as a left child!
            Dag::Pair(left, _) => Some(left),
            Dag::Disconnect(left, _) => Some(left),
            Dag::Witness => None,
            Dag::Fail => None,
            Dag::Hidden => None,
            Dag::Jet => None,
            Dag::Word => None,
        }
    }

    /// Accessor for the right child of the node, if one exists
    fn right_child(&self) -> Option<Self> {
        match self.as_dag_node() {
            Dag::Iden => None,
            Dag::Unit => None,
            Dag::InjL(_) => None,
            Dag::InjR(_) => None,
            Dag::Take(_) => None,
            Dag::Drop(_) => None,
            Dag::Comp(_, right) => Some(right),
            Dag::AssertL(_) => None,
            Dag::AssertR(_) => None, // note that we treat the child of an assertR as a left child!
            Dag::Case(_, right) => Some(right),
            Dag::Pair(_, right) => Some(right),
            Dag::Disconnect(_, right) => Some(right),
            Dag::Witness => None,
            Dag::Fail => None,
            Dag::Hidden => None,
            Dag::Jet => None,
            Dag::Word => None,
        }
    }

    /// Obtains an iterator of all the nodes rooted at the DAG, in post order.
    ///
    /// Each node is only yielded once, at the leftmost position that it
    /// appears in the DAG.
    fn post_order_iter<S: SharingTracker<Self> + Default>(self) -> PostOrderIter<Self, S> {
        PostOrderIter {
            index: 0,
            stack: vec![IterStackItem::unprocessed(self, Previous::Root)],
            tracker: Default::default(),
        }
    }

    /// Obtains an post-order iterator of all the nodes rooted at DAG, using the
    /// given tracker.
    ///
    /// Ordinary users will never need to use this method; but it is available to
    /// enable obscure iteration use-cases.
    fn post_order_iter_with_tracker<S: SharingTracker<Self>>(
        self,
        tracker: S,
    ) -> PostOrderIter<Self, S> {
        PostOrderIter {
            index: 0,
            stack: vec![IterStackItem::unprocessed(self, Previous::Root)],
            tracker,
        }
    }
}

impl<'a, J: jet::Jet> DagLike for &'a CommitNode<J> {
    type Node = CommitNode<J>;

    fn data(&self) -> &CommitNode<J> {
        self
    }

    #[rustfmt::skip]
    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner() {
            CommitNodeInner::Iden => Dag::Iden,
            CommitNodeInner::Unit => Dag::Unit,
            CommitNodeInner::InjL(ref sub) => Dag::InjL(sub),
            CommitNodeInner::InjR(ref sub) => Dag::InjR(sub),
            CommitNodeInner::Take(ref sub) => Dag::Take(sub),
            CommitNodeInner::Drop(ref sub) => Dag::Drop(sub),
            CommitNodeInner::Comp(ref left, ref right) => Dag::Comp(left, right),
            CommitNodeInner::Case(ref left, ref right) => Dag::Case(left, right),
            CommitNodeInner::AssertL(ref left, _) => Dag::AssertL(left),
            CommitNodeInner::AssertR(_, ref right) => Dag::AssertR(right),
            CommitNodeInner::Pair(ref left, ref right) => Dag::Pair(left, right),
            CommitNodeInner::Disconnect(ref left, ref right) => Dag::Disconnect(left, right),
            CommitNodeInner::Witness => Dag::Witness,
            CommitNodeInner::Fail(..) => Dag::Fail,
            CommitNodeInner::Jet(..) => Dag::Jet,
            CommitNodeInner::Word(..) => Dag::Word,
        }
    }
}

impl<J: jet::Jet> DagLike for Rc<CommitNode<J>> {
    type Node = CommitNode<J>;

    fn data(&self) -> &CommitNode<J> {
        self
    }

    #[rustfmt::skip]
    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner() {
            CommitNodeInner::Iden => Dag::Iden,
            CommitNodeInner::Unit => Dag::Unit,
            CommitNodeInner::InjL(ref sub) => Dag::InjL(Rc::clone(sub)),
            CommitNodeInner::InjR(ref sub) => Dag::InjR(Rc::clone(sub)),
            CommitNodeInner::Take(ref sub) => Dag::Take(Rc::clone(sub)),
            CommitNodeInner::Drop(ref sub) => Dag::Drop(Rc::clone(sub)),
            CommitNodeInner::Comp(ref left, ref right) => Dag::Comp(Rc::clone(left), Rc::clone(right)),
            CommitNodeInner::Case(ref left, ref right) => Dag::Case(Rc::clone(left), Rc::clone(right)),
            CommitNodeInner::AssertL(ref left, _) => Dag::AssertL(Rc::clone(left)),
            CommitNodeInner::AssertR(_, ref right) => Dag::AssertR(Rc::clone(right)),
            CommitNodeInner::Pair(ref left, ref right) => Dag::Pair(Rc::clone(left), Rc::clone(right)),
            CommitNodeInner::Disconnect(ref left, ref right) => Dag::Disconnect(Rc::clone(left), Rc::clone(right)),
            CommitNodeInner::Witness => Dag::Witness,
            CommitNodeInner::Fail(..) => Dag::Fail,
            CommitNodeInner::Jet(..) => Dag::Jet,
            CommitNodeInner::Word(..) => Dag::Word,
        }
    }
}

impl<'a, J: jet::Jet> DagLike for &'a RedeemNode<J> {
    type Node = RedeemNode<J>;

    fn data(&self) -> &RedeemNode<J> {
        self
    }

    #[rustfmt::skip]
    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner {
            RedeemNodeInner::Iden => Dag::Iden,
            RedeemNodeInner::Unit => Dag::Unit,
            RedeemNodeInner::InjL(ref sub) => Dag::InjL(sub),
            RedeemNodeInner::InjR(ref sub) => Dag::InjR(sub),
            RedeemNodeInner::Take(ref sub) => Dag::Take(sub),
            RedeemNodeInner::Drop(ref sub) => Dag::Drop(sub),
            RedeemNodeInner::Comp(ref left, ref right) => Dag::Comp(left, right),
            RedeemNodeInner::Case(ref left, ref right) => Dag::Case(left, right),
            RedeemNodeInner::AssertL(ref left, _) => Dag::AssertL(left),
            RedeemNodeInner::AssertR(_, ref right) => Dag::AssertR(right),
            RedeemNodeInner::Pair(ref left, ref right) => Dag::Pair(left, right),
            RedeemNodeInner::Disconnect(ref left, ref right) => Dag::Disconnect(left, right),
            RedeemNodeInner::Witness(..) => Dag::Witness,
            RedeemNodeInner::Fail(..) => Dag::Fail,
            RedeemNodeInner::Jet(..) => Dag::Jet,
            RedeemNodeInner::Word(..) => Dag::Word,
        }
    }
}

impl<J: jet::Jet> DagLike for Rc<RedeemNode<J>> {
    type Node = RedeemNode<J>;

    fn data(&self) -> &RedeemNode<J> {
        self
    }

    #[rustfmt::skip]
    fn as_dag_node(&self) -> Dag<Self> {
        match self.inner {
            RedeemNodeInner::Iden => Dag::Iden,
            RedeemNodeInner::Unit => Dag::Unit,
            RedeemNodeInner::InjL(ref sub) => Dag::InjL(Rc::clone(sub)),
            RedeemNodeInner::InjR(ref sub) => Dag::InjR(Rc::clone(sub)),
            RedeemNodeInner::Take(ref sub) => Dag::Take(Rc::clone(sub)),
            RedeemNodeInner::Drop(ref sub) => Dag::Drop(Rc::clone(sub)),
            RedeemNodeInner::Comp(ref left, ref right) => Dag::Comp(Rc::clone(left), Rc::clone(right)),
            RedeemNodeInner::Case(ref left, ref right) => Dag::Case(Rc::clone(left), Rc::clone(right)),
            RedeemNodeInner::AssertL(ref left, _) => Dag::AssertL(Rc::clone(left)),
            RedeemNodeInner::AssertR(_, ref right) => Dag::AssertR(Rc::clone(right)),
            RedeemNodeInner::Pair(ref left, ref right) => Dag::Pair(Rc::clone(left), Rc::clone(right)),
            RedeemNodeInner::Disconnect(ref left, ref right) => Dag::Disconnect(Rc::clone(left), Rc::clone(right)),
            RedeemNodeInner::Witness(..) => Dag::Witness,
            RedeemNodeInner::Fail(..) => Dag::Fail,
            RedeemNodeInner::Jet(..) => Dag::Jet,
            RedeemNodeInner::Word(..) => Dag::Word,
        }
    }
}

enum Child<D: DagLike> {
    None,
    Repeat { idx: usize },
    New(D),
}

#[derive(Clone, Debug)]
enum Previous {
    /// This is the root element and there are no previous elements
    Root,
    /// This is a left child and the previous element is its parent
    ParentLeft,
    /// This is a left child and the previous element is its sibling
    SiblingLeft,
    /// This is a right child and the previous element is its parent
    ParentRight,
}

#[derive(Clone, Debug)]
struct IterStackItem<D: DagLike> {
    /// The element on the stack
    elem: D,
    /// Whether we have dealt with this item (and pushed its children,
    /// if any) yet.
    processed: bool,
    /// If the item has been processed, the index of its left child, if known
    left_idx: Option<usize>,
    /// If the item has been processed, the index of its right child, if known
    right_idx: Option<usize>,
    /// Whether the element is a left- or right-child of its parent
    previous: Previous,
}

impl<D: DagLike> IterStackItem<D> {
    /// Constructor for a new stack item with a given element and relationship
    /// to its parent.
    fn unprocessed(elem: D, previous: Previous) -> Self {
        IterStackItem {
            elem,
            processed: false,
            left_idx: None,
            right_idx: None,
            previous,
        }
    }

    fn left_child<V: SharingTracker<D>>(&self, tracker: &V) -> Child<D> {
        match self.elem.left_child() {
            Some(child) => match tracker.seen_before(&child) {
                Some(idx) => Child::Repeat { idx },
                None => Child::New(child),
            },
            None => Child::None,
        }
    }

    fn right_child<V: SharingTracker<D>>(&self, tracker: &V) -> Child<D> {
        match self.elem.right_child() {
            Some(child) => match tracker.seen_before(&child) {
                Some(idx) => Child::Repeat { idx },
                None => Child::New(child),
            },
            None => Child::None,
        }
    }
}

/// Iterates over a DAG in _post order_.
///
/// That means nodes are yielded in the order (left child, right child, parent).
/// Nodes may be repeated or not, based on the `S` parameter which defines how
/// the iterator treats sharing.
#[derive(Clone, Debug)]
pub struct PostOrderIter<D: DagLike, S: SharingTracker<D>> {
    /// The index of the next item to be yielded
    index: usize,
    /// A stack of elements to be yielded; each element is a node, then its left
    /// and right children (if they exist and if they have been yielded already)
    stack: Vec<IterStackItem<D>>,
    /// Data which tracks which nodes have been yielded already and therefore
    /// should be skipped.
    tracker: S,
}

/// A set of data yielded by a `PostOrderIter`
pub struct PostOrderIterItem<D: DagLike> {
    /// The actual node data
    pub node: D,
    /// The index of this node (equivalent to if you'd called `.enumerate()` on
    /// the iterator)
    pub index: usize,
    /// The index of this node's left child, if it has a left child
    pub left_index: Option<usize>,
    /// The index of this node's right child, if it has a left child
    pub right_index: Option<usize>,
}

impl<D: DagLike, S: SharingTracker<D>> Iterator for PostOrderIter<D, S> {
    type Item = PostOrderIterItem<D>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Look at the current top item on the stack
            if let Some(mut current) = self.stack.pop() {
                if !current.processed {
                    current.processed = true;
                    // When we first encounter an item, it is completely unknown; it is
                    // nominally the next item to be yielded, but it might have children,
                    // and if so, they come first
                    match (
                        current.left_child(&self.tracker),
                        current.right_child(&self.tracker),
                    ) {
                        // No children is easy, just mark it processed and iterate.
                        // (We match _ for the right child but we know that if the left one
                        // is Child::None, then the right one will be Child::None as well.)
                        (Child::None, _) => {
                            self.stack.push(current);
                        }
                        // Only a left child, already yielded
                        (Child::Repeat { idx }, Child::None) => {
                            current.left_idx = Some(idx);
                            self.stack.push(current);
                        }
                        // Only a left child, new
                        (Child::New(child), Child::None) => {
                            self.stack.push(current);
                            self.stack
                                .push(IterStackItem::unprocessed(child, Previous::ParentLeft));
                        }
                        // Two children, both already yielded
                        (Child::Repeat { idx: lidx }, Child::Repeat { idx: ridx }) => {
                            current.left_idx = Some(lidx);
                            current.right_idx = Some(ridx);
                            self.stack.push(current);
                        }
                        // Two children, one yielded already
                        (Child::New(child), Child::Repeat { idx }) => {
                            current.right_idx = Some(idx);
                            self.stack.push(current);
                            self.stack
                                .push(IterStackItem::unprocessed(child, Previous::ParentLeft));
                        }
                        (Child::Repeat { idx }, Child::New(child)) => {
                            current.left_idx = Some(idx);
                            self.stack.push(current);
                            self.stack
                                .push(IterStackItem::unprocessed(child, Previous::ParentRight));
                        }
                        // Two children, neither yielded already
                        (Child::New(lchild), Child::New(rchild)) => {
                            self.stack.push(current);
                            self.stack
                                .push(IterStackItem::unprocessed(rchild, Previous::ParentRight));
                            self.stack
                                .push(IterStackItem::unprocessed(lchild, Previous::SiblingLeft));
                        }
                    }
                } else {
                    // The next time we encounter an item, we have the indices for its children
                    // (which we ignore for now, but will use in a future commit) and can yield
                    // it. But first we need to update the indices of its parent.
                    let mut check_sibling = false;
                    let stack_len = self.stack.len();
                    match current.previous {
                        Previous::Root => {
                            assert_eq!(stack_len, 0);
                        }
                        Previous::ParentLeft => {
                            assert!(self.stack[stack_len - 1].processed);
                            self.stack[stack_len - 1].left_idx = Some(self.index);
                        }
                        Previous::ParentRight => {
                            assert!(self.stack[stack_len - 1].processed);
                            self.stack[stack_len - 1].right_idx = Some(self.index);
                        }
                        Previous::SiblingLeft => {
                            assert!(self.stack[stack_len - 2].processed);
                            self.stack[stack_len - 2].left_idx = Some(self.index);
                            check_sibling = true;
                        }
                    }
                    self.tracker.record(
                        &current.elem,
                        self.index,
                        current.left_idx,
                        current.right_idx,
                    );
                    // There is a special case here where we are yielding a node which
                    // is identical to its sibling (according to the sharing tracker).
                    // In this case we have a repeated node which nonetheless made it
                    // into the stack, so we pop it here.
                    if check_sibling
                        && self
                            .tracker
                            .seen_before(&self.stack[stack_len - 1].elem)
                            .is_some()
                    {
                        self.stack[stack_len - 2].right_idx = Some(self.index);
                        self.stack.pop();
                    }
                    self.index += 1;
                    return Some(PostOrderIterItem {
                        node: current.elem,
                        index: self.index - 1,
                        left_index: current.left_idx,
                        right_index: current.right_idx,
                    });
                }
            } else {
                // If there is nothing on the stack we are done.
                return None;
            }
        }
    }
}

impl<'a, J: jet::Jet, S: SharingTracker<&'a RedeemNode<J>> + Clone>
    PostOrderIter<&'a RedeemNode<J>, S>
{
    /// Adapt the iterator to only yield witnesses
    ///
    /// The witnesses are yielded in the order in which they appear in the DAG
    /// *except* that each witness is only yielded once, and future occurences
    /// are skipped.
    pub fn into_witnesses(self) -> impl Iterator<Item = &'a Value> + Clone {
        self.filter_map(|data| {
            if let RedeemNodeInner::Witness(value) = &data.node.inner {
                Some(value)
            } else {
                None
            }
        })
    }
}

impl<D: DagLike, S: SharingTracker<D>> PostOrderIter<D, S> {
    /// Display the DAG as an indexed list in post order.
    ///
    /// `display_body()` formats the node body in front of the node indices.
    /// `display_aux()` formats auxiliary items after the node indices.
    pub fn into_display<F, G>(
        self,
        f: &mut fmt::Formatter<'_>,
        mut display_body: F,
        mut display_aux: G,
    ) -> fmt::Result
    where
        F: FnMut(&D, &mut fmt::Formatter<'_>) -> fmt::Result,
        G: FnMut(&D, &mut fmt::Formatter<'_>) -> fmt::Result,
    {
        let mut node_to_index = HashMap::new();

        for (index, node) in self.map(|data| data.node).enumerate() {
            write!(f, "{}: ", index)?;
            display_body(&node, f)?;

            if let Some(left) = node.left_child() {
                let i_abs = node_to_index.get(&PointerId::from(&left)).unwrap();

                if let Some(right) = node.right_child() {
                    let j_abs = node_to_index.get(&PointerId::from(&right)).unwrap();

                    write!(f, "({}, {})", i_abs, j_abs)?;
                } else {
                    write!(f, "({})", i_abs)?;
                }
            }

            display_aux(&node, f)?;
            f.write_str("\n")?;
            node_to_index.insert(PointerId::from(&node), index);
        }

        Ok(())
    }
}