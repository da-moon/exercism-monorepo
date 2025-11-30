# Learning Notes

## Building a 2D Matrix from `&[&str]` using fold + BTreeMap

This pattern converts a slice of strings into a coordinate-indexed map:

```rust
use std::collections::BTreeMap;

let matrix = garden.iter().copied().enumerate().fold(
    BTreeMap::new(),
    |mut acc: BTreeMap<(usize, usize), char>, row: (usize, &str)| {
        let (row_idx, row_val) = row;
        row_val.chars().enumerate().for_each(|(col_idx, elem)| {
            acc.insert((row_idx, col_idx), elem);
        });
        acc
    },
);
```

### What it does:

- `garden.iter().copied()` - iterates over `&str` items
- `.enumerate()` - pairs each row with its index:
  `(0, "row0"), (1, "row1"), ...`
- `.fold(BTreeMap::new(), ...)` - accumulates into a map
- Inner loop: `.chars().enumerate()` - gets each char with column index
- Result: `BTreeMap<(row, col), char>` where you can lookup any cell by
  coordinate

### Example:

Input: `&[" * ", "   "]`

Produces:

```
{(0,0): ' ', (0,1): '*', (0,2): ' ', (1,0): ' ', (1,1): ' ', (1,2): ' '}
```

### Why BTreeMap over HashMap?

- Keys stay sorted by `(row, col)` - useful for debugging/iteration in order
- For small grids, performance difference is negligible

---

## Why `&result.push()` gives "unused borrow" warning

This expression:

```rust
&result.push(row)
```

Parses as:

```rust
&(result.push(row))
```

What happens:

1. `result.push(row)` executes and returns `()` (unit type — push returns
   nothing)
2. `&` takes a reference to that `()`, giving you `&()`
3. That `&()` is immediately discarded — hence "unused borrow"

### Rust's auto-ref for method calls

You don't need to manually add `&` or `&mut` when calling methods. When you
write:

```rust
result.push(row)
```

Rust sees that `push` needs `&mut self` and automatically transforms it to:

```rust
(&mut result).push(row)
```

This is called "auto-ref" — Rust automatically adds the appropriate reference
based on the method signature.

### Key takeaway

Just write `result.push(row)` — never `&result.push()` or `&mut result.push()`.

---

## Functional Style Alternative for Flower Field

Instead of imperative loops with `for` and `push`, you can use iterators and
`map`/`collect`:

```rust
pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()                           // 1
        .enumerate()                      // 2
        .map(|(i, row)| {                 // 3
            row
                .bytes()                  // 4
                .enumerate()              // 5
                .map(|(j, cell)| {        // 6
                    if cell == b'*' {     // 7
                        '*'
                    } else {
                        match count_flowers(garden, i as i32, j as i32) {  // 8
                            0 => ' ',                                       // 9
                            n => char::from_digit(n, 10).unwrap(),          // 10
                        }
                    }
                })
                .collect()                // 11
        })
        .collect()                        // 12
}
```

### Line-by-line explanation:

1. **`.iter()`** — Iterates over `garden`, yielding `&&str` (references to each
   row string)

2. **`.enumerate()`** — Wraps each item with its index:
   `(0, &"row0"), (1, &"row1"), ...`

3. **`.map(|(i, row)| { ... })`** — Transforms each `(index, row)` pair into a
   new `String`. The closure receives:
   - `i`: row index (usize)
   - `row`: the row string (`&&str`, but Rust auto-derefs so you can treat it
     like `&str`)

4. **`row.bytes()`** — Iterates over the row as bytes (`u8`). Faster than
   `.chars()` for ASCII. Each byte is the ASCII value (e.g., `b'*'` = 42,
   `b' '` = 32)

5. **`.enumerate()`** — Pairs each byte with its column index:
   `(0, b' '), (1, b'*'), ...`

6. **`.map(|(j, cell)| { ... })`** — Transforms each `(column_index, byte)`
   into a `char` for the output. The closure receives:
   - `j`: column index (usize)
   - `cell`: the byte value (`u8`)

7. **`if cell == b'*'`** — Check if current cell is a flower. `b'*'` is a byte
   literal (u8). If it's a flower, return `'*'` (the char, not byte)

8. **`count_flowers(garden, i as i32, j as i32)`** — For non-flower cells,
   count adjacent flowers. Cast to `i32` to allow negative offset arithmetic

9. **`0 => ' '`** — If count is 0, return a space (no adjacent flowers)

10. **`n => char::from_digit(n, 10).unwrap()`** — Convert count (1-8) to its
    char representation:
    - `char::from_digit(n, 10)` converts a number to a char in base 10
    - Returns `Some('1')` for 1, `Some('2')` for 2, etc.
    - `.unwrap()` extracts the char (safe here since count is always 0-8)

11. **`.collect()`** (inner) — Collects the iterator of `char`s into a
    `String`. This works because `String` implements `FromIterator<char>`

12. **`.collect()`** (outer) — Collects the iterator of `String`s into a
    `Vec<String>`

### Key differences from imperative style:

| Imperative                         | Functional                             |
| ---------------------------------- | -------------------------------------- |
| `let mut result = Vec::new()`      | No mutable variables needed            |
| `for i in 0..len`                  | `.enumerate()` gives indices           |
| `result.push(row)`                 | `.collect()` builds the collection     |
| `row.push_str(&count.to_string())` | `char::from_digit()` converts directly |

### When to use which?

- **Imperative**: Easier to debug, step through, add logging
- **Functional**: More concise, no mutation, often more "Rusty"

Both are valid. Choose based on readability for your use case.
