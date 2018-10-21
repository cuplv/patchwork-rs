use crate::sem::rep::{Ctx,Ctxs};

fgi_mod!{
    open crate::sem;

    /// Work queue representation (for work queue algorithm)
    /// ----------------------------------------------------
    type Queue; // := Host(Queue)

    /// empty queue
    fn queue_empty : (Thk[0] 0 F Queue) = { 
        hostfn (0) {
            let q : Queue = Queue( vec![ ] );
            fgi_rtval!( host q )
        }
    }

    /// singleton queue
    fn queue_sing : (Thk[0] 0 Ctx -> 0 F Queue) = {
        hostfn (1) {
            #(ctx:Ctx).
            let q : Queue = Queue( vec![ ctx ] );
            fgi_rtval!( host q )
        }
    }
    
    /// pop the next analysis context from the queue
    fn queue_pop : (Thk[0] 0 Queue -> 0 F (+ Unit + (x Queue x Ctx))) = {
        hostfn (1) {
            #(mut q:Queue).
            match q.0.pop() {
                None    => fgi_rtval!( inj1() ),
                Some(v) => fgi_rtval!( inj2(host q, host v) ),
            }
        }
    }

    /// push the given analysis context onto the queue
    fn queue_push : (Thk[0] 0 Queue -> 0 Ctx -> 0 F Queue) = {
        hostfn (2) {
            #(mut q:Queue). 
            #(ctx: Ctx).
            q.push( ctx ); 
            fgi_rtval!( host q )
        }
    }

    /// push the given analysis context set onto the queue
    fn queue_push_all : (Thk[0] 0 Queue -> 0 Ctxs -> 0 F Queue) = {
        hostfn (2) {
            #(mut q:Queue). 
            #(ctxs: Ctxs).
            q.push_all( ctxs );
            fgi_rtval!( host q )
        }
    }
}

#[derive(Clone,Debug,Eq,PartialEq,Hash)]
pub struct Queue( Vec<Ctx> );

impl Queue {
    pub fn push(&mut self, ctx:Ctx) {
        // Before pusing, check to see if the context is already in the queue
        let found = {
            let mut found = false;
            for q_ctx in self.0.iter() { 
                if q_ctx == &ctx { 
                    found = true; 
                    break 
                }
            };
            found
        };
        if ! found {
            self.0.push( ctx );
        };
    }
    pub fn push_all(&mut self, ctxs:Ctxs) {
        for ctx in ctxs {
            self.push(ctx)
        }
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



