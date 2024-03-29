# mango-util-mem

This crate provides tools for measuring the heap memory usage of specific structures.

## Annotating types with `MallocSizeOf` trait

To measure your struct's memory usage, it and all of its child types must implement the `MallocSizeOf` trait.

For types that are local to your crate, this is really easy. Just add:

```rust
#[derive(MallocSizeOf)]
```

Note that `size_of` should return only the number of **heap-allocated bytes** used by the type. For example, a type such as `struct MyStruct([u8; 128])` would return **zero**, not 128. Recursive accounting for heap-allocated memory when your struct is part of e.g. a `Vec` or `HashMap` is already taken care of by the implementations of `MallocSizeOf` on those collection types.

Oftentimes, the public interface of the type you are measuring does not provide enough information to precisely measure the amount of heap space it allocates. In that case, you can try just to produce a reasonable estimate.

## Measuring memory usage

To compute the heap usage of an annotated type at runtime, simply call `mango_util_mem::malloc_size(&my_struct)`. For complete memory usage, add in the inline size of the type as well, as in:

```rust
mango_util_mem::malloc_size(&my_struct) + std::mem::size_of::<MyStruct>()
```
