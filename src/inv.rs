fgi_mod!{
    open crate::sem;

    /// Invariant map representation
    /// ----------------------------

    /// Rust-based representation of a finite map.
    /// (eventually, use finer-grained representation of map updates).
    /// The finite map associates each distinct analysis context with an abstract state.
    type Map;
    
    /// Invariant map; the refinement type tracks the set of named update operations
    type Inv = (foralli (X):NmSet. Map);
    
    /// create the initial invariant map; no updates yet; names := empty set
    fn inv_init : (Thk[0] 0 F Inv[0]) = { unsafe (0) trapdoor::inv_init }
    
    /// update the abstract state at a particular context in the invariant map
    fn inv_update : (
        Thk[0] foralli (X,Y,XY):NmSet | ((X%Y)=XY:NmSet).
            0 Inv[X] -> 0 Nm[Y] -> 0 Ctx -> 0 AbsState -> 0 F Inv[X%Y]
    ) = { 
        unsafe (4) trapdoor::inv_update
    }

    /// project a particular context's abstract state from the invariant map
    fn inv_get : (
        Thk[0] foralli (X):NmSet.
            0 Inv[X] -> 0 Ctx -> 0 F AbsState
    ) = {
        unsafe (2) trapdoor::inv_get
    }

}


/*  Try this:
 *  $ cargo inv::typing 2>&1 | less -R
 *
 */    
#[test]
pub fn typing() { fgi_listing_test!{
    open crate::inv;
    ret 0
}}

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

