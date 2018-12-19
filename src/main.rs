#![recursion_limit="128"]
#![feature(libc)]
#![feature(untagged_unions)]
#[macro_use] extern crate fungi_lang;

pub mod sem;
pub mod cfg;
pub mod inv;
pub mod queue;
pub mod fix;
pub mod apron;

///////////////////////////////////////////////////////////////////////////////////////

/*  Try this:
 *  $ export FUNGI_VERBOSE_REDUCE=1
 *  $ cargo run | less -R
 *
 */


fn main() {
    fix::run();
    //apron::test_boxes();
}

