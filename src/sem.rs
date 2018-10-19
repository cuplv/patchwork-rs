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
    type AbsState; // := Host(rep::AbsState)

    /// The "bottom element" of the lattice of abstract states
    fn bottom : (Thk[0] 0 F AbsState) = {
        unsafe (0) trapdoor::bottom
    }
}

/// Rust-based representations of the program to analyze, and its analysis state
pub mod rep {
    use std::rc::Rc;

    /// (abstract) program expressions
    #[derive(Clone,Debug,Eq,PartialEq,Hash)]
    pub enum Exp {
        /// Constant number
        Num(usize),
        /// (Program) variable
        Var(String),
        /// e1 + e2
        Plus(RecExp,RecExp),
    }
    pub type RecExp = Rc<Exp>;
    
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
    
    /// Analysis context := program location (a unique number)
    pub type Ctx = usize;

    /// Analysis context set := vector of program locations
    pub type Ctxs = Vec<usize>;

    /// abstract state: a proposition (a logical formula) that
    /// summarizes how the program variables are related
    pub type AbsState = Formula;

    /// Bottom element: No information about the state.
    pub fn bottom() -> AbsState {
        Formula::Tt 
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

mod trapdoor {
    use fungi_lang::dynamics::{RtVal,ExpTerm,ret};
    use fungi_lang::hostobj::{rtval_of_obj, 
                              //obj_of_rtval
    };
    use crate::sem::rep;
    
    pub fn bottom(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 0);
        ret(rtval_of_obj( rep::bottom() ))
    }
}
