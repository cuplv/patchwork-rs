#[macro_use] extern crate fungi_lang;

fgi_mod!{
    /// TODO -- Define the type definitions and analysis loop here    
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

