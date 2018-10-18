fgi_mod!{
    open crate::sem;
    
    /// the immediate successors of the given analysis context
    fn ctx_succs : (Thk[0] 0 Ctx -> 0 F Ctxs) = {
        unsafe (0) trapdoor::ctx_succs
    }
}


/*  Try this:
 *  $ cargo cfg::typing 2>&1 | less -R
 *
 */    
#[test]
pub fn typing() { fgi_listing_test!{
    open crate::cfg;
    ret 0
}}


pub mod trapdoor {
    //use std::rc::Rc;
    //use adapton::engine;
    use fungi_lang::dynamics::{RtVal,ExpTerm,ret};
    use fungi_lang::hostobj::{rtval_of_obj, obj_of_rtval};
    //use super::*;
    use crate::sem::rep::{Ctx,Ctxs};

    pub fn ctx_succs(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 1);
        let ctx : Ctx = obj_of_rtval( &args[0] ).unwrap();
        // TODO: consult the control flow graph that we got from somewhere external
        drop(ctx);
        let emp : Ctxs = vec![] ;
        ret(rtval_of_obj( emp ))
    }
}
