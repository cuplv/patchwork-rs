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

    /// A set of successors consists of a statement for each possible target context
    type Succs; // := Host(rep::Succs);
    
    /// A set of predecessors consists of a statement for each possible source context
    type Preds; // := Host(rep::Preds);

    /// Abstract state, e.g., an invariant that is (locally) true
    type AbsState; // := Host(rep::domain::AbsState)

    /// The "bottom element" of the lattice of abstract states
    fn bottom : (Thk[0] 0 Ctx -> 0 F AbsState) = {
        unsafe (1) trapdoor::bottom
    }

    /// Test two abstract states for equality
    fn domain_eq : (
        Thk[0] 
            0 AbsState -> 
            0 AbsState -> 
            0 F Bool) = 
    {
        unsafe (2) trapdoor::domain_eq
    }

    /// The "transfer function" of the program semantics/analysis
    fn transfer : (
        Thk[0] 
            0 AbsState -> 
            0 Ctx -> 
            0 Stmt -> 
            0 F AbsState) = {
        unsafe (3) trapdoor::transfer
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
    }

    /// Abstract domain for the analysis
    // TODO: Permit a compile-time flag to change the domain here:
    // (I kind of wish that Rust had functors here!)
    pub use crate::sem::dominator as domain;
    //pub use crate::sem::octagon as domain;
}

mod trapdoor {
    use super::rep;
    use fungi_lang::dynamics::{RtVal,ExpTerm,ret};
    use fungi_lang::hostobj::{rtval_of_obj, obj_of_rtval};
    use super::rep::{Ctx,Stmt,domain::{AbsState}};
        
    pub fn bottom(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 1);
        let ctx : Ctx = obj_of_rtval( &args[0] ).unwrap();
        ret(rtval_of_obj( rep::domain::bottom( &ctx ) ))
    }

    pub fn transfer(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 3);
        let pre  : AbsState = obj_of_rtval( &args[0] ).unwrap();
        let ctx  : Ctx      = obj_of_rtval( &args[1] ).unwrap();
        let stmt : Stmt     = obj_of_rtval( &args[2] ).unwrap();
        ret(rtval_of_obj( rep::domain::transfer( pre, &ctx, &stmt ) ))
    }

    pub fn domain_eq(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 2);
        let s1  : AbsState = obj_of_rtval( &args[0] ).unwrap();
        let s2  : AbsState = obj_of_rtval( &args[1] ).unwrap();
        ret(RtVal::Bool( s1 == s2 ))
    }
}

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
        if ctx == &1 { 
            vec![ 1 ]
        } else {
            crate::cfg::example::all_ctxs()
        }
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

/// Example: Octagon analysis
///
/// https://arxiv.org/pdf/cs/0703084.pdf
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
