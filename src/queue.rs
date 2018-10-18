fgi_mod!{
    open crate::sem;

    /// Work queue representation (for work queue algorithm)
    /// ----------------------------------------------------
    type Queue; // := Host(trapdoor::Queue)

    /// create the initial queue
    fn queue_init : (Thk[0] 0 F Queue)  = { unsafe (0) trapdoor::queue_init }
    
    /// pop the next analysis context from the queue
    fn queue_pop : (Thk[0] 0 Queue -> 0 F (+ Unit + (x Queue x Ctx))) = {
        unsafe (1) trapdoor::queue_pop
    }

    /// push the successors of a given analysis context onto the queue
    fn queue_push_succs : (Thk[0] 0 Queue -> 0 Ctx -> 0 F Queue) = {
        unsafe (2) trapdoor::queue_push_succs
    }
}

/*  Try this:
 *  $ cargo queue::typing 2>&1 | less -R
 *
 */    
#[test]
pub fn typing() { fgi_listing_test!{
    open crate::queue;
    ret 0
}}


/// Trapdoor into Fungi's dynamic semantics.
/// 
/// This module defines operations on our new Patchwork-specific types
/// (work queues and invariant maps) by extending the Fungi
/// evaluator's semantics, but from within this crate (Patchwork).
///
pub mod trapdoor {
    use std::rc::Rc;
    use adapton::engine;
    use fungi_lang::dynamics::{RtVal,ExpTerm,ret};
    use fungi_lang::hostobj::{rtval_of_obj, obj_of_rtval};
    //use super::*;

    pub type Queue = engine::Art<Vec<RtVal>>;

    pub fn queue_init(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 0);
        let empty : Queue = engine::put( vec![] );
        ret(rtval_of_obj( empty ))
    }
    
    pub fn queue_pop(args:Vec<RtVal>) -> ExpTerm {
        let q : Queue = obj_of_rtval( &args[0] ).unwrap();
        // TODO -- pop
        ret(RtVal::Inj1(Rc::new(rtval_of_obj( q ))))
    }

    pub fn queue_push_succs(args:Vec<RtVal>) -> ExpTerm {
        let q : Queue = obj_of_rtval( &args[0] ).unwrap();
        // TODO -- push
        ret(rtval_of_obj( q ))
    }
}
