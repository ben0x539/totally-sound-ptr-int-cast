# totally sound ptr&lt;-&gt; int casts

## doom is upon us

Okay, so, it looks like `as`-casts between `usize` and raw pointer types are
on the way out. People are talking about how those make provenance really
annoying which is bad for formalizing a memory model. In this hypothetical
future, it would be highly illegal to cast a pointer to an integer, maybe
roundtrip it through some OS or FFI API, cast it back to an integer, and then
dereference it.

It may be too late to avert that terrible fate. In this doomed timeline, it
may become necessary to use this crate to perform totally sound
ptr&lt;-&gt;int casts.

```rust
let n = totally_sound_ptr_int_cast::ptr2int(
    Bxo::into_raw(your_boxed_stuff.into_raw()));
```

and, at any later point within the same program execution,

```rust
let p = totally_sound_ptr_int_cast::int2ptr_mut(n);
unsafe { println!("{:?}", &*p); } // or whatever
```

you get your stuff back.
