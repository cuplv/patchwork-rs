fgi_mod!{
    open crate::sem;

    /// the entry points into the program to analyze
    fn entry_ctxs : (Thk[0] 0 F Ctxs) = {
        unsafe (0) trapdoor::entry_ctxs
    }
    
    /// the immediate successors of the given analysis context
    fn ctx_succs : (Thk[0] 0 Ctx -> 0 F Ctxs) = {
        unsafe (1) trapdoor::ctx_succs
    }
}


/*  Try this:
 *  $ cargo test cfg::typing 2>&1 | less -R
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

    pub fn entry_ctxs(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 0);
        // TODO: consult the control flow graph that we got from somewhere external
        // 
        // For now, this is hard-coded:
        let entry_ctxs : Ctxs = vec![ 
            1, 6, 31 
        ];
        ret(rtval_of_obj( entry_ctxs ))
    }

    pub fn ctx_succs(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 1);
        let ctx : Ctx = obj_of_rtval( &args[0] ).unwrap();
        // TODO: consult the control flow graph that we got from somewhere external
        //
        // For now, this is hard-coded:
        let succs : Ctxs = match ctx {
            1 => vec![ 2, 3 ],
            2 => vec![ 4 ],
            3 => vec![ 4 ],
            4 => vec![ 1, 5 ],
            // all other nodes are undefined; they have no successors
            _ => vec![],
        };
        ret(rtval_of_obj( succs ))
    }
}
