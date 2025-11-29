# Rust Iterator Notes

## Consuming Iterators

Iterators in Rust are **lazy** - adapter methods like `inspect()`, `map()`, `filter()` return another iterator that does nothing until consumed by a terminal operation.

### Terminal operations that consume iterators:

```rust
// for_each - most common for side effects
first_list.into_iter().inspect(|x| {
    dbg!(x);
}).for_each(drop);

// count() - consumes and counts elements
first_list.into_iter().inspect(|x| {
    dbg!(x);
}).count();

// collect() - if you need the result
let _: Vec<_> = first_list.into_iter().inspect(|x| {
    dbg!(x);
}).collect();
```

### Common mistakes:

1. **Calling `.to_owned()` on an iterator** - doesn't do what you expect
2. **Using `dbg!("{x}")` instead of `dbg!(x)`** - prints the literal string `"{x}"`, not the value

### Simplest pattern for debugging:

```rust
first_list.into_iter().for_each(|x| {
    dbg!(x);
});
```

## Iterator Types and Dereferencing

When iterating over slices, the yielded type depends on the method:

| Method | On `&[i32]` | Yields |
|--------|-------------|--------|
| `.iter()` | borrows | `&i32` |
| `.iter().copied()` | borrows + copies | `i32` |
| `.into_iter()` | on `&[T]` same as iter | `&i32` |

### Getting `i32` from `&i32`:

```rust
// Option 1: .copied() - for Copy types like i32
first_list.iter().copied().for_each(|x| {
    dbg!(x);  // x is i32
});

// Option 2: Pattern matching in closure
first_list.iter().for_each(|&x| {
    dbg!(x);  // x is i32 via destructuring
});

// Option 3: Explicit dereference
first_list.iter().for_each(|x| {
    dbg!(*x);  // dereference with *
});
```

### Why `&&i32`?

If you're seeing `&&i32`, you likely have:
- A `&[&i32]` (slice of references)
- Or calling `.iter()` on something already borrowed twice

Use `.copied()` twice or chain dereferences as needed.

## Collecting Iterators into Collections

### Can't collect into borrowed types

```rust
// WRONG - can't collect into a borrowed slice
.collect::<&[bool]>();  // compile error!

// CORRECT - collect into owned Vec
.collect::<Vec<bool>>();

// Or let type inference figure it out
.collect::<Vec<_>>();
```

**Why?** `&[bool]` is a *borrow* of existing memory - you can't create new data into it. `Vec<bool>` *owns* the memory and can be created from an iterator.

### Example: Create a list of bools from comparison

```rust
let results: Vec<bool> = first_list
    .iter()
    .copied()
    .map(|x| second_list.contains(&x))
    .collect();
```

### Alternatives to collecting

```rust
// Check if all are true (returns single bool)
.all(|b| b)

// Check if any are true
.any(|b| b)

// Count how many are true
.filter(|&b| b).count()
```

## Handling Option in Iterator Pipelines

When you have an `Option<T>` in a pipeline and want if-else style branching:

### `map_or` - most idiomatic

```rust
// If Some(x), run comparison; if None, return false
second_list.get(i).map_or(false, |&x| x == val)
```

### `is_some_and` (Rust 1.70+)

```rust
second_list.get(i).is_some_and(|&x| x == val)
```

### `map` + `unwrap_or`

```rust
second_list.get(i).map(|&x| x == val).unwrap_or(false)
```

### Match in closure

```rust
.map(|(i, val)| match second_list.get(i) {
    Some(&x) => x == val,
    None => false,
})
```

### Full example: Check all elements match

```rust
let all_match: bool = first_list
    .iter()
    .copied()
    .enumerate()
    .all(|(i, val)| second_list.get(i).map_or(false, |&x| x == val));
```

`.all()` short-circuits and returns `false` as soon as any element doesn't match.
