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
        ret 0
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


////////////////////////////////////////////////////////////////////////
// For reference: Some list-processing tests
pub mod test_list {

    /*  Try this:
     *  $ cargo test test_list::typing 2>&1 | less -R
     *
     */    
    #[test]
    pub fn typing() { fgi_listing_test!{
        open fungi_lang::examples::list_nat;
        ret 0
    }}

    /*  Try this:
     *  $ export FUNGI_VERBOSE_REDUCE=1
     *  $ cargo test test_list::reduction -- --nocapture | less -R
     *
     */
    #[test]
    pub fn reduction() { fgi_dynamic_trace!{
        [Expect::SuccessXXX]
        open fungi_lang::examples::list_nat;
        open fungi_lang::examples::list_nat_edit;
        open fungi_lang::examples::list_nat_reverse;
        
        /// Generate input
        let list1  = {ws (@@gen) {{force gen} 10}}

        /// Allocate nil ref cell
        let refnil = {ref (@@nil) roll inj1 ()}

        /// Allocate archivist thunk: when forced, it reverses the input list
        let t = {ws (@@archivst) thk (@@comp) {
            let list2 = {{force reverse} {!list1} refnil (@@revres)}
            ret (list1, list2)
        }}

        /// Initial run
        let outs_1 = {force t}

        /// First change: Insert name 666, element 666 after name 5
        let b1 = {
            {force insert_after}[?] (@5) (@666) 666 {!list1}
        }

        /// Re-force archivist; Precipitates change propagation
        let outs_2 = {force t}

        /// Second change: Remove inserted name 666, and element 666
        let b2 = {
            {force remove_after}[?] (@5) {!list1}
        }

        /// Re-force archivist; Precipitates change propagation
        let outs_3 = {force t}
        ret (b1, b2)
    }}
}

