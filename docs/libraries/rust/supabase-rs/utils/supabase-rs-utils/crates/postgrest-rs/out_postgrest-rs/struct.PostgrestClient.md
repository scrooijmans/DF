# Struct PostgrestClient Copy item path

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/src/supabase_rust_postgrest/lib.rs.html#144-157" class="src">Source</a>

``` rust
pub struct PostgrestClient { /* private fields */ }
```

Expand description

PostgreST クライアント

## Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#impl-PostgrestClient" class="anchor">§</a>

### impl <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html" class="struct" title="struct supabase_rust_postgrest::PostgrestClient">PostgrestClient</a>

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.new" class="fn">new</a>( base_url: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, api_key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, http_client: <a href="https://docs.rs/reqwest/0.11.27/x86_64-unknown-linux-gnu/reqwest/async_impl/client/struct.Client.html" class="struct" title="struct reqwest::async_impl::client::Client">Client</a>, ) -\> Self

新しい PostgreST クライアントを作成

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.rpc" class="fn">rpc</a>( base_url: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, api_key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, function_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, params: <a href="https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html" class="enum" title="enum serde_json::value::Value">Value</a>, http_client: <a href="https://docs.rs/reqwest/0.11.27/x86_64-unknown-linux-gnu/reqwest/async_impl/client/struct.Client.html" class="struct" title="struct reqwest::async_impl::client::Client">Client</a>, ) -\> Self

RPCリクエストを作成

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.with_header" class="fn">with_header</a>(self, key: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

ヘッダーを追加

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.with_auth" class="fn">with_auth</a>(self, token: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

認証トークンを設定

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.select" class="fn">select</a>(self, columns: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

取得するカラムを指定

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.inner_join" class="fn">inner_join</a>( self, foreign_table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, foreign_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> Self

結合クエリ: 参照テーブルとの内部結合

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.left_join" class="fn">left_join</a>( self, foreign_table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, foreign_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> Self

結合クエリ: 参照テーブルとの左外部結合

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.include" class="fn">include</a>( self, foreign_table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_foreign_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, columns: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> Self

結合クエリ: 一対多関係の子テーブルを含める

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.referenced_by" class="fn">referenced_by</a>(self, foreign_table: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, foreign_column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

結合クエリ: 外部キーの参照先テーブルを含める

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.eq" class="fn">eq</a>(self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

等価フィルター

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.gt" class="fn">gt</a>(self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

より大きいフィルター

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.gte" class="fn">gte</a>(self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

以上フィルター

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.lt" class="fn">lt</a>(self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

より小さいフィルター

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.lte" class="fn">lte</a>(self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

以下フィルター

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.like" class="fn">like</a>(self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, pattern: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

LIKE フィルター

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.ilike" class="fn">ilike</a>(self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, pattern: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

ILIKE フィルター（大文字小文字を区別しない）

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.in_list" class="fn">in_list</a>(self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, values: &\[&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\]) -\> Self

IN フィルター

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.not" class="fn">not</a>(self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, operator_with_value: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

NOT フィルター

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.contains" class="fn">contains</a>( self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html" class="enum" title="enum serde_json::value::Value">Value</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

JSON/JSONB カラムが指定した値を含むか (`cs`, `@>`) フィルター value は serde_json::Value で指定します

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.contained_by" class="fn">contained_by</a>( self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, value: &<a href="https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html" class="enum" title="enum serde_json::value::Value">Value</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<Self, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

JSON/JSONB カラムが指定した値に含まれるか (`cd`, `<@`) フィルター value は serde_json::Value で指定します

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.order" class="fn">order</a>(self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, order: <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html" class="enum" title="enum supabase_rust_postgrest::SortOrder">SortOrder</a>) -\> Self

ソート順を指定

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.limit" class="fn">limit</a>(self, count: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

取得件数を制限

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.offset" class="fn">offset</a>(self, count: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> Self

オフセットを指定

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.text_search" class="fn">text_search</a>( self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, query: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, config: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>, ) -\> Self

全文検索

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.geo_distance" class="fn">geo_distance</a>( self, column: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, lat: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, lng: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, distance: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, unit: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> Self

地理空間データの距離ベース検索

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.group_by" class="fn">group_by</a>(self, columns: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

グループ化

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.count" class="fn">count</a>(self, exact: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

行数カウント

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.ignore_rls" class="fn">ignore_rls</a>(self) -\> Self

RLS（行レベルセキュリティ）ポリシーを無視

#### pub fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.schema" class="fn">schema</a>(self, schema_name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

スキーマを指定（デフォルトのpublicスキーマではない場合）

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.export_csv" class="fn">export_csv</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

CSVとしてデータをエクスポート

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.execute" class="fn">execute</a>\<T: for\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\>\>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<T\>, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

データを取得

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.insert" class="fn">insert</a>\<T: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a>\>( &self, values: T, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html" class="enum" title="enum serde_json::value::Value">Value</a>, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

データを挿入

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.update" class="fn">update</a>\<T: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a>\>( &self, values: T, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html" class="enum" title="enum serde_json::value::Value">Value</a>, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

データを更新

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.delete" class="fn">delete</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/serde_json/1.0.140/x86_64-unknown-linux-gnu/serde_json/value/enum.Value.html" class="enum" title="enum serde_json::value::Value">Value</a>, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

データを削除

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.call_rpc" class="fn">call_rpc</a>\<T: for\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\>\>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<T, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

RPC関数を呼び出す (POSTリクエスト)

#### pub async fn <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#method.begin_transaction" class="fn">begin_transaction</a>( &self, isolation_level: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.IsolationLevel.html" class="enum" title="enum supabase_rust_postgrest::IsolationLevel">IsolationLevel</a>\>, transaction_mode: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.TransactionMode.html" class="enum" title="enum supabase_rust_postgrest::TransactionMode">TransactionMode</a>\>, timeout_seconds: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html" class="struct" title="struct supabase_rust_postgrest::PostgrestTransaction">PostgrestTransaction</a>, <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>\>

トランザクションを開始

## Auto Trait Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html#blanket-implementations" class="anchor">§</a>
