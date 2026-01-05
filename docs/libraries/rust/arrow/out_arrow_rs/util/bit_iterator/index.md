# Module bit_iterator Copy item path

<a href="https://docs.rs/arrow-buffer/56.2.0/x86_64-unknown-linux-gnu/src/arrow_buffer/util/mod.rs.html#19" class="src">Source</a>

Expand description

Types for iterating over packed bitmasks

## Structs<a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIndexIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitIndexIterator">BitIndexIterator</a>  
An iterator of `usize` whose index in a provided bitmask is true

<a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIndexU32Iterator.html" class="struct" title="struct arrow::util::bit_iterator::BitIndexU32Iterator">BitIndexU32Iterator</a>  
An iterator of u32 whose index in a provided bitmask is true Respects arbitrary offsets and slice lead/trail padding exactly like BitIndexIterator

<a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitIterator">BitIterator</a>  
Iterator over the bits within a packed bitmask

<a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/struct.BitSliceIterator.html" class="struct" title="struct arrow::util::bit_iterator::BitSliceIterator">BitSliceIterator</a>  
Iterator of contiguous ranges of set bits within a provided packed bitmask

## Functions<a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/bit_iterator/fn.try_for_each_valid_idx.html" class="fn" title="fn arrow::util::bit_iterator::try_for_each_valid_idx">try_for_each_valid_idx</a>  
Calls the provided closure for each index in the provided null mask that is set, using an adaptive strategy based on the null count
