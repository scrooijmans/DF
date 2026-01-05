# Crate supabase_rust_postgrest Copy item path

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/src/supabase_rust_postgrest/lib.rs.html#1-2221" class="src">Source</a>

Expand description

Supabase PostgREST client for Rust

This crate provides database functionality for Supabase, allowing for querying, filtering, and manipulating data in PostgreSQL.

## <a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/index.html#features" class="doc-anchor">§</a>Features

- Query API (`select`, `insert`, `update`, `delete`)
- Filtering (`eq`, `gt`, `lt`, etc.)
- Ordering and pagination
- Transactions
- RPC function calls
- CSV export

## Structs<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestApiErrorDetails.html" class="struct" title="struct supabase_rust_postgrest::PostgrestApiErrorDetails">PostgrestApiErrorDetails</a>  
PostgREST APIエラーの詳細情報

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestClient.html" class="struct" title="struct supabase_rust_postgrest::PostgrestClient">PostgrestClient</a>  
PostgreST クライアント

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/struct.PostgrestTransaction.html" class="struct" title="struct supabase_rust_postgrest::PostgrestTransaction">PostgrestTransaction</a>  
トランザクションクライアント

## Enums<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.IsolationLevel.html" class="enum" title="enum supabase_rust_postgrest::IsolationLevel">IsolationLevel</a>  
トランザクションの分離レベル

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.PostgrestError.html" class="enum" title="enum supabase_rust_postgrest::PostgrestError">PostgrestError</a>  
エラー型

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.SortOrder.html" class="enum" title="enum supabase_rust_postgrest::SortOrder">SortOrder</a>  
ソート方向

<a href="https://docs.rs/supabase-rust-postgrest/0.4.0/supabase_rust_postgrest/enum.TransactionMode.html" class="enum" title="enum supabase_rust_postgrest::TransactionMode">TransactionMode</a>  
トランザクションの読み取り/書き込みモード
