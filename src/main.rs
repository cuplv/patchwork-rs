#![recursion_limit="128"]
#[macro_use] extern crate fungi_lang;

pub mod sem;
pub mod cfg;
pub mod inv;
pub mod queue;
pub mod fix;

///////////////////////////////////////////////////////////////////////////////////////

/*  Try this:
 *  $ export FUNGI_VERBOSE_REDUCE=1
 *  $ cargo run | less -R
 *
 */

fn main() {
    fix::run();    
}

