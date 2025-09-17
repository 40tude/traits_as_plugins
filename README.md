The associated [blog post](https://www.40tude.fr/docs/06_programmation/rust/015_traits/traits_00.html).


---
## How to name a trait ? (by Chat GPT)

## Do / Don’t cheatsheet

**Do**

* Name traits with **nouns** (capabilities) or **adjectives** (markers).
* Name methods with **clear verbs** in `snake_case`.
* Use `as_/to_/into_` consistently for borrowing/owning/consuming conversions.
* Use `try_*` for fallible operations returning `Result`.
* Use `*Ext` for extension traits that add convenience APIs.
* Keep traits focused and methods symmetric.

**Don’t**

* Don’t use `-able` suffixes by default (`Readable`, `Iterable`) — unidiomatic.
* Don’t prefix getters with `get_` unless signaling *checked lookup* (`get`, `get_mut`).
* Don’t repeat the trait name inside method names (avoid stutter).
* Don’t pack unrelated behaviors into one “mega-trait”.

---

## Practical naming templates

* Capability: `Read`, `Write`, `Render`, `Encode`, `Decode`, `Validate`, `Persist`
* Marker: `Clone`, `Copy`, `Debug`, `Send`, `Sync`, `Sized`
* Extension: `FooExt`, `IteratorExt`, `HttpExt`
* Methods: `read`, `write`, `render`, `encode`, `decode`, `validate`, `persist`, `open`, `close`, `start`, `stop`
* Conversions: `as_x`, `to_x`, `into_x`
* Fallible: `try_x`
* Checked access: `get`, `get_mut`




---


## Detailed explanations

### 1) Prefer short **nouns** (or noun-like) for capabilities

* Good: `Read`, `Write`, `Display`, `Iterator`, `Borrow`, `AsRef`, `Deref`, `Drop`, `Hash`, `Eq`, `Ord`
* Avoid: `Readable`, `Iterable`, `Hashable` (the `-able` suffix is rare in Rust)

### 2) Use **adjectives** for marker traits (no required methods)

* Good: `Send`, `Sync`, `Sized`, `Unpin`, `Clone`, `Copy`, `Debug`
* These communicate a property/type marker rather than a behavior API.

### 3) Use `*Ext` for **extension traits**

* Example: `IteratorExt`, `HttpClientExt`
* These add convenience methods to an existing type/trait without forcing blanket impls in user code.

### 4) Use `*Mut` when the trait clearly implies mutability compared to a base trait

* Example: `Borrow` vs `BorrowMut`.

### 5) Avoid prefixing the trait name in its methods

* If the trait is `Read`, the method should be `read`, not `read_read` or `read_from_reader`.

### 6) Keep the **scope tight**

* A trait should express a single responsibility (coherent “capability”). If you feel you need many unrelated methods, split the trait.

---

## How to name trait methods

### 1) Methods are **verbs** in `snake_case`

* `read`, `write`, `flush`, `render`, `validate`, `persist`, `fetch_next`

### 2) Use the standard conversion triad carefully

* `as_*` — cheap borrow/view (no allocation), e.g., `as_str`
* `to_*` — cheap if possible, otherwise **clones/allocates** to produce an owned value, e.g., `to_string`
* `into_*` — **consumes** `self` to produce a new owned value, e.g., `into_string`

### 3) Use `try_*` for fallible actions returning `Result<_, E>`

* e.g., `try_reserve`, `try_parse`

### 4) Getter conventions

* Simple accessors: **no `get_` prefix** — `len()`, `capacity()`, `is_empty()`
* Use `get` when it signals *checked lookup* (often returning `Option`): `get(index)`, `get_mut(index)`

### 5) Mutability reveals intent

* Non-mutable view: `as_*`
* Mutable view: `as_*_mut` or `get_mut`
* Consuming: `into_*`

### 6) Pairs and symmetry help

* If you have `start()` then usually also `stop()`/`shutdown()`
* If you have `open()` then usually `close()`
* Keep names parallel across methods.

### 7) Avoid stutter

* If the trait is `Render`, method should be `render()`, not `render_render()` or `do_render()`.

---

## Associated items inside traits

* Associated types: short, generic, and conventional: `type Item`, `type Error`
* Type parameters: `T`, `U`, `E`, `K`, `V` (domain-specific names are OK if clearer)
* Associated consts: `UPPER_SNAKE_CASE`

---

## Small examples

### Capability trait (noun) + verb methods

```rust
pub trait Render {
    /// Render into the provided target. Returns the number of bytes written.
    fn render(&self, target: &mut Vec<u8>) -> Result<usize, RenderError>;

    /// Render directly to stdout as a convenience.
    fn render_stdout(&self) -> Result<(), RenderError>;
}
```

### Marker trait (adjective) with no methods

```rust
/// Types that are safe to persist as-is without redaction.
/// Marker trait: no required methods.
pub trait PersistSafe {}
```

### Extension trait pattern

```rust
/// Extra convenience methods on `Iterator`.
pub trait IteratorExt: Iterator {
    /// Collect into a `Vec` and assert non-empty.
    fn collect_non_empty(self) -> Result<Vec<Self::Item>, &'static str>
    where
        Self: Sized,
    {
        // NOTE: Uses standard semantics: consumes `self`.
        let v: Vec<_> = self.collect();
        if v.is_empty() { Err("empty") } else { Ok(v) }
    }
}

impl<I: Iterator> IteratorExt for I {}
```

### Conversions (as\_/to\_/into\_)

```rust
pub trait AsBytes {
    /// Borrowing view, no allocation.
    fn as_bytes(&self) -> &[u8];
}

pub trait ToBytes {
    /// May allocate/clone to produce owned bytes.
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait IntoBytes {
    /// Consumes `self` to produce owned bytes.
    fn into_bytes(self) -> Vec<u8>;
}
```

