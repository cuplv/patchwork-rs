fgi_mod!{
    /// Program semantics and representation
    open crate::sem;

    /// define CFG relations between program contexts: successors, predecessors
    open crate::cfg;

    /// Invariant map representation
    open crate::inv;
    
    /// Work queue representation
    open crate::queue;

    /// Transfer function (of the program analysis)
    /// -------------------------------------------

    /// The result of visiting the next analysis context:
    /// Either: no change, or an updated invariant map, with a new name.
    type VisitRes = (foralli (X1, X2):NmSet. 
                     (+ Unit 
                      + (x Inv[X1 % X2] 
                         x Nm[X2])
                     ));
    
    // TODO: This type should return an Inv with an existentially-bound set of names
    fn visit_ctx : (
        Thk[0] foralli (X1):NmSet.
            0 Inv[X1] -> 
            0 Ctx -> 
        // TODO: instead of the constant @1, use 
        //       `exists (X2):NmSet | (X1 % X2).`
            0 F VisitRes[X1][{@1}]
    ) = {
        #inv.#ctx.
        let preds = {{force ctx_preds} ctx}
        let join  = {{force inv_join}[X1] inv preds ctx}
        let s1    = {{force inv_get}[X1] inv ctx}
        let test  = {{force domain_eq} s1 join}
        if ( test ) {
            ret inj1 ()
        } else {
            // TODO: Choose a name somehow; do the (named) update.
            // (e.g., the name could be '(ctx,join)' --- a distinct name not already in X1).
            let nm = {ret (name @1)}
            let inv = {
                {force inv_update}[X1][{@1}][X1%{@1}] 
                    inv nm ctx join
            }
            ret inj2 (inv, nm)
        }
    }

    /// Fixed-point computation, via a work list algorithm
    /// --------------------------------------------------
       
    // TODO: This type should return an Inv with an existentially-bound set of names
    fn do_work_queue : (
        Thk[0] foralli (X):NmSet.
            0 Inv[X] ->
            0 Queue ->
        { {@!}({@1})    // TODO: Existential write set
        ; {@!}({@1}) }  // TODO: Existential read set
            F Inv[X % {@1}]
    ) = {
        #inv.#q.
        let m = {{force queue_pop} q}
        match m {
            /* None */ _u => {ret inv}
            /* Some */ q_ctx => { 
                let (q, ctx) = {ret q_ctx}
                let m = {{force visit_ctx}[X] inv ctx}
                match m {
                    /* NoChange */ _u  => {{force do_work_queue}[X] inv q}
                    /* Changed: New invariant name, with new name nm */  inv_nm => {
                        let (inv, nm) = {ret inv_nm}
                        let ctxs = {{force ctx_succs} ctx}
                        let q    = {{force queue_push_all} q ctxs}
                        let (_r, r) = {memo (nm) {{force do_work_queue}[X % {@1}] inv q}}
                        ret r
                    }
                }
            }            
        }
    }

    // TODO: This return type needs an existential quantifier
    fn run : (
        Thk[0] 
        { {@!}({@1}) 
        ; {@!}({@1}) }
        F Inv[ {@1} ]) = 
    {
        let inv  = {{force inv_init}}
        //let ctxs = {{force entry_ctxs}}
        let ctxs = {{force all_ctxs}}
        let q    = {{force queue_empty}}
        let q    = {{force queue_push_all} q ctxs}
        let res  = {{force do_work_queue} [0] inv q}
        ret res
    }
}


/*  Try this:
 *  $ cargo test fix::typing 2>&1 | less -R
 *
 */    
#[test]
pub fn typing() { fgi_listing_test!{
    open crate::fix;
    {force run}
}}

pub fn run() { fgi_dynamic_trace!{[Expect::SuccessXXX]
    open crate::fix;
    {force run}
}}

/*  Try this:
 *  $ export FUNGI_VERBOSE_REDUCE=1
 *  $ cargo test test::reduction -- --nocapture | less -R
 *
 */
#[test]
pub fn reduction() { run() }
