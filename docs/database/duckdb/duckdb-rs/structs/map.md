# Map in duckdb - Rust

[Source](about:blank/src/duckdb/row.rs.html#165)
[§](#associatedtype.Error)

The error type.

[Source](about:blank/src/duckdb/row.rs.html#166)
[§](#associatedtype.Item)

The type being iterated over.

[Source](about:blank/src/duckdb/row.rs.html#169-174)
[§](#method.next)

Advances the iterator and returns the next value. [Read more](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/fallible_iterator/trait.FallibleIterator.html#tymethod.next)

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#142)
[§](#method.size_hint)

Returns bounds on the remaining length of the iterator. [Read more](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/fallible_iterator/trait.FallibleIterator.html#method.size_hint)

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#148-150)
[§](#method.count)

Consumes the iterator, returning the number of remaining items.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#157-159)
[§](#method.last)

Returns the last element of the iterator.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#166)
[§](#method.nth)

Returns the `n`th element of the iterator.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#182-184)
[§](#method.step_by)

Returns an iterator starting at the same point, but stepping by the given amount at each iteration. [Read more](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/fallible_iterator/trait.FallibleIterator.html#method.step_by)

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#197-200)
[§](#method.chain)

Returns an iterator which yields the elements of this iterator followed by another.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#212-215)
[§](#method.zip)

Returns an iterator that yields pairs of this iterator’s and another iterator’s values.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#223-226)
[§](#method.map)

Returns an iterator which applies a fallible transform to the elements of the underlying iterator.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#233-236)
[§](#method.for_each)

Calls a fallible closure on each element of an iterator.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#245-248)
[§](#method.filter)

Returns an iterator which uses a predicate to determine which values should be yielded. The predicate may fail; such failures are passed to the caller.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#256-259)
[§](#method.filter_map)

Returns an iterator which both filters and maps. The closure may fail; such failures are passed along to the consumer.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#267-269)
[§](#method.enumerate)

Returns an iterator which yields the current iteration count as well as the value.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#277-279)
[§](#method.peekable)

Returns an iterator that can peek at the next element without consuming it.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#289-292)
[§](#method.skip_while)

Returns an iterator that skips elements based on a predicate.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#303-306)
[§](#method.take_while)

Returns an iterator that yields elements based on a predicate.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#317-319)
[§](#method.skip)

Returns an iterator which skips the first `n` values of this iterator.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#327-329)
[§](#method.take)

Returns an iterator that yields only the first `n` values of this iterator.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#340-343)
[§](#method.scan)

Returns an iterator which applies a stateful map to values of this iterator.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#354-358)
[§](#method.flat_map)

Returns an iterator which maps this iterator’s elements to iterators, yielding those iterators’ values.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#368-371)
[§](#method.flatten)

Returns an iterator which flattens an iterator of iterators, yielding those iterators’ values.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#386-388)
[§](#method.fuse)

Returns an iterator which yields this iterator’s elements and ends after the first `Ok(None)`. [Read more](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/fallible_iterator/trait.FallibleIterator.html#method.fuse)

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#398-401)
[§](#method.inspect)

Returns an iterator which passes each element to a closure before returning it.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#411-413)
[§](#method.by_ref)

Borrow an iterator rather than consuming it. [Read more](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/fallible_iterator/trait.FallibleIterator.html#method.by_ref)

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#422-425)
[§](#method.collect)

Transforms the iterator into a collection. [Read more](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/fallible_iterator/trait.FallibleIterator.html#method.collect)

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#432-436)
[§](#method.partition)

Transforms the iterator into two collections, partitioning elements by a closure.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#456-459)
[§](#method.fold)

Applies a function over the elements of the iterator, producing a single final value.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#468-472)
[§](#method.try_fold)

Applies a function over the elements of the iterator, producing a single final value. [Read more](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/fallible_iterator/trait.FallibleIterator.html#method.try_fold)

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#482-485)
[§](#method.all)

Determines if all elements of this iterator match a predicate.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#499-502)
[§](#method.any)

Determines if any element of this iterator matches a predicate.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#516-519)
[§](#method.find)

Returns the first element of the iterator that matches a predicate.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#533-536)
[§](#method.find_map)

Applies a function to the elements of the iterator, returning the first non-`None` result.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#545-548)
[§](#method.position)

Returns the position of the first element of this iterator that matches a predicate. The predicate may fail; such failures are returned to the caller.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#562-565)
[§](#method.max)

Returns the maximal element of the iterator.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#573-577)
[§](#method.max_by_key)

Returns the element of the iterator which gives the maximum value from the function.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#597-600)
[§](#method.max_by)

Returns the element that gives the maximum value with respect to the function.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#619-622)
[§](#method.min)

Returns the minimal element of the iterator.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#630-634)
[§](#method.min_by_key)

Returns the element of the iterator which gives the minimum value from the function.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#654-657)
[§](#method.min_by)

Returns the element that gives the minimum value with respect to the function.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#686-690)
[§](#method.unzip)

Converts an iterator of pairs into a pair of containers.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#706-709)
[§](#method.cloned)

Returns an iterator which clones all of its elements.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#729-733)
[§](#method.cmp)

Lexicographically compares the elements of this iterator to that of another.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#753-757)
[§](#method.partial_cmp)

Lexicographically compares the elements of this iterator to that of another.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#777-781)
[§](#method.eq)

Determines if the elements of this iterator are equal to those of another.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#801-805)
[§](#method.ne)

Determines if the elements of this iterator are not equal to those of another.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#825-829)
[§](#method.lt)

Determines if the elements of this iterator are lexicographically less than those of another.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#851-855)
[§](#method.le)

Determines if the elements of this iterator are lexicographically less than or equal to those of another.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#877-881)
[§](#method.gt)

Determines if the elements of this iterator are lexicographically greater than those of another.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#903-907)
[§](#method.ge)

Determines if the elements of this iterator are lexicographically greater than or equal to those of another.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#928-930)
[§](#method.iterator)

Returns a normal (non-fallible) iterator over `Result<Item, Error>`.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#938-941)
[§](#method.map_err)

Returns an iterator which applies a transform to the errors of the underlying iterator.

[Source](https://docs.rs/fallible-iterator/0.3.0/x86_64-unknown-linux-gnu/src/fallible_iterator/lib.rs.html#948-951)
[§](#method.unwrap)

Returns an iterator which unwraps all of its elements.
