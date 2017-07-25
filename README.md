# match_all
Provides the match_all! macro for rust

This macro provides similar functionality to a vanilla `match` statement but allows for multiple expression blocks to be executed.

Format:
```rust
match_all!{
    value,
    IfNoMatch => expr,
    pat | pat ... => expr,
    ...
}
```
* `value`: is the value that is wished to be matched on, this can be any function or variable that reduces to a value
* `IfNoMatch`: the expression after this is executed if none of the other patterns are matched. This branch is optional.
* `pat | pat ...`: this is groupings of patterns that will be checked. If any of them match to value then their corresponding expression is executed. After checking a group of patterns then the next group is checked until all groups have been checked. If none match then the `IfNoMatch` expression will be executed.

This can be used to set values of a variable but the following must hold:

1. The `IfNoMatch` block must be present

Examples:

1.
```rust
    let value = Some(4);

    match_all!{ value,
        None => println!("Hi"),
        Some(3) | Some(4) => println!("Hello"),
        Some(4) | Some(5) => println!("Howdy")
    }

    > Hello
    > Howdy
```
2.
```rust
    let x = match_all!{ some_fn(),
        IfNoMatch => {
            println!("No Match");
            false
        }
        0..4 => true
    };
```
