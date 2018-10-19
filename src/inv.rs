fgi_mod!{
    open crate::sem;

    /// Invariant map representation
    /// ----------------------------

    /// Rust-based representation of a finite map.
    /// (eventually, use finer-grained representation of map updates).
    /// The finite map associates each distinct analysis context with an abstract state.
    type Map;
    
    /// Invariant map; the refinement type tracks the set of named update operations
    type Inv = (foralli (X):NmSet. Map);
    
    /// create the initial invariant map; no updates yet; names := empty set
    fn inv_init : (Thk[0] 0 F Inv[0]) = { unsafe (0) trapdoor::inv_init }
    
    /// update the abstract state at a particular context in the invariant map
    fn inv_update : (
        Thk[0] foralli (X,Y,XY):NmSet | ((X%Y)=XY:NmSet).
            0 Inv[X] -> 0 Nm[Y] -> 0 Ctx -> 0 AbsState -> 0 F Inv[X%Y]
    ) = { 
        unsafe (4) trapdoor::inv_update
    }

    /// project a particular context's abstract state from the invariant map
    fn inv_get : (
        Thk[0] foralli (X):NmSet.
            0 Inv[X] -> 0 Ctx -> 0 F AbsState
    ) = {
        unsafe (2) trapdoor::inv_get
    }

}


/*  Try this:
 *  $ cargo inv::typing 2>&1 | less -R
 *
 */    
#[test]
pub fn typing() { fgi_listing_test!{
    open crate::inv;
    ret 0
}}


/////////////////////////////////////////////////////////////////////////////////////

/// Trapdoor into Fungi's dynamic semantics.
/// 
/// This module defines operations on invariant maps by extending the
/// Fungi evaluator's semantics, but from within this (Patchwork) crate.
///
pub mod trapdoor {
    use std::collections::{HashMap};
    use std::hash::{Hash, Hasher};
    use fungi_lang::dynamics::{RtVal,ExpTerm,ret};
    use fungi_lang::hostobj::{rtval_of_obj, obj_of_rtval};
    //use super::*;
    use crate::sem::rep::{Ctx,domain::{AbsState,bottom}};

    #[derive(Clone,Debug,Eq,PartialEq)]
    pub struct Map ( HashMap<Ctx,AbsState> );
    
    impl Map {
        fn get(&self, ctx:Ctx) -> AbsState {
            let r = self.0.get( &ctx ).map(|x|x.clone());
            match r {
                None    => bottom(),
                Some(s) => s
            }
        }
        fn update(&mut self, ctx:Ctx, s:AbsState) {
            *self.0.entry( ctx ).or_insert( bottom() ) = s;
        }
    }

    // This representation does not permit an efficient O(1)-time hash operation.
    impl Hash for Map {        
        fn hash<H:Hasher>(&self, h: &mut H) {
            // Take O(n log n) time to sort keys, and hash each key-value pair
            let mut keys : Vec<Ctx> = 
                self.0.keys().map(|x|x.clone()).into_iter().collect();
            keys.sort();
            for k in keys {
                k.hash(h);
                self.0.get(&k).hash(h);
            }
        }
    }
        
    pub fn inv_init(_args:Vec<RtVal>) -> ExpTerm {
        let inv : Map = Map(HashMap::new());
        ret(rtval_of_obj( inv ))
    }

    pub fn inv_get(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 2);
        let inv : Map = obj_of_rtval( &args[0] ).unwrap();
        let loc : Ctx = obj_of_rtval( &args[1] ).unwrap();
        ret(rtval_of_obj(inv.get(loc)))
    }
    
    pub fn inv_update(args:Vec<RtVal>) -> ExpTerm {
        assert_eq!(args.len(), 4);
        let mut inv : Map  = obj_of_rtval( &args[0] ).unwrap();
        //let nm  : RtVal    = obj_of_rtval( &args[1] ).unwrap();
        let ctx : Ctx      = obj_of_rtval( &args[2] ).unwrap();
        let st  : AbsState = obj_of_rtval( &args[3] ).unwrap();
        inv.update(ctx, st);
        ret(rtval_of_obj( inv ))
    }
}

