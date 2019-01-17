extern crate libc;
extern crate gmp_mpfr_sys;

// Represents opaque C structs from apron
// See github.com/rust-lang/rust/issues/27303 for explanation of this representation as enum.
enum __ap_manager_t {}
type ap_manager_t = *mut __ap_manager_t;

enum __box_t {}
pub type box_t = *mut __box_t;

#[repr(C)]
struct __itv_t {
    inf: bound_t, //NB: to match APRON internal representation, `inf` is the _negation_ of the infimum
    sup: bound_t
} type itv_t = *mut __itv_t;

#[repr(C)]
struct ap_interval_t {
    inf: *mut ap_scalar_t,
    sup: *mut ap_scalar_t
}

#[repr(C)]
struct mpfr {} type mpfr_ptr = *mut mpfr;
#[repr(C)]
struct __itv_internal_t {} type itv_internal_t = *mut __itv_internal_t;
#[repr(C)]
struct __bound_t {} type bound_t = *mut __bound_t;

#[repr(C)]
union scalar_val  {
    dbl: libc::c_double,
    mpq_ptr: bound_t,
    mpfr_ptr: mpfr
}

#[repr(C)]
struct ap_scalar_t {
    discr: ap_scalar_discr_t,
    val: scalar_val
}

#[repr(C)]
enum ap_scalar_discr_t {
    AP_SCALAR_DOUBLE,
    AP_SCALAR_MPQ,
    AP_SCALAR_MPFR
}

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
        tinterval: *mut *mut ap_interval_t
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

    fn ap_interval_alloc() -> *mut ap_interval_t;
    fn ap_interval_free(itv: *mut ap_interval_t) -> ();
    fn ap_interval_print(itv: *mut ap_interval_t) -> ();
    
    fn ap_interval_set_int(
        itv: *mut ap_interval_t,
        inf: i64,
        sup: i64
    ) -> ();


    fn itv_internal_alloc() -> itv_internal_t;
    fn itv_internal_free(intern: itv_internal_t) -> ();
    fn itv_add(a: itv_t, b: itv_t, c: itv_t) -> ();
    fn itv_neg(a: itv_t, b: itv_t) -> ();
    fn itv_array_alloc(size: libc::size_t) -> itv_t;
    fn itv_array_free(a: itv_t, size: libc::size_t) -> ();
    fn itv_set_ap_interval(intern: itv_internal_t, dest: itv_t, src: *mut ap_interval_t) -> bool;
    fn ap_interval_set_itv(intern: itv_internal_t, dest: *mut ap_interval_t, src: itv_t) -> bool;
    fn itv_print(itv: itv_t) -> ();

    fn box_copy(man: ap_manager_t, b: box_t) -> box_t;

    fn box_bound_dimension(
        man: ap_manager_t,
        b: box_t,
        dim: u32
    ) -> *mut ap_interval_t;

    fn ap_scalar_neg(target: *mut ap_scalar_t, source: *mut ap_scalar_t) -> ();
    fn ap_scalar_alloc() -> *mut ap_scalar_t;
    fn ap_scalar_fprint_stdout(scalar: *mut ap_scalar_t) -> ();
}


fn box_bottom_1d() -> box_t {
    unsafe { box_man.with(|man| box_bottom(*man,1,0)) }
}

fn box_top_1d() -> box_t {
    unsafe { box_man.with(|man| box_top(*man,1,0)) }
}

fn box_new_1d(inf: i64, sup: i64) -> box_t {
    unsafe {
        let mut itv: *mut ap_interval_t = ap_interval_alloc();
        ap_interval_set_int(itv,inf,sup);
        let result = box_man.with(|man| box_of_box(*man,1,0, &mut itv) );

        ap_interval_free(itv);

        result
    }
}

fn box_add(l:box_t, r:box_t) -> box_t {
    box_man.with(|man| {unsafe{
        let l_interval = box_bound_dimension(*man, l, 0);
        let r_interval = box_bound_dimension(*man, r, 0);
        let mut new_interval = ap_interval_alloc();
        let (mut l_itv, mut r_itv, mut new_itv) = (itv_array_alloc(1), itv_array_alloc(1), itv_array_alloc(1));
        let intern = itv_internal_alloc();

        itv_set_ap_interval(intern, l_itv, l_interval);
        itv_set_ap_interval(intern, r_itv, r_interval);
        itv_add(new_itv, l_itv, r_itv);
        ap_interval_set_itv(intern, new_interval, new_itv);
        let result = box_of_box(*man, 1, 0, &mut new_interval);

        itv_array_free(l_itv, 1);
        itv_array_free(r_itv, 1);
        itv_array_free(new_itv, 1);
        itv_internal_free(intern);
        ap_interval_free(l_interval);
        ap_interval_free(r_interval);
        ap_interval_free(new_interval);

        result
    }})
}

