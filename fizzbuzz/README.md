# Stupid ways to write FizzBuzz

## Simple FizzBuzz

[v1_stupid.rs](src/v1_stupid.rs) - Just a regular Joe's FizzBuzz implementation.

## A better one with Vector allocation optimized

[v2_slightly_optimized.rs](src/v2_slightly_optimized.rs) -
Uses `with_capacity()` to preallocate the vector

The usage of `String`s is arguable, that's why the Enum is used in later implementations, 
leaving the String representation implementation to users.

## Iterator

[v3_iterator.rs](src/v3_iterator.rs) -
If you need the FizzBuzz iterator for some reason.

Current limitations: This iterator is rather the iterator object, 
hence it returns `T` instead of `&T` 

## Lazy BTree
[v4_lazy_btree.rs](src/v4_lazy_btree.rs) - I'm not actually sure if it is lazy enough, 
but if it's needed to iterate over the cached value, this implementation should fit.

Current limitations: a lazy b-tree iterator over cached values iterates over `&T`, 
and this is  the correct implementation. Unfortunately, the iterator itself iterates 
over `T`. This should be fixed.