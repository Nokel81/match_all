# match_all
Provides the `match_all!` macro for rust

This macro provides similar functionality to a vanilla `match` statement but allows for multiple expression blocks to be executed.

## Format

```rust
match_all!{ value,
    IfNoMatch => expr,
    pat | pat ... => expr,
    ...
}
```

* `value`: the expression to match on.
* `IfNoMatch`: the expression after this is executed if none of the other patterns are matched. This branch is optional.
* `pat | pat ...`: this is groupings of patterns that will be checked. If any of them match to value then their corresponding expression is executed. After checking a group of patterns then the next group is checked until all groups have been checked. If none match then the `IfNoMatch` expression will be executed.

## Example One

```rust
let value = Some(4);

match_all!{ value,
    None => println!("Hi"),
    Some(3) | Some(4) => println!("Hello"),
    Some(4) | Some(5) => println!("Howdy")
}
```

This would print:

```
Hello
Howdy
```

## Example Two

```rust
let value = Some(20);
match_all!{ value,
    IfNoMatch => println!("No Match"),
    0...4 => println!("0 through 4")
};
```

This would print:

```
No Match
```

It prints this because it uses the special `IfNoMatch` expression to provide an expression that matches only when no other expression was matched.