fgi_mod!{
    open crate::sem;

    /// the entry points into the program to analyze
    fn entry_ctxs : (Thk[0] 0 F Ctxs) = {
        unsafe (0) trapdoor::entry_ctxs
    }

    /// all of the points of the program to analyze
    fn all_ctxs : (Thk[0] 0 F Ctxs) = {
        unsafe (0) trapdoor::all_ctxs
    }
    
    /// the immediate successors of the given analysis context
    fn ctx_succs : (Thk[0] 0 Ctx -> 0 F Ctxs) = {
        unsafe (1) trapdoor::ctx_succs
    }

    /// the immediate successors of the given analysis context
    fn ctx_preds : (Thk[0] 0 Ctx -> 0 F Ctxs) = {
        unsafe (1) trapdoor::ctx_preds
    }
}

pub mod example {
    use crate::sem::rep::{Ctx,Ctxs,Preds,Stmt};

    pub fn entry_ctxs() -> Ctxs { 
        vec![ 1 ]
    }

    pub fn all_ctxs() -> Ctxs { 
        vec![ 1, 2, 3, 4, 5 ]
    }
    
    pub fn succs(ctx:Ctx) -> Ctxs {
        match ctx {
            1 => vec![ 2, 3 ],
            2 => vec![ 4 ],
            3 => vec![ 4 ],
            4 => vec![ 1, 5 ],
            // all other nodes are undefined; they have no successors
            _ => vec![],
        }
    }
    
    pub fn preds(ctx:Ctx) -> Preds {
        match ctx {
            1 => vec![  ],
            2 => vec![ (1, Stmt::Nop) ],
            3 => vec![ (1, Stmt::Nop) ],
            4 => vec![ (2, Stmt::Nop), 
                        (3, Stmt::Nop) ],
            5 => vec![ (4, Stmt::Nop) ],
            // all other nodes are undefined; they have no successors
            _ => vec![],
        }
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
    use crate::sem::rep::{Ctx,Ctxs,Preds,Stmt};

    pub fn entry_ctxs(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 0);
        ret(rtval_of_obj( super::example::entry_ctxs() ))
    }

    pub fn all_ctxs(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 0);
        ret(rtval_of_obj( super::example::all_ctxs() ))
    }

    pub fn ctx_succs(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 1);
        let ctx : Ctx = obj_of_rtval( &args[0] ).unwrap();
        ret(rtval_of_obj( super::example::succs( ctx ) ))
    }
    
    pub fn ctx_preds(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 1);
        let ctx : Ctx = obj_of_rtval( &args[0] ).unwrap();
        ret(rtval_of_obj( super::example::preds( ctx ) ))
    }
}
