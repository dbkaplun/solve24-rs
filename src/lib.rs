#![feature(match_default_bindings)]
#![feature(box_patterns)]

/*!
```
let card = solve24::Card::new(vec![1., 3., 4., 6.]); // Has one solution
let mut solutions = card.solve();
assert_eq!(solutions.next().unwrap().to_infix_notation(), "(6/(1-(3/4)))");
assert_eq!(solutions.next(), None);
```
*/

mod val;
pub use val::{Val};

mod card;
pub use card::{Card, DEFAULT_SOLUTION};

mod op;
pub use op::{Op, OpFn, Ops};

mod boundop;
pub use boundop::BoundOp;

mod util;
