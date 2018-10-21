fgi_mod!{
    open crate::sem;

    /// Invariant map representation
    /// ----------------------------

    /// Rust-based representation of a finite map.
    /// (eventually, use finer-grained representation of map updates).
    /// The finite map associates each distinct analysis context with an abstract state.
    type Map; // := Host(Map)
    
    /// Invariant map; the refinement type tracks the set of named update operations
    type Inv = (foralli (X):NmSet. Map[X]);
    
    /// create the initial invariant map; no updates yet; names := empty set
    fn inv_init : (Thk[0] 0 F Inv[0]) = { 
        hostfn (0) {
            let empty : Map = Map::new();
            fgi_rtval!( host empty )
        }
    }
    
    /// update the abstract state at a particular context in the invariant map
    fn inv_update : (
        Thk[0] foralli (X,Y,XY):NmSet | ((X%Y)=XY:NmSet).
            0 Inv[X] -> 0 Nm[Y] -> 0 Ctx -> 0 AbsState -> 0 F Inv[X%Y]
    ) = { 
        hostfn (4) {
            #(mut inv : Map).
            #_nm.
            #(ctx : Ctx).
            #(st  : AbsState).
            inv.update(ctx, st);
            fgi_rtval!( host inv )
        }            
    }

    /// project a particular context's abstract state from the invariant map
    fn inv_get : (
        Thk[0] foralli (X):NmSet.
            0 Inv[X] -> 0 Ctx -> 0 F AbsState
    ) = {
        hostfn (2) {
            #(inv : Map).
            #(ctx : Ctx).
            let s : AbsState = inv.get(&ctx);
            fgi_rtval!( host s )
        }
    }

    /// ???
    fn inv_join : (
        Thk[0] foralli (X):NmSet.
            0 Inv[X] -> 0 Preds -> 0 Ctx -> 0 F AbsState
    ) = {
        hostfn (3) {
            #(inv   : Map).
            #(preds : Preds).
            #(ctx   : Ctx).
            let s : AbsState = inv.join(preds, ctx);
            fgi_rtval!( host s )
        }
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

use std::hash::{Hash, Hasher};
use std::collections::{HashMap};
use crate::sem::rep::{Ctx,Preds};
use crate::sem::domain::{AbsState,bottom,join,transfer};

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Map ( HashMap<Ctx,AbsState> );
    
impl Map {
    fn new() -> Map {
        Map ( HashMap::new () )
    }

    fn get(&self, ctx:&Ctx) -> AbsState {
        let r = self.0.get( &ctx ).map(|x|x.clone());
        match r {
            None    => bottom(&ctx),
            Some(s) => s
        }
    }
    fn join(&self, preds:Preds, ctx:Ctx) -> AbsState {
        let mut s = None;
        for (pred_ctx, pred_stmt) in preds.iter() {
            let s1 = self.get(pred_ctx);
            let s2 = transfer(s1, pred_ctx, pred_stmt);
            s = s.map(|s| join(s, s2));
        }
        match s {
            // Do not call bottom unless there are no predecessors at all
            None    => bottom(&ctx),
            Some(s) => s,
        }
    }
    fn update(&mut self, ctx:Ctx, s:AbsState) {
        *self.0.entry( ctx ).or_insert( bottom(&ctx) ) = s;
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
