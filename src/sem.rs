fgi_mod!{
    /// Program representation and semantics
    /// -------------------------------------
    ///
    /// Rust-based representations of program analysis contexts and
    /// their associated abstract states.

    /// Analysis context, e.g., a program location
    type Ctx; // := Host(rep::Ctx)

    /// Abstract state, e.g., an invariant that is (locally) true
    type AbsState; // := Host(rep::AbsState)
}

/// Rust-based representations of the program to analyze, and its analysis state
pub mod rep {
    use std::rc::Rc;

    /// (abstract) program expressions
    pub enum Exp {
        Num(usize),
        Var(String),
        Plus(RecExp,RecExp),
    }
    pub type RecExp = Rc<Exp>;
    
    /// formula: propositions in the ambient logic that talk about program
    /// expressions.
    pub enum Formula {
        Tt, Conj(RecFormula, RecFormula),
        Ineq(Exp,Exp)
    }
    pub type RecFormula = Rc<Formula>;
    
    /// Analysis context := program location (a unique number)
    pub type Ctx = usize;

    /// abstract state: a proposition (a logical formula) that
    /// summarizes how the program variables are related
    pub type AbsState = Formula;
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

