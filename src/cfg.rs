use crate::sem::rep::{Ctx,Ctxs,Preds,Stmt};

//
// Example program
//
// Represented as a control flow graph (CFG), whose nodes consist of
// unique IDs (each of type Ctx), and whose edges are each labeled
// with a program statement, of type Stmt.
//

pub fn all_ctxs () -> Ctxs {
    vec![ 0, 1, 2, 3, 4, 5 ]
}

fgi_mod!{
    open crate::sem;

    /// the entry point into the program to analyze
    fn entry_ctx : (Thk[0] 0 F Ctx) = {
        ret 1
    }

    /// all of the points of the program to analyze
    fn all_ctxs : (Thk[0] 0 F Ctxs) = {
        hostfn (0) {
            let ctxs : Ctxs = all_ctxs();
            fgi_rtval!( host ctxs )
        }
    }

    /// the immediate successors of the given analysis context
    fn ctx_succs : (Thk[0] 0 Ctx -> 0 F Ctxs) = {
        hostfn (1) {
            #(ctx:Ctx).
            let succs = match ctx {
                0 => vec![ 1 ],
                1 => vec![ 2, 3 ],
                2 => vec![ 4, 1 ],
                3 => vec![ 4 ],
                4 => vec![ 5 ],
                // all other nodes are undefined; they have no successors
                _ => vec![],
            };
            fgi_rtval!( host succs )
        }
    }

    /// the immediate successors of the given analysis context
    fn ctx_preds : (Thk[0] 0 Ctx -> 0 F Preds) = {
        hostfn (1) {
            #(ctx:Ctx).
            let preds : Preds = match ctx {
                1 => vec![ (0, Stmt::Set(0)),
                            (2, Stmt::Nop) ],
                2 => vec![ (1, Stmt::AssertEq(crate::sem::rep::Exp::Var("x".to_string()), crate::sem::rep::Exp::Num(0))) ],
                3 => vec![ (1, Stmt::AssertNeq(crate::sem::rep::Exp::Var("x".to_string()), crate::sem::rep::Exp::Num(0))) ],
                4 => vec![ (2, Stmt::Update("x".to_string(),
                                            crate::sem::rep::Exp::Plus(Rc::new(crate::sem::rep::Exp::Var("x".to_string())), Rc::new(crate::sem::rep::Exp::Num(1))) )),
                           (3, Stmt::Update("x".to_string(),
                                            crate::sem::rep::Exp::Plus(Rc::new(crate::sem::rep::Exp::Var("x".to_string())), Rc::new(crate::sem::rep::Exp::Num(2))) )) ],
                5 => vec![ (4, Stmt::Nop) ],
                // all other nodes are undefined; they have no successors
                _ => vec![],
            };
            fgi_rtval!( host preds )
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
