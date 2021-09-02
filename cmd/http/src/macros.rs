#[macro_export]
macro_rules! wraped_or_tree {
    ($x:expr $(,)?) => { wraped_debug_boxed!($x) };
    ($($x:expr),+ $(,)?) => {
        wraped_or_tree!(@internal; $($x),+; $($x),+)
    };
    (@internal $($left:expr),*; $head:expr, $($tail:expr),+; $a:expr $(,$b:expr)?) => {
        (wraped_or_tree!($($left,)* $head)).or(wraped_or_tree!($($tail),+))
    };
    (@internal $($left:expr),*; $head:expr, $($tail:expr),+; $a:expr, $b:expr, $($more:expr),+) => {
        wraped_or_tree!(@internal $($left,)* $head; $($tail),+; $($more),+)
    };
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! wraped_debug_boxed {
    ($x:expr) => {
        ::warp::Filter::boxed($x)
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! wraped_debug_boxed {
    ($x:expr) => {
        $x
    };
}