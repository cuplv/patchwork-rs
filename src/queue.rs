fgi_mod!{
    open crate::inv;

    /// Work queue representation (for work queue algorithm)
    /// ----------------------------------------------------
    type Queue;

    /// create the initial queue
    fn queue_init : (Thk[0] 0 F Queue)  = { unsafe (0) trapdoor::queue_init }
    
    /// pop the next location from the queue
    fn queue_pop : (Thk[0] 0 Queue -> 0 F (+ Unit + (x Queue x Loc))) = {
        unsafe (1) trapdoor::queue_pop
    }

    /// push the successors of a given location onto the queue
    fn queue_push_succs : (Thk[0] 0 Queue -> 0 Loc -> 0 F Queue) = {
        unsafe (2) trapdoor::queue_push_succs
    }
}


/// Trapdoor into Fungi's dynamic semantics.
/// 
/// This module defines operations on our new Patchwork-specific types
/// (work queues and invariant maps) by extending the Fungi
/// evaluator's semantics, but from within this crate (Patchwork).
///
pub mod trapdoor {
    use std::rc::Rc;
    use fungi_lang::dynamics::{RtVal,ExpTerm,ret};
    use fungi_lang::hostobj::{rtval_of_obj, obj_of_rtval};
    //use super::*;

    pub fn queue_init(_args:Vec<RtVal>) -> ExpTerm {
        ret(rtval_of_obj( () )) //TODO -- represent empty queue
    }
    
    pub fn queue_pop(args:Vec<RtVal>) -> ExpTerm {
        let q : () = obj_of_rtval( &args[0] ).unwrap();
        // TODO -- pop
        ret(RtVal::Inj1(Rc::new(rtval_of_obj( q ))))
    }

    pub fn queue_push_succs(args:Vec<RtVal>) -> ExpTerm {
        let q : () = obj_of_rtval( &args[0] ).unwrap();
        // TODO -- push
        ret(rtval_of_obj( q ))
    }
}
