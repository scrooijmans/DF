# Type Alias Reader Copy item path

<a href="https://arrow.apache.org/rust/src/arrow_csv/reader/mod.rs.html#444" class="src">Source</a>

``` rust
pub type Reader<R> = BufReader<BufReader<R>>;
```

Expand description

CSV file reader using [`std::io::BufReader`](https://doc.rust-lang.org/nightly/std/io/buffered/bufreader/struct.BufReader.html "struct std::io::buffered::bufreader::BufReader")

## Aliased Type<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html#aliased-type" class="anchor">Â§</a>

``` rust
pub struct Reader<R> {
    reader: BufReader<R>,
    decoder: Decoder,
}
```

## Fields<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html#fields" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html#structfield.reader" class="anchor field">Â§</a>`reader: `<a href="https://doc.rust-lang.org/nightly/std/io/buffered/bufreader/struct.BufReader.html" class="struct" title="struct std::io::buffered::bufreader::BufReader"><code>BufReader</code></a>`<R>`

File reader

<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html#structfield.decoder" class="anchor field">Â§</a>`decoder: `<a href="https://arrow.apache.org/rust/arrow_csv/reader/struct.Decoder.html" class="struct" title="struct arrow_csv::reader::Decoder"><code>Decoder</code></a>

The decoder

## Implementations<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html#implementations" class="anchor">Â§</a>

<a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html#impl-BufReader%3CBufReader%3CR%3E%3E" class="anchor">Â§</a>

### impl\<R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>\> <a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html" class="type" title="type arrow_csv::reader::Reader">Reader</a>\<R\>

#### pub fn <a href="https://arrow.apache.org/rust/arrow_csv/reader/type.Reader.html#method.schema" class="fn">schema</a>(&self) -\> SchemaRef

Returns the schema of the reader, useful for getting the schema without reading record batches
