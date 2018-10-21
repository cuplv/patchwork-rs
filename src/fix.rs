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
    /// Either: no change, or an updated invariant map
    type VisitRes = (foralli (X):NmSet. (+ Unit + Inv[X]));

    // TODO: This type should return an Inv with an existentially-bound set of names
    fn visit_ctx : (Thk[0] foralli (X):NmSet.
                    0 Inv[X] -> 
                    0 Ctx -> 
                    0 F VisitRes[X]) = {
        #inv.#ctx.
        let s1    = {{force inv_get}[X] inv ctx}
        let preds = {{force ctx_preds} ctx}
        let join  = {{force inv_join}[X] inv preds ctx}
        let test  = {{force domain_eq} s1 join}
        if ( test ) {
            ret inj1 ()
        } else {
            // TODO: Choose a name somehow; do the (named) update.
            let inv = {{force inv_update}[X][{@1}][X%{@1}] 
                       inv (name @1) ctx join}
            ret inj2 inv
        }
    }

    /// Fixed-point computation, via a work list algorithm
    /// --------------------------------------------------
       
    // TODO: This type should return an Inv with an existentially-bound set of names
    fn do_work_queue : (Thk[0] foralli (X):NmSet.
                      0 Inv[X] ->
                      0 Queue ->
                      0 F Inv[X]) = {
        #inv.#q.
        let m = {{force queue_pop} q}
        match m {
            /* None */ _u => {ret inv}
            /* Some */ q_ctx => { 
                let (q, ctx) = {ret q_ctx}
                let m = {{force visit_ctx}[X] inv ctx}
                match m {
                    /* NoChange */ _u  => {{force do_work_queue}[X] inv q}
                    /* Changed */  inv => {
                        let ctxs = {{force ctx_succs} ctx}
                        let q    = {{force queue_push_all} q ctxs}
                        {{force do_work_queue}[X] inv q}
                    }
                }
            }            
        }
    }

    // TODO: This return type needs an existential quantifier
    fn run : (Thk[0] 0 F Inv[0]) = {
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
