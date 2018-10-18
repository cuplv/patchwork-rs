fgi_mod!{
    open crate::sem;

    /// Work queue representation (for work queue algorithm)
    /// ----------------------------------------------------
    type Queue; // := Host(trapdoor::Queue)

    /// empty queue
    fn queue_empty : (Thk[0] 0 F Queue)  = { unsafe (0) trapdoor::queue_empty }

    /// singleton queue
    fn queue_sing : (Thk[0] 0 Ctx -> 0 F Queue)  = { unsafe (1) trapdoor::queue_sing }
    
    /// pop the next analysis context from the queue
    fn queue_pop : (Thk[0] 0 Queue -> 0 F (+ Unit + (x Queue x Ctx))) = {
        unsafe (1) trapdoor::queue_pop
    }

    /// push the given analysis context onto the queue
    fn queue_push : (Thk[0] 0 Queue -> 0 Ctx -> 0 F Queue) = {
        unsafe (2) trapdoor::queue_push
    }

    /// push the given analysis context set onto the queue
    fn queue_push_all : (Thk[0] 0 Queue -> 0 Ctxs -> 0 F Queue) = {
        unsafe (2) trapdoor::queue_push_all
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
    use crate::sem::rep::{Ctx,Ctxs};
    //use super::*;

    pub type Queue = engine::Art<Vec<RtVal>>;

    pub fn queue_empty(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 0);
        let emp : Queue = engine::put( vec![] );
        ret(rtval_of_obj( emp ))
    }

    pub fn queue_sing(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 1);
        let sing : Queue = engine::put( vec![ args[0].clone() ] );
        ret(rtval_of_obj( sing ))
    }
    
    pub fn queue_pop(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 1);
        let q : Queue = obj_of_rtval( &args[0] ).unwrap();
        let mut vs : Vec<RtVal> = engine::force(&q);
        match vs.pop() {
            None    => ret(RtVal::Inj1(Rc::new(RtVal::Unit))),
            Some(v) => {
                let q : Queue = engine::put( vs );
                ret(RtVal::Inj2(Rc::new(
                    RtVal::Pair(
                        Rc::new(rtval_of_obj( q )),
                        Rc::new(v)
                    ))))
            }
        }
    }
    
    pub fn queue_push(args:Vec<RtVal>) -> ExpTerm {
        let q   : Queue = obj_of_rtval( &args[0] ).unwrap();
        let ctx : Ctx   = obj_of_rtval( &args[1] ).unwrap();        
        // TODO -- push
        drop(ctx);
        ret(rtval_of_obj( q ))
    }

    pub fn queue_push_all(args:Vec<RtVal>) -> ExpTerm {
        let q    : Queue = obj_of_rtval( &args[0] ).unwrap();
        let ctxs : Ctxs  = obj_of_rtval( &args[1] ).unwrap();
        // TODO -- push all
        drop(ctxs);
        ret(rtval_of_obj( q ))
    }
}
