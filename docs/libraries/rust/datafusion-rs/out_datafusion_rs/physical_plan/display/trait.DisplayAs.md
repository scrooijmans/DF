# Trait DisplayAs Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/display.rs.html#1023" class="src">Source</a>

``` rust
pub trait DisplayAs {
    // Required method
    fn fmt_as(
        &self,
        t: DisplayFormatType,
        f: &mut Formatter<'_>,
    ) -> Result<(), Error>;
}
```

Expand description

Trait for types which could have additional details when formatted in `Verbose` mode

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#tymethod.fmt_as" class="fn">fmt_as</a>( &self, t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Format according to `DisplayFormatType`, used when verbose representation looks different from the default one

Should not include a newline

## Implementations on Foreign Types<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-FileGroupDisplay%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource/display/struct.FileGroupDisplay.html" class="struct" title="struct datafusion_datasource::display::FileGroupDisplay">FileGroupDisplay</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#method.fmt_as" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#tymethod.fmt_as" class="fn">fmt_as</a>( &self, t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-MemSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion-datasource/50.2.0/x86_64-unknown-linux-gnu/datafusion_datasource/memory/struct.MemSink.html" class="struct" title="struct datafusion_datasource::memory::MemSink">MemSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#method.fmt_as-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#tymethod.fmt_as" class="fn">fmt_as</a>( &self, t: <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/enum.DisplayFormatType.html" class="enum" title="enum datafusion::physical_plan::DisplayFormatType">DisplayFormatType</a>, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-CsvSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/csv/struct.CsvSink.html" class="struct" title="struct datafusion::datasource::file_format::csv::CsvSink">CsvSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-JsonSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSink.html" class="struct" title="struct datafusion::datasource::file_format::json::JsonSink">JsonSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-ParquetSink" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/parquet/struct.ParquetSink.html" class="struct" title="struct datafusion::datasource::file_format::parquet::ParquetSink">ParquetSink</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-DataSourceExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/memory/struct.DataSourceExec.html" class="struct" title="struct datafusion::datasource::memory::DataSourceExec">DataSourceExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-FileScanConfig" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileScanConfig.html" class="struct" title="struct datafusion::datasource::physical_plan::FileScanConfig">FileScanConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-DataSinkExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/sink/struct.DataSinkExec.html" class="struct" title="struct datafusion::datasource::sink::DataSinkExec">DataSinkExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-OutputRequirementExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_optimizer/output_requirements/struct.OutputRequirementExec.html" class="struct" title="struct datafusion::physical_optimizer::output_requirements::OutputRequirementExec">OutputRequirementExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-AggregateExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/aggregates/struct.AggregateExec.html" class="struct" title="struct datafusion::physical_plan::aggregates::AggregateExec">AggregateExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-AnalyzeExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/analyze/struct.AnalyzeExec.html" class="struct" title="struct datafusion::physical_plan::analyze::AnalyzeExec">AnalyzeExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-AsyncFuncExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/async_func/struct.AsyncFuncExec.html" class="struct" title="struct datafusion::physical_plan::async_func::AsyncFuncExec">AsyncFuncExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-CoalesceBatchesExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_batches/struct.CoalesceBatchesExec.html" class="struct" title="struct datafusion::physical_plan::coalesce_batches::CoalesceBatchesExec">CoalesceBatchesExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-CoalescePartitionsExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coalesce_partitions/struct.CoalescePartitionsExec.html" class="struct" title="struct datafusion::physical_plan::coalesce_partitions::CoalescePartitionsExec">CoalescePartitionsExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-CooperativeExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeExec.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeExec">CooperativeExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-EmptyExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/empty/struct.EmptyExec.html" class="struct" title="struct datafusion::physical_plan::empty::EmptyExec">EmptyExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-ExplainExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/explain/struct.ExplainExec.html" class="struct" title="struct datafusion::physical_plan::explain::ExplainExec">ExplainExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-FilterExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/filter/struct.FilterExec.html" class="struct" title="struct datafusion::physical_plan::filter::FilterExec">FilterExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-CrossJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.CrossJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::CrossJoinExec">CrossJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-HashJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.HashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::HashJoinExec">HashJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-NestedLoopJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.NestedLoopJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::NestedLoopJoinExec">NestedLoopJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-SortMergeJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SortMergeJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SortMergeJoinExec">SortMergeJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-SymmetricHashJoinExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/joins/struct.SymmetricHashJoinExec.html" class="struct" title="struct datafusion::physical_plan::joins::SymmetricHashJoinExec">SymmetricHashJoinExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-GlobalLimitExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/struct.GlobalLimitExec.html" class="struct" title="struct datafusion::physical_plan::limit::GlobalLimitExec">GlobalLimitExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-LocalLimitExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/struct.LocalLimitExec.html" class="struct" title="struct datafusion::physical_plan::limit::LocalLimitExec">LocalLimitExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-LazyMemoryExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryExec.html" class="struct" title="struct datafusion::physical_plan::memory::LazyMemoryExec">LazyMemoryExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-PlaceholderRowExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/placeholder_row/struct.PlaceholderRowExec.html" class="struct" title="struct datafusion::physical_plan::placeholder_row::PlaceholderRowExec">PlaceholderRowExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-ProjectionExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/projection/struct.ProjectionExec.html" class="struct" title="struct datafusion::physical_plan::projection::ProjectionExec">ProjectionExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-RecursiveQueryExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/recursive_query/struct.RecursiveQueryExec.html" class="struct" title="struct datafusion::physical_plan::recursive_query::RecursiveQueryExec">RecursiveQueryExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-RepartitionExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/repartition/struct.RepartitionExec.html" class="struct" title="struct datafusion::physical_plan::repartition::RepartitionExec">RepartitionExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-PartialSortExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/partial_sort/struct.PartialSortExec.html" class="struct" title="struct datafusion::physical_plan::sorts::partial_sort::PartialSortExec">PartialSortExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-SortExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort/struct.SortExec.html" class="struct" title="struct datafusion::physical_plan::sorts::sort::SortExec">SortExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-SortPreservingMergeExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/sorts/sort_preserving_merge/struct.SortPreservingMergeExec.html" class="struct" title="struct datafusion::physical_plan::sorts::sort_preserving_merge::SortPreservingMergeExec">SortPreservingMergeExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-StreamingTableExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/struct.StreamingTableExec.html" class="struct" title="struct datafusion::physical_plan::streaming::StreamingTableExec">StreamingTableExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-BarrierExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.BarrierExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::BarrierExec">BarrierExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-BlockingExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.BlockingExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::BlockingExec">BlockingExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-ErrorExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.ErrorExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::ErrorExec">ErrorExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-MockExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.MockExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::MockExec">MockExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-PanicExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.PanicExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::PanicExec">PanicExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-StatisticsExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.StatisticsExec.html" class="struct" title="struct datafusion::physical_plan::test::exec::StatisticsExec">StatisticsExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-TestMemoryExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/struct.TestMemoryExec.html" class="struct" title="struct datafusion::physical_plan::test::TestMemoryExec">TestMemoryExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-InterleaveExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/struct.InterleaveExec.html" class="struct" title="struct datafusion::physical_plan::union::InterleaveExec">InterleaveExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-UnionExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/union/struct.UnionExec.html" class="struct" title="struct datafusion::physical_plan::union::UnionExec">UnionExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-UnnestExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/unnest/struct.UnnestExec.html" class="struct" title="struct datafusion::physical_plan::unnest::UnnestExec">UnnestExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-BoundedWindowAggExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.BoundedWindowAggExec.html" class="struct" title="struct datafusion::physical_plan::windows::BoundedWindowAggExec">BoundedWindowAggExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-WindowAggExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/windows/struct.WindowAggExec.html" class="struct" title="struct datafusion::physical_plan::windows::WindowAggExec">WindowAggExec</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/display/trait.DisplayAs.html#impl-DisplayAs-for-WorkTableExec" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.DisplayAs.html" class="trait" title="trait datafusion::physical_plan::DisplayAs">DisplayAs</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/work_table/struct.WorkTableExec.html" class="struct" title="struct datafusion::physical_plan::work_table::WorkTableExec">WorkTableExec</a>