fn box_negate(b:box_t) -> box_t {
    box_man.with(|man| {unsafe{
        let b_interval = box_bound_dimension(*man, b, 0);
        let mut new_interval = ap_interval_alloc();
        let mut b_itv = itv_array_alloc(1);
        let mut new_itv = itv_array_alloc(1);
        let intern = itv_internal_alloc();
        itv_set_ap_interval(intern, b_itv, b_interval);
        itv_neg(new_itv, b_itv);
        ap_interval_set_itv(intern, new_interval, new_itv);
        let result = box_of_box(*man,1,0,&mut new_interval);

        itv_array_free(b_itv,1);
        itv_array_free(new_itv,1);
        itv_internal_free(intern);
        ap_interval_free(new_interval);
        ap_interval_free(b_interval);

        result
    }})
}

fn clone(b: box_t) -> box_t {
    unsafe {box_man.with(|man| {
        box_copy(*man, b)
    })}
}


pub fn test_boxes() {
    unsafe {box_man.with(|man| {
        box_fdump_stdout(*man, box_new_1d(2, 5));
        box_fdump_stdout(*man, box_negate(box_new_1d(2,5)));
        box_fdump_stdout(*man, box_add(box_new_1d(2,5), box_new_1d(2,5)));
    })}}

thread_local! {
    static box_man: ap_manager_t = unsafe{box_manager_alloc()};
}

pub mod interval {
    use crate::sem::rep::{Stmt, Ctx, Exp};
    use std::collections::BTreeMap;
    use std::rc::Rc;
    use crate::apron;
    
    #[derive(Clone,Debug,Hash)]
    pub enum Invariant {
        Bottom,
        Inv(BTreeMap<String, apron::box_t>),
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

    pub fn bottom(_ctx : &Ctx) -> AbsState {
        Invariant::Bottom
    }
    pub fn init_absstate() -> AbsState {
        Invariant::Inv(BTreeMap::new())
    }

    pub fn join(phi1: AbsState, phi2: AbsState) -> AbsState {
        match (phi1.clone(), phi2.clone()) {
            (Invariant::Bottom, _) => phi2,
            
            (_, Invariant::Bottom) => phi1,
            
            (Invariant::Top,_) | (_, Invariant::Top) => Invariant::Top,
            
            (Invariant::Inv(inv1), Invariant::Inv(inv2)) => {
                let mut new_inv : BTreeMap<String, apron::box_t> = BTreeMap::new();
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

    fn abstract_eval(exp:Exp, inv:&Invariant) -> apron::box_t {
        match exp {
            Exp::Num(n)     => apron::box_new_1d(n as i64, n as i64),
            Exp::Var(v)     => invariant_get(inv, &v),
            Exp::Neg(e)     => {
                let subexpr = match Rc::try_unwrap(e) {
                    Ok(se) => se,
                    Err(_) => panic!("RC cell error in subexpression")
                };
                let subexpr_box = abstract_eval(subexpr,inv);
                apron::box_negate(subexpr_box)
            },
            Exp::Plus(l, r) => {
                let (l_expr,r_expr) = match (Rc::try_unwrap(l),Rc::try_unwrap(r)) {
                    (Ok(l_subexpr),Ok(r_subexpr)) => (l_subexpr, r_subexpr),
                    _ => panic!("RC cell error in subexpression")
                };
                let l_box = abstract_eval(l_expr, inv);
                let r_box = abstract_eval(r_expr, inv);
                apron::box_add(l_box, r_box)

            }
        }
    }

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

    fn invariant_get(inv:&Invariant, key:&String) -> apron::box_t {
        match inv {
            Invariant::Top => apron::box_top_1d(),
            Invariant::Bottom => apron::box_bottom_1d(),
            Invariant::Inv(env) => *env.get(key).unwrap_or(&apron::box_bottom_1d())
        }
    }
}
    
