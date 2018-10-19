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
    type AbsState; // := Host(rep::domain::AbsState)

    /// The "bottom element" of the lattice of abstract states
    fn bottom : (Thk[0] 0 F AbsState) = {
        unsafe (0) trapdoor::bottom
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

// Representations for the program and its states' abstract domain
pub mod rep {
    /// Analysis context := program location (a unique number)
    pub type Ctx = usize;

    /// Analysis context set := vector of analysis contexts
    pub type Ctxs = Vec<Ctx>;

    /// Abstract domain for the analysis
    // TODO: Permit a compile-time flag to change the domain here:
    // (I kind of wish that Rust had functors here!)
    pub use crate::sem::dominator as domain;
    //pub use crate::sem::octagon as domain;
}

mod trapdoor {
    use super::rep;
    use fungi_lang::dynamics::{RtVal,ExpTerm,ret};
    use fungi_lang::hostobj::{rtval_of_obj};
        
    pub fn bottom(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 0);
        ret(rtval_of_obj( rep::domain::bottom() ))
    }
}

/// Example: Dominator analysis
///
/// https://en.wikipedia.org/wiki/Dominator_(graph_theory)
pub mod dominator {
    /// abstract state: a proposition (a logical formula) that
    /// summarizes how the program variables are related
    pub type AbsState = super::rep::Ctxs;

    /// Bottom element: No information about the state.
    pub fn bottom() -> AbsState {
        vec![]
    }
}

/// Example: Octagon analysis
///
/// https://arxiv.org/pdf/cs/0703084.pdf
pub mod octagon {
    use std::rc::Rc;

    /// abstract state: a proposition (a logical formula) that
    /// summarizes how the program variables are related
    pub type AbsState = Formula;

    /// Bottom element: No information about the state.
    pub fn bottom() -> AbsState {
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
