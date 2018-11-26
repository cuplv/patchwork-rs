use crate::sem::domain::{AbsState};
use crate::sem::rep::{Stmt,Ctx};

fgi_mod!{
    /// Program representation and semantics
    /// -------------------------------------
    ///
    /// Rust-based representations of program analysis contexts and
    /// their associated abstract states.

    /// Analysis context, e.g., a program location
    type Ctx; // := Host(rep::Ctx)

    /// Analysis context set (or list, or collection, etc)
    type Ctxs; // := Host(rep::Ctxs)

    /// Abstract state, e.g., an invariant that is (locally) true
    type Stmt; // := Host(rep::Stmt)

    /// A set of successors consists of a statement for each possible target context
    type Succs; // := Host(rep::Succs);

    /// A set of predecessors consists of a statement for each possible source context
    type Preds; // := Host(rep::Preds);

    /// Abstract state, e.g., an invariant that is (locally) true
    type AbsState; // := Host(domain::AbsState)

    /// The "bottom element" of the lattice of abstract states
    fn bottom : (Thk[0] 0 Ctx -> 0 F AbsState) = {
        hostfn (1) {
            #(ctx : Ctx).
            let b : AbsState =
                crate::sem::domain::bottom( &ctx );
            fgi_rtval!( host b )
        }
    }

    /// Test two abstract states for equality
    fn domain_eq : (
        Thk[0]
            0 AbsState ->
            0 AbsState ->
            0 F Bool) =
    {
        hostfn (2) {
            #(s1 : AbsState).
            #(s2 : AbsState).
            let b : bool = s1 == s2;
            fgi_rtval!( bool b )
        }
    }

    /// The "transfer function" of the program semantics/analysis
    fn transfer : (
        Thk[0]
            0 AbsState -> // join of all predecessors' states
            0 Ctx ->      // context
            0 Stmt ->     // ????
            0 F AbsState) = {
        hostfn (3) {
            #(pre  : AbsState).
            #(ctx  : Ctx).
            #(stmt : Stmt).
            let s : AbsState = crate::sem::domain::transfer( pre, &ctx, &stmt );
            fgi_rtval!( host s )
        }
    }
}

/*  Try this:
 *  $ cargo sem::typing 2>&1 | less -R
 *
 */
#[test]
pub fn typing() { fgi_listing_test!{
    open crate::sem;
    ret 0
}}

/// Representations for the program and its states' abstract domain.
///
/// TODO: Finish this; use an existing (completed) implementation of
/// the program expressions and statements.  The one below is a
/// temporary sketch.
pub mod rep {
    use std::rc::Rc;

    /// Analysis context := program location (a unique number)
    pub type Ctx = usize;
    pub type Ctxs = Vec<usize>;

    /// A set of successors consists of a statement for each possible target context
    pub type Succs = Vec<(Stmt,Ctx)>;

    /// A set of predecessors consists of a statement for each possible source context
    pub type Preds = Vec<(Ctx, Stmt)>;

    /// (concrete) program expressions
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum Exp {
        /// Constant number
        Num(usize),
        /// (Program) variable
        Var(String),
        /// -(e)
        Neg(RecExp),
        /// e1 + e2
        Plus(RecExp,RecExp),
    }
    pub type RecExp = Rc<Exp>;

    /// Statements: Imperative commands that label the edges of the
    /// control flow graph.
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum Stmt {
        /// The "identity effect"
        Nop,
        /// Conditional control flow: "Eq" Case
        AssertEq(Exp, Exp),
        /// Conditional control flow: "Neq" Case
        AssertNeq(Exp, Exp),
        /// Imperative variable update
        Update(String, Exp),
        //adding Incr and Decr to test interval
        Incr,
        Decr,
        Set(usize),
    }
}

/// Abstract domain for the analysis
// TODO: Permit a compile-time flag to change the domain here:
// (I kind of wish that Rust had functors here!)
//
// ** Choose one of the following:
//
// pub use crate::sem::octagon as domain;
// pub use crate::sem::dominator as domain;
pub use crate::sem::interval as domain;
//

/// Example: Dominator analysis
///
/// https://en.wikipedia.org/wiki/Dominator_(graph_theory)
pub mod dominator {
    use super::rep::{Stmt,Ctx,Ctxs};

    /// abstract state: a set of dominating program locations ("contexts")
    pub type AbsState = Ctxs;

    pub fn bottom(ctx:&Ctx) -> AbsState {
        // Bottom element: No information about the dominators,
        // except at the entry nodes.
        /*
        if ctx == &1 {
            vec![ 1 ]
        } else {
            crate::cfg::all_ctxs()
        }
        */
        vec![ ]
    }

    pub fn join(s1:AbsState, s2:AbsState) -> AbsState {
        // Join predecessors' abstract states (dominators) by doing
        // set intersection
        intersect(&s1, &s2)
    }

    pub fn transfer(pre:AbsState, ctx:&Ctx, _stmt:&Stmt) -> AbsState {
        // For dominator analysis, the statment does not matter; there
        // is no data flow analysis here.
        union( pre , vec![ ctx.clone() ] )
    }

    fn union(s1:AbsState, s2:AbsState) -> AbsState {
        let mut s1 = s1;
        let mut s2 = s2;
        s1.append(&mut s2);
        s1
    }

    fn intersect(s1:&AbsState, s2:&AbsState) -> AbsState {
        let mut res = vec![];
        for x in s1.iter() {
            for y in s2.iter() {
                if x == y { res.push(x.clone()); break }
            }
        }
        res
    }
}

pub mod interval {
    use super::rep::{Stmt, Ctx};

    pub type AbsState = Interval;

    #[derive(Clone,Hash,Debug)]
    pub enum Interval {
        //bottom element
        Bottom,
        //interval with both sides infty
        IIInt,
        //interval with 2 ints
        RRInt(usize, usize),
        //interval with low side infty
        IRInt(usize),
        //interval with high side infty
        RIInt(usize)
    }
    impl PartialEq for Interval {
        fn eq(&self, other: &Interval) -> bool {
            match (self, other) {
                (Interval::Bottom, Interval::Bottom) => true,
                (Interval::IIInt, Interval::IIInt) => true,
                (Interval::IRInt(c1), Interval::IRInt(c2)) => c1 == c2,
                (Interval::RIInt(c1), Interval::RIInt(c2)) => c1 == c2,
                (Interval::RRInt(l1, h1), Interval::RRInt(l2, h2)) => l1 == h1 && l2 == h2,
                (_, _) => false
            }
        }
    }
    impl Eq for Interval {}

    pub fn bottom(_ctx : &super::rep::Ctx) -> AbsState {
        Interval::Bottom
    }

    //application of widening operator assumes s1 is first AbsState and s2 second
    //this widening impl is more writing b/c i implemented infty via enum rather than usize.max (or something)
    //potentially this is not optimal
    pub fn join(s1:AbsState, s2:AbsState) -> AbsState {
        match (s1.clone(), s2.clone()) {
            (Interval::Bottom, _) => s2,
            (_, Interval::Bottom) => s1,
            (Interval::IIInt, _) | (_, Interval::IIInt) => Interval::IIInt,
            //one-sided opposite-sided inftys
            (Interval::IRInt(_), Interval::RIInt(_)) | (Interval::RIInt(_), Interval::IRInt(_)) => Interval::IIInt,
            //widening below
            //one-sided same-sided inftys
            (Interval::IRInt(h1), Interval::IRInt(h2)) => if h2 > h1 { Interval::IIInt } else { Interval::IRInt(h1) },
            (Interval::RIInt(l1), Interval::RIInt(l2)) => if l2 < l1 { Interval::IIInt } else { Interval::RIInt(l1) },
            //low-sided infty with reg intervals
            (Interval::IRInt(h1), Interval::RRInt(_l2, h2)) => if h2 > h1 { Interval::IIInt } else { Interval::IRInt(h1) },
            (Interval::RRInt(_l1, h1), Interval::IRInt(h2)) => if h2 > h1 { Interval::IIInt } else { Interval::IRInt(h1) },
            //high-sided infty with reg intervals
            (Interval::RIInt(l1), Interval::RRInt(l2, _h2)) => if l2 < l1 { Interval::IIInt } else { Interval::RIInt(l1) },
            (Interval::RRInt(l1, _h1), Interval::RIInt(l2)) => if l2 < l1 { Interval::IIInt } else { Interval::RIInt(l1) },
            //two reg intervals
            (Interval::RRInt(l1, h1), Interval::RRInt(l2, h2)) =>
                if l2 < l1 {
                    if h2 > h1 {
                        Interval::IIInt
                    } else {
                        Interval::IRInt(h1)
                    }
                } else {
                    if h2 > h1 {
                        Interval::RIInt(l1)
                    } else {
                        Interval::RRInt(l1, h1)
                    }
                }
        }
    }

    //TODO: is it safe to assume here that the AbsState is relevant to the stmt?
    //should AbsState actually be a map from variables to intervals?
    pub fn transfer(pre:AbsState, _ctx:&Ctx, stmt:&Stmt) -> AbsState {
        match (pre.clone(), stmt.clone()) {
            (_, Stmt::Nop) | (_, Stmt::AssertEq(_, _)) |
            (_, Stmt::AssertNeq(_, _)) | (_, Stmt::Update(_, _)) => pre,
            (_, Stmt::Set(n)) => Interval::RRInt(n, n),
            //should bottom be pre? I think it should
            (Interval::IIInt, _ ) | (Interval::Bottom, _) => pre,

            (Interval::IRInt(c), Stmt::Incr) => Interval::IRInt(c+1),
            (Interval::RIInt(c), Stmt::Incr) => Interval::RIInt(c+1),
            (Interval::RRInt(l,h), Stmt::Incr) => Interval::RRInt(l+1, h+1),

            (Interval::IRInt(c), Stmt::Decr) => Interval::IRInt(c-1),
            (Interval::RIInt(c), Stmt::Decr) => Interval::RIInt(c-1),
            (Interval::RRInt(l,h), Stmt::Decr) => Interval::RRInt(l-1, h-1),
        }
    }
}

/// Example: Octagon analysis
///
/// https://arxiv.org/pdf/cs/0703084.pdf
///
/// TODO: Finish this; use an existing (completed) implementation of
/// this abstract domain.  The one below is a temporary sketch.
///
pub mod octagon {
    use std::rc::Rc;

    /// abstract state: a proposition (a logical formula) that
    /// summarizes how the program variables are related
    pub type AbsState = Formula;

    /// Bottom element: No information about the state.
    pub fn bottom(_ctx:super::rep::Ctx) -> AbsState {
        Formula::Tt
    }

    /// formula: propositions in the ambient logic that talk about program
    /// expressions.
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum Formula {
        /// Tautology
        Tt,
        /// Conjunction
        Conj(RecFormula, RecFormula),
        /// e1 <= e2
        Lte(Exp,Exp)
    }
    pub type RecFormula = Rc<Formula>;

    /// (abstract) program expressions
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum Exp {
        /// Constant number
        Num(usize),
        /// (Program) variable
        Var(String),
        /// -(e)
        Neg(RecExp),
        /// e1 + e2
        Plus(RecExp,RecExp),
    }
    pub type RecExp = Rc<Exp>;
}
