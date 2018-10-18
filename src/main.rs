#![recursion_limit="128"]
#[macro_use] extern crate fungi_lang;

pub mod sem;
pub mod cfg;
pub mod inv;
pub mod queue;

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
        // TODO
        ret inj1 ()
    }
       
    // TODO: This type should return an Inv with an existentially-bound set of names
    fn visit_queue : (Thk[0] foralli (X):NmSet.
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
                    /* NoChange */ _u  => {{force visit_queue}[X] inv q}
                    /* Changed */  inv => {
                        let ctxs = {{force ctx_succs} ctx}
                        let q    = {{force queue_push_all} q ctxs}
                        {{force visit_queue}[X] inv q}
                    }
                }
            }            
        }
    }
}


pub mod test {
    /*  Try this:
     *  $ cargo test::typing 2>&1 | less -R
     *
     */    
    #[test]
    pub fn typing() { fgi_listing_test!{
        open crate;
        let inv = {force inv_init}
        // todo: load the program graph; put entry nodes into q
        let q = {force queue_empty} 
        let res = {{force visit_queue} [0] inv q}
        ret res
    }}
    
    /*  Try this:
     *  $ export FUNGI_VERBOSE_REDUCE=1
     *  $ cargo test test::reduction -- --nocapture | less -R
     *
     */
    #[test]
    pub fn reduction() { fgi_dynamic_trace!{[Expect::SuccessXXX]
        open crate;
        let inv = {force inv_init}
        // todo: load the program graph; put entry nodes into q
        let q = {force queue_empty} 
        let res = {{force visit_queue} [0] inv q}
        ret 0
    }}
}

///////////////////////////////////////////////////////////////////////////////////////

fn main() {
    println!("Hello, world!");
}

