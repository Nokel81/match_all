//! # match_all
//! Provides the `match_all!` macro for rust
//!
//! This macro provides similar functionality to a vanilla `match` statement but allows for multiple expression blocks to be executed.
//!
//! ## Format
//!
//!     match_all!{ value,
//!         IfNoMatch => expr,
//!         pat | pat ... => expr,
//!         ...
//!     }
//!
//! * `value`: the expression to match on.
//! * `IfNoMatch`: the expression after this is executed if none of the other patterns are matched. This branch is optional.
//! * `pat | pat ...`: this is groupings of patterns that will be checked. If any of them match to value then their corresponding expression is executed. After checking a group of patterns then the next group is checked until all groups have been checked. If none match then the `IfNoMatch` expression will be executed.
//!
//! ## Example One
//!
//!     let value = Some(4);
//!
//!     match_all!{ value,
//!         None => println!("Hi"),
//!         Some(3) | Some(4) => println!("Hello"),
//!         Some(4) | Some(5) => println!("Howdy")
//!     }
//!
//! This would print:
//!
//!     Hello
//!     Howdy
//!
//! ## Example Two
//!
//!     let value = Some(20);
//!     match_all!{ value,
//!         IfNoMatch => println!("No Match"),
//!         0...4 => println!("0 through 4")
//!     };
//!
//! This would print:
//!
//!     No Match
//!
//! It prints this because it uses the special `IfNoMatch` expression to provide an expression that matches only when no other expression was matched.


#[macro_export]
macro_rules! match_all {
   ($val:expr, IfNoMatch => $c:expr, $($($p:pat)|+ => $b:expr),+) => {{
        let mut matched = false;
        $(
            loop {
                $(
                    #[allow(unreachable_patterns)]
                    match $val {
                        $p => {
                            $b;
                            matched = true;
                            break;
                        },
                        _ => ()
                    }
                )+
                break
            }
        )+
        if !matched {
            $c
        }
   }};
   ($val:expr, $($($p:pat)|+ => $b:expr),+) => {{
        $(
            loop {
                $(
                    #[allow(unreachable_patterns)]
                    match $val {
                        $p => {
                            $b;
                            break;
                        },
                        _ => ()
                    }
                )+
                break
            }
        )+
   }};
}

//! # for_match_all
//! Provides the `for_match_all!` macro for rust
//!
//! This macro combines the functionality of a for loop and the `match_all!`
//!
//! ## Format
//!
//!     for_match_all!{ ident in arr,
//!         IfNoMatch => expr,
//!         pat | pat ... => expr,
//!         ...
//!     }
//!
//! * `ident`: the identifier that can be used during each for loop iterator in either the patterns or the expressions
//! * `arr`: an expression that has the `.iter()` method, this holds the values to iterate through
//! * `IfNoMatch`: the expression after this is executed if none of the other patterns are matched. This branch is optional.
//! * `pat | pat ...`: this is groupings of patterns that will be checked. If any of them match to value then their corresponding expression is executed. After checking a group of patterns then the next group is checked until all groups have been checked. If none match then the `IfNoMatch` expression will be executed.
//!
//! ## Example One
//!
//!     let arr = [1, 2, 3, 4];
//!
//!     for_match_all!{x in arr,
//!         IfNoMatch => println!("even"),
//!         1 | 3 | 5 => println!("odd")
//!     }
//!
//! This would print:
//!
//!     odd
//!     even
//!     odd
//!     even

#[macro_export]
macro_rules! for_match_all {
   ($var:ident in $val:expr, IfNoMatch => $c:expr, $($($p:pat)|+ => $b:expr),+) => {{
    for $var in $val.iter() {
        let mut matched = false;
        $(
            loop {
                $(
                    #[allow(unreachable_patterns)]
                    match *$val {
                        $p => {
                            $b;
                            matched = true;
                            break;
                        },
                        _ => ()
                    }
                )+
                break
            }
        )+
        if !matched {
            $c
        }
    }
   }};
   ($var:ident in $val:expr, $($($p:pat)|+ => $b:expr),+) => {{
     for $var in $val.iter() {
        $(
            loop {
                $(
                    #[allow(unreachable_patterns)]
                    match *$var {
                        $p => {
                            $b;
                            break;
                        },
                        _ => ()
                    }
                )+
                break
            }
        )+
    }
   }};
}

//! # for_match
//! Provides the `for_match!` macro for rust
//!
//! This macro combines the functionality of a `for` loop and a `match` statement. So it iterates through each element in the expression and calls match on it
//!
//! ## Format
//!
//!     for_match_all!{ ident in arr,
//!         pat | pat ... => expr,
//!         ...
//!     }
//!
//! * `arr`: an expression that has the `.iter()` method, this holds the values to iterate through
//! * `IfNoMatch`: the expression after this is executed if none of the other patterns are matched. This branch is optional.
//! * `pat | pat ...`: this is groupings of patterns that will be checked. If any of them match to value then their corresponding expression is executed. After checking a group of patterns then the next group is checked until all groups have been checked. If none match then the `IfNoMatch` expression will be executed.
//!
//! ## Example One
//!
//!     let arr = [1, 2, 3, 4];
//!
//!     for_match!{x in arr,
//!         1 | 3 | 5 => println!("odd"),
//!         _ => println!("even")
//!     }
//!
//! This would print:
//!
//!     odd
//!     even
//!     odd
//!     even

macro_rules! for_match {
   ($var:ident in $val:expr, $($($p:pat)|+ => $b:expr),+) => {{
    for $var in $val.iter() {
        loop {
            $(
                $(
                    #[allow(unreachable_patterns)]
                    match *$var {
                        $p => {
                            $b;
                            break;
                        },
                        _ => ()
                    }
                )+
            )+
            break
        }
    }
   }};
}
