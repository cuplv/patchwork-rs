extern crate libc;
extern crate gmp_mpfr_sys;

// Represents opaque C structs from apron
// See github.com/rust-lang/rust/issues/27303 for explanation of this representation as enum.
enum __ap_manager_t {}
type ap_manager_t = *mut __ap_manager_t;

enum __box_t {}
pub type box_t = *mut __box_t;

enum __ap_interval_t {}
type ap_interval_t = *mut __ap_interval_t;

extern {
    fn box_manager_alloc () -> ap_manager_t;

    fn box_fdump_stdout(
        man: ap_manager_t,
        b: box_t
    ) -> ();
    
    fn box_bottom(
        man: ap_manager_t,
        intdim: libc::size_t,
        realdim: libc::size_t
    ) -> box_t;
    
    fn box_top(
        man: ap_manager_t,
        intdim: libc::size_t,
        realdim: libc::size_t
    ) -> box_t;

    fn box_of_box(
        man: ap_manager_t,
        intdim: libc::size_t,
        realdim: libc::size_t,
        tinterval: *mut ap_interval_t
    ) -> box_t;
    
    fn box_join(
        man: ap_manager_t,
        destructive: bool,
        b1: box_t,
        b2: box_t
    ) -> box_t;

    fn box_meet(
        man: ap_manager_t,
        destructive: bool,
        b1: box_t,
        b2: box_t
    ) -> box_t;

    fn box_widen(
        man: ap_manager_t,
        b1: box_t,
        b2: box_t
    ) -> box_t;

    fn box_is_eq(
        man: ap_manager_t,
        b1: box_t,
        b2: box_t
    ) -> bool;

    fn ap_interval_alloc() -> ap_interval_t;
    fn ap_interval_free(itv: ap_interval_t) -> ();
    fn ap_interval_print(itv: ap_interval_t) -> ();
    
    fn ap_interval_set_int(
        itv: ap_interval_t,
        inf: i64,
        sup: i64
    ) -> ap_interval_t;    
}

fn box_new_1d(inf: i64, sup: i64) -> box_t {
    unsafe {
        let mut itv = ap_interval_alloc();
        ap_interval_set_int(itv,inf,sup);
        box_man.with(|man|
                     box_of_box(*man,1,0, &mut itv)
        )
    }
}

pub fn test_boxes() {
    unsafe {box_man.with(|man| {
        box_fdump_stdout(*man, box_bottom(*man,1,0));
        box_fdump_stdout(*man, box_top(*man,1,0));
        box_fdump_stdout(*man, box_new_1d(2, 5));
    })}

}
thread_local! {
    static box_man: ap_manager_t = unsafe{box_manager_alloc()};
}

pub mod interval {
    use crate::sem::rep::{Stmt, Ctx, Exp};
    use std::collections::{HashMap};
    use crate::apron;
    
    #[derive(Clone,Debug)]
    pub enum Invariant {
        Bottom,
        Inv(HashMap<String, apron::box_t>),
        Top,
    }
    
    pub type AbsState = Invariant;

    impl PartialEq for Invariant {
        fn eq(&self, other: &Invariant) -> bool {
            match (self, other) {
                (Invariant::Bottom, Invariant::Bottom) => true,
                (Invariant::Top, Invariant::Top) => true,
                (Invariant::Inv(inv1), Invariant::Inv(inv2)) =>
                    if inv1.len() != inv2.len() {
                        false
                    } else {
                        apron::box_man.with( |man| 
                                       inv1.iter().all( |(k,v1)| inv2.get(k).map_or(false, |v2| unsafe{apron::box_is_eq(*man,*v1,*v2)}))
                        )
                    }
                (_, _) => false
            }
        }

    }
    
    impl Eq for Invariant {}

    pub fn bottom(_ctx : Ctx) -> AbsState {
        Invariant::Bottom
    }

    pub fn join(phi1: AbsState, phi2: AbsState) -> AbsState {
        match (phi1.clone(), phi2.clone()) {
            (Invariant::Bottom, _) => phi2,
            
            (_, Invariant::Bottom) => phi1,
            
            (Invariant::Top,_) | (_, Invariant::Top) => Invariant::Top,
            
            (Invariant::Inv(inv1), Invariant::Inv(inv2)) => {
                let mut new_inv : HashMap<String, apron::box_t> = HashMap::new();
                for (key, value) in inv1.iter() {
                    new_inv.insert(key.to_owned(), *value);
                }
                for (key, value) in inv2.iter() {
                    if new_inv.contains_key(key) {
                        let inv1_value = *new_inv.get(key).unwrap();
                        let joined_value = unsafe { apron::box_man.with( |man|
                            apron::box_join(*man, false, inv1_value, *value)
                        )};                                                              
                        new_inv.insert(key.to_owned(), joined_value);
                    } else {
                        new_inv.insert(key.to_owned(), *value);
                    }
                }
                Invariant::Inv(new_inv)
            }
            
        }
    }
    
    pub fn transfer(pre:Invariant, _ctx:&Ctx, stmt:&Stmt) -> Invariant {
        if pre == Invariant::Top || pre == Invariant::Bottom {
            return pre;
        }
        match stmt.clone() {
            Stmt::Nop | Stmt::AssertEq(_, _) | Stmt::AssertNeq(_, _) => pre,
            Stmt::Incr | Stmt::Decr | Stmt::Set(_) => panic!("This domain is designed for a language with variables, not the one-memory-cell toy language that uses these statements"),
            Stmt::Update(lhs, rhs) => invariant_insert(pre.clone(), lhs, abstract_eval(rhs, &pre))
        }
    }

    fn abstract_eval(exp:Exp, inv:&Invariant) -> apron::box_t {panic!("TODO")}

    fn invariant_insert(inv:Invariant, key:String, value:apron::box_t) -> Invariant {
        match inv {
            Invariant::Top | Invariant::Bottom => inv,
            Invariant::Inv(env) => {
                let mut new_env = env.clone();
                new_env.insert(key, value);
                Invariant::Inv(new_env)
            }
        }
    }
    
}
    
