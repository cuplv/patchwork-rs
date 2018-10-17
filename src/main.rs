#[macro_use] extern crate fungi_lang;

fgi_mod!{
    /// Rust-based representations of program locations and abstract
    /// states.
    type Loc;
    type AbsState;

    /// Rust-based representation of a finite map.
    /// (eventually, use finer-grained representation of map updates).
    /// The finite map associates each program location with an abstract state.
    type Map;
    type Queue;
    
    /// Invariant map; the type tracks the set of update names
    type Inv = (foralli (X):NmSet. Map);
    
    /// The result of visiting a location: 
    /// Either: no change, or an updated invariant map
    type VisitRes = (foralli (X):NmSet. (+ Unit + Inv[X]));

    /// create the initial queue
    fn queue_init : (Thk[0] 0 F Queue)  = { unsafe (0) trapdoor::queue_init }
    
    /// create the initial invariant map
    fn inv_init : (Thk[0] 0 F Inv[0]) = { unsafe (0) trapdoor::inv_init }

    fn queue_pop : (Thk[0] 0 Queue -> 0 F (+ Unit + (x Queue x Loc))) = {
        // TODO
        #q. ret inj1 ()
    }

    fn queue_push_succs : (Thk[0] 0 Queue -> 0 Loc -> 0 F Queue) = {
        // TODO
        #q. #loc. ret q
    }    

    // TODO: This type should return an Inv with an existentially-bound set of names
    fn visit_loc : (Thk[0] foralli (X):NmSet.
                    0 Inv[X] -> 
                    0 Loc -> 
                    0 F VisitRes[X]) = {
        #inv.#loc.
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
            _u => {ret inv}
            q_loc => { 
                let (q, loc) = {ret q_loc}
                let m = {{force visit_loc}[X] inv loc}
                match m {
                    _u  => {{force visit_queue}[X] inv q}
                    inv => {
                        let q = {{force queue_push_succs} q loc}
                        {{force visit_queue}[X] inv q}
                    }
                }
            }            
        }
    }
}

pub mod trapdoor {
    // This code essentially extends the Fungi evaluator from within Patchwork.
    use fungi_lang::dynamics::{RtVal,ExpTerm};
    //use super::*;
    
    pub fn queue_init(_args:Vec<RtVal>) -> ExpTerm {
        unimplemented!()
    }
    pub fn inv_init(_args:Vec<RtVal>) -> ExpTerm {
        unimplemented!()
    }
}


fn main() {
    println!("Hello, world!");
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
        let q = {force queue_init} 
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
        ret 0
    }}
}
