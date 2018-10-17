
// Rust-based representations of the program to analyze, and its analysis state
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
    
    /// program location
    pub type Loc = usize;

    /// abstract state: local to a program location, sometime during
    /// the program analysis.
    pub type AbsState = Formula;
}

fgi_mod!{
    /// Invariant map representation
    /// ----------------------------

    /// Rust-based representations of program locations and abstract
    /// states.
    type Loc; // := Host(rep::Loc)
    type AbsState; // := Host(rep::AbsState)

    /// Rust-based representation of a finite map.
    /// (eventually, use finer-grained representation of map updates).
    /// The finite map associates each program location with an abstract state.
    type Map;
    
    /// Invariant map; the type tracks the set of named update operations
    type Inv = (foralli (X):NmSet. Map);
    
    /// create the initial invariant map; no updates yet; names := empty set
    fn inv_init : (Thk[0] 0 F Inv[0]) = { unsafe (0) trapdoor::inv_init }
    
    /// update the abstract state at a particular location in the invariant map
    fn inv_update : (
        Thk[0] foralli (X,Y):NmSet | ((X%Y):NmSet).
            0 Inv[X] -> 0 Nm[Y] -> 0 Loc -> 0 AbsState -> 0 F Inv[X%Y]
    ) = { 
        unsafe (4) trapdoor::inv_update
    }

    /// project a particular location's abstract state from the invariant map
    fn inv_get : (
        Thk[0] foralli (X):NmSet.
            0 Inv[X] -> 0 Loc -> 0 F AbsState
    ) = {
        unsafe (2) trapdoor::inv_get
    }

}

/// Trapdoor into Fungi's dynamic semantics.
/// 
/// This module defines operations on our new Patchwork-specific types
/// (work queues and invariant maps) by extending the Fungi
/// evaluator's semantics, but from within this crate (Patchwork).
///
pub mod trapdoor {
    use fungi_lang::dynamics::{RtVal,ExpTerm,ret};
    use fungi_lang::hostobj::{rtval_of_obj, obj_of_rtval};
    //use super::*;

    pub fn inv_init(_args:Vec<RtVal>) -> ExpTerm {
        ret(rtval_of_obj( () )) //TODO -- represent empty mapping
    }

    pub fn inv_get(args:Vec<RtVal>) -> ExpTerm {
        let inv : () = obj_of_rtval( &args[0] ).unwrap();
        let loc : () = obj_of_rtval( &args[1] ).unwrap();
        // TODO -- project from the mapping
        drop((inv,loc));
        ret(rtval_of_obj( () ))
    }
    
    pub fn inv_update(args:Vec<RtVal>) -> ExpTerm {
        let inv : () = obj_of_rtval( &args[0] ).unwrap();
        let nm  : () = obj_of_rtval( &args[1] ).unwrap();
        let loc : () = obj_of_rtval( &args[2] ).unwrap();
        let st  : () = obj_of_rtval( &args[3] ).unwrap();
        // TODO -- update representation of the mapping
        drop((inv,nm,loc,st));
        ret(rtval_of_obj( () ))
    }
}

