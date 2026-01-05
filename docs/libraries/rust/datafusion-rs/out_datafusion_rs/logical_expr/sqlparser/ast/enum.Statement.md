# Enum Statement Copy item path

<a href="https://docs.rs/sqlparser/0.58.0/x86_64-unknown-linux-gnu/src/sqlparser/ast/mod.rs.html#3086" class="src">Source</a>

``` rust
pub enum Statement {
Show 111 variants    Analyze {
        table_name: ObjectName,
        partitions: Option<Vec<Expr>>,
        for_columns: bool,
        columns: Vec<Ident>,
        cache_metadata: bool,
        noscan: bool,
        compute_statistics: bool,
        has_table_keyword: bool,
    },
    Set(Set),
    Truncate {
        table_names: Vec<TruncateTableTarget>,
        partitions: Option<Vec<Expr>>,
        table: bool,
        identity: Option<TruncateIdentityOption>,
        cascade: Option<CascadeOption>,
        on_cluster: Option<Ident>,
    },
    Msck {
        table_name: ObjectName,
        repair: bool,
        partition_action: Option<AddDropSync>,
    },
    Query(Box<Query>),
    Insert(Insert),
    Install {
        extension_name: Ident,
    },
    Load {
        extension_name: Ident,
    },
    Directory {
        overwrite: bool,
        local: bool,
        path: String,
        file_format: Option<FileFormat>,
        source: Box<Query>,
    },
    Case(CaseStatement),
    If(IfStatement),
    While(WhileStatement),
    Raise(RaiseStatement),
    Call(Function),
    Copy {
        source: CopySource,
        to: bool,
        target: CopyTarget,
        options: Vec<CopyOption>,
        legacy_options: Vec<CopyLegacyOption>,
        values: Vec<Option<String>>,
    },
    CopyIntoSnowflake {Show 14 fields
        kind: CopyIntoSnowflakeKind,
        into: ObjectName,
        into_columns: Option<Vec<Ident>>,
        from_obj: Option<ObjectName>,
        from_obj_alias: Option<Ident>,
        stage_params: StageParamsObject,
        from_transformations: Option<Vec<StageLoadSelectItemKind>>,
        from_query: Option<Box<Query>>,
        files: Option<Vec<String>>,
        pattern: Option<String>,
        file_format: KeyValueOptions,
        copy_options: KeyValueOptions,
        validation_mode: Option<String>,
        partition: Option<Box<Expr>>,
    },
    Open(OpenStatement),
    Close {
        cursor: CloseCursor,
    },
    Update {
        table: TableWithJoins,
        assignments: Vec<Assignment>,
        from: Option<UpdateTableFromKind>,
        selection: Option<Expr>,
        returning: Option<Vec<SelectItem>>,
        or: Option<SqliteOnConflict>,
    },
    Delete(Delete),
    CreateView {Show 14 fields
        or_alter: bool,
        or_replace: bool,
        materialized: bool,
        name: ObjectName,
        columns: Vec<ViewColumnDef>,
        query: Box<Query>,
        options: CreateTableOptions,
        cluster_by: Vec<Ident>,
        comment: Option<String>,
        with_no_schema_binding: bool,
        if_not_exists: bool,
        temporary: bool,
        to: Option<ObjectName>,
        params: Option<CreateViewParams>,
    },
    CreateTable(CreateTable),
    CreateVirtualTable {
        name: ObjectName,
        if_not_exists: bool,
        module_name: Ident,
        module_args: Vec<Ident>,
    },
    CreateIndex(CreateIndex),
    CreateRole {Show 18 fields
        names: Vec<ObjectName>,
        if_not_exists: bool,
        login: Option<bool>,
        inherit: Option<bool>,
        bypassrls: Option<bool>,
        password: Option<Password>,
        superuser: Option<bool>,
        create_db: Option<bool>,
        create_role: Option<bool>,
        replication: Option<bool>,
        connection_limit: Option<Expr>,
        valid_until: Option<Expr>,
        in_role: Vec<Ident>,
        in_group: Vec<Ident>,
        role: Vec<Ident>,
        user: Vec<Ident>,
        admin: Vec<Ident>,
        authorization_owner: Option<ObjectName>,
    },
    CreateSecret {
        or_replace: bool,
        temporary: Option<bool>,
        if_not_exists: bool,
        name: Option<Ident>,
        storage_specifier: Option<Ident>,
        secret_type: Ident,
        options: Vec<SecretOption>,
    },
    CreateServer(CreateServerStatement),
    CreatePolicy {
        name: Ident,
        table_name: ObjectName,
        policy_type: Option<CreatePolicyType>,
        command: Option<CreatePolicyCommand>,
        to: Option<Vec<Owner>>,
        using: Option<Expr>,
        with_check: Option<Expr>,
    },
    CreateConnector(CreateConnector),
    AlterTable {
        name: ObjectName,
        if_exists: bool,
        only: bool,
        operations: Vec<AlterTableOperation>,
        location: Option<HiveSetLocation>,
        on_cluster: Option<Ident>,
        iceberg: bool,
    },
    AlterIndex {
        name: ObjectName,
        operation: AlterIndexOperation,
    },
    AlterView {
        name: ObjectName,
        columns: Vec<Ident>,
        query: Box<Query>,
        with_options: Vec<SqlOption>,
    },
    AlterType(AlterType),
    AlterRole {
        name: Ident,
        operation: AlterRoleOperation,
    },
    AlterPolicy {
        name: Ident,
        table_name: ObjectName,
        operation: AlterPolicyOperation,
    },
    AlterConnector {
        name: Ident,
        properties: Option<Vec<SqlOption>>,
        url: Option<String>,
        owner: Option<AlterConnectorOwner>,
    },
    AlterSession {
        set: bool,
        session_params: KeyValueOptions,
    },
    AttachDatabase {
        schema_name: Ident,
        database_file_name: Expr,
        database: bool,
    },
    AttachDuckDBDatabase {
        if_not_exists: bool,
        database: bool,
        database_path: Ident,
        database_alias: Option<Ident>,
        attach_options: Vec<AttachDuckDBDatabaseOption>,
    },
    DetachDuckDBDatabase {
        if_exists: bool,
        database: bool,
        database_alias: Ident,
    },
    Drop {
        object_type: ObjectType,
        if_exists: bool,
        names: Vec<ObjectName>,
        cascade: bool,
        restrict: bool,
        purge: bool,
        temporary: bool,
        table: Option<ObjectName>,
    },
    DropFunction {
        if_exists: bool,
        func_desc: Vec<FunctionDesc>,
        drop_behavior: Option<DropBehavior>,
    },
    DropDomain(DropDomain),
    DropProcedure {
        if_exists: bool,
        proc_desc: Vec<FunctionDesc>,
        drop_behavior: Option<DropBehavior>,
    },
    DropSecret {
        if_exists: bool,
        temporary: Option<bool>,
        name: Ident,
        storage_specifier: Option<Ident>,
    },
    DropPolicy {
        if_exists: bool,
        name: Ident,
        table_name: ObjectName,
        drop_behavior: Option<DropBehavior>,
    },
    DropConnector {
        if_exists: bool,
        name: Ident,
    },
    Declare {
        stmts: Vec<Declare>,
    },
    CreateExtension {
        name: Ident,
        if_not_exists: bool,
        cascade: bool,
        schema: Option<Ident>,
        version: Option<Ident>,
    },
    DropExtension {
        names: Vec<Ident>,
        if_exists: bool,
        cascade_or_restrict: Option<ReferentialAction>,
    },
    Fetch {
        name: Ident,
        direction: FetchDirection,
        position: FetchPosition,
        into: Option<ObjectName>,
    },
    Flush {
        object_type: FlushType,
        location: Option<FlushLocation>,
        channel: Option<String>,
        read_lock: bool,
        export: bool,
        tables: Vec<ObjectName>,
    },
    Discard {
        object_type: DiscardObject,
    },
    ShowFunctions {
        filter: Option<ShowStatementFilter>,
    },
    ShowVariable {
        variable: Vec<Ident>,
    },
    ShowStatus {
        filter: Option<ShowStatementFilter>,
        global: bool,
        session: bool,
    },
    ShowVariables {
        filter: Option<ShowStatementFilter>,
        global: bool,
        session: bool,
    },
    ShowCreate {
        obj_type: ShowCreateObject,
        obj_name: ObjectName,
    },
    ShowColumns {
        extended: bool,
        full: bool,
        show_options: ShowStatementOptions,
    },
    ShowDatabases {
        terse: bool,
        history: bool,
        show_options: ShowStatementOptions,
    },
    ShowSchemas {
        terse: bool,
        history: bool,
        show_options: ShowStatementOptions,
    },
    ShowObjects(ShowObjects),
    ShowTables {
        terse: bool,
        history: bool,
        extended: bool,
        full: bool,
        external: bool,
        show_options: ShowStatementOptions,
    },
    ShowViews {
        terse: bool,
        materialized: bool,
        show_options: ShowStatementOptions,
    },
    ShowCollation {
        filter: Option<ShowStatementFilter>,
    },
    Use(Use),
    StartTransaction {
        modes: Vec<TransactionMode>,
        begin: bool,
        transaction: Option<BeginTransactionKind>,
        modifier: Option<TransactionModifier>,
        statements: Vec<Statement>,
        exception: Option<Vec<ExceptionWhen>>,
        has_end_keyword: bool,
    },
    Comment {
        object_type: CommentObject,
        object_name: ObjectName,
        comment: Option<String>,
        if_exists: bool,
    },
    Commit {
        chain: bool,
        end: bool,
        modifier: Option<TransactionModifier>,
    },
    Rollback {
        chain: bool,
        savepoint: Option<Ident>,
    },
    CreateSchema {
        schema_name: SchemaName,
        if_not_exists: bool,
        with: Option<Vec<SqlOption>>,
        options: Option<Vec<SqlOption>>,
        default_collate_spec: Option<Expr>,
    },
    CreateDatabase {
        db_name: ObjectName,
        if_not_exists: bool,
        location: Option<String>,
        managed_location: Option<String>,
    },
    CreateFunction(CreateFunction),
    CreateTrigger {Show 15 fields
        or_alter: bool,
        or_replace: bool,
        is_constraint: bool,
        name: ObjectName,
        period: TriggerPeriod,
        events: Vec<TriggerEvent>,
        table_name: ObjectName,
        referenced_table_name: Option<ObjectName>,
        referencing: Vec<TriggerReferencing>,
        trigger_object: TriggerObject,
        include_each: bool,
        condition: Option<Expr>,
        exec_body: Option<TriggerExecBody>,
        statements: Option<ConditionalStatements>,
        characteristics: Option<ConstraintCharacteristics>,
    },
    DropTrigger {
        if_exists: bool,
        trigger_name: ObjectName,
        table_name: Option<ObjectName>,
        option: Option<ReferentialAction>,
    },
    CreateProcedure {
        or_alter: bool,
        name: ObjectName,
        params: Option<Vec<ProcedureParam>>,
        language: Option<Ident>,
        body: ConditionalStatements,
    },
    CreateMacro {
        or_replace: bool,
        temporary: bool,
        name: ObjectName,
        args: Option<Vec<MacroArg>>,
        definition: MacroDefinition,
    },
    CreateStage {
        or_replace: bool,
        temporary: bool,
        if_not_exists: bool,
        name: ObjectName,
        stage_params: StageParamsObject,
        directory_table_params: KeyValueOptions,
        file_format: KeyValueOptions,
        copy_options: KeyValueOptions,
        comment: Option<String>,
    },
    Assert {
        condition: Expr,
        message: Option<Expr>,
    },
    Grant {
        privileges: Privileges,
        objects: Option<GrantObjects>,
        grantees: Vec<Grantee>,
        with_grant_option: bool,
        as_grantor: Option<Ident>,
        granted_by: Option<Ident>,
        current_grants: Option<CurrentGrantsKind>,
    },
    Deny(DenyStatement),
    Revoke {
        privileges: Privileges,
        objects: Option<GrantObjects>,
        grantees: Vec<Grantee>,
        granted_by: Option<Ident>,
        cascade: Option<CascadeOption>,
    },
    Deallocate {
        name: Ident,
        prepare: bool,
    },
    Execute {
        name: Option<ObjectName>,
        parameters: Vec<Expr>,
        has_parentheses: bool,
        immediate: bool,
        into: Vec<Ident>,
        using: Vec<ExprWithAlias>,
        output: bool,
        default: bool,
    },
    Prepare {
        name: Ident,
        data_types: Vec<DataType>,
        statement: Box<Statement>,
    },
    Kill {
        modifier: Option<KillType>,
        id: u64,
    },
    ExplainTable {
        describe_alias: DescribeAlias,
        hive_format: Option<HiveDescribeFormat>,
        has_table_keyword: bool,
        table_name: ObjectName,
    },
    Explain {
        describe_alias: DescribeAlias,
        analyze: bool,
        verbose: bool,
        query_plan: bool,
        estimate: bool,
        statement: Box<Statement>,
        format: Option<AnalyzeFormat>,
        options: Option<Vec<UtilityOption>>,
    },
    Savepoint {
        name: Ident,
    },
    ReleaseSavepoint {
        name: Ident,
    },
    Merge {
        into: bool,
        table: TableFactor,
        source: TableFactor,
        on: Box<Expr>,
        clauses: Vec<MergeClause>,
        output: Option<OutputClause>,
    },
    Cache {
        table_flag: Option<ObjectName>,
        table_name: ObjectName,
        has_as: bool,
        options: Vec<SqlOption>,
        query: Option<Box<Query>>,
    },
    UNCache {
        table_name: ObjectName,
        if_exists: bool,
    },
    CreateSequence {
        temporary: bool,
        if_not_exists: bool,
        name: ObjectName,
        data_type: Option<DataType>,
        sequence_options: Vec<SequenceOptions>,
        owned_by: Option<ObjectName>,
    },
    CreateDomain(CreateDomain),
    CreateType {
        name: ObjectName,
        representation: UserDefinedTypeRepresentation,
    },
    Pragma {
        name: ObjectName,
        value: Option<Value>,
        is_eq: bool,
    },
    LockTables {
        tables: Vec<LockTable>,
    },
    UnlockTables,
    Unload {
        query: Box<Query>,
        to: Ident,
        with: Vec<SqlOption>,
    },
    OptimizeTable {
        name: ObjectName,
        on_cluster: Option<Ident>,
        partition: Option<Partition>,
        include_final: bool,
        deduplicate: Option<Deduplicate>,
    },
    LISTEN {
        channel: Ident,
    },
    UNLISTEN {
        channel: Ident,
    },
    NOTIFY {
        channel: Ident,
        payload: Option<String>,
    },
    LoadData {
        local: bool,
        inpath: String,
        overwrite: bool,
        table_name: ObjectName,
        partitioned: Option<Vec<Expr>>,
        table_format: Option<HiveLoadDataFormat>,
    },
    RenameTable(Vec<RenameTable>),
    List(FileStagingCommand),
    Remove(FileStagingCommand),
    RaisError {
        message: Box<Expr>,
        severity: Box<Expr>,
        state: Box<Expr>,
        arguments: Vec<Expr>,
        options: Vec<RaisErrorOption>,
    },
    Print(PrintStatement),
    Return(ReturnStatement),
}
```

Expand description

A top-level statement (SELECT, INSERT, CREATE, etc.)

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Analyze" class="anchor">§</a>

### Analyze

``` sql
ANALYZE
```

Analyze (Hive)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Analyze.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Analyze.field.partitions" class="anchor field">§</a>`partitions: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Analyze.field.for_columns" class="anchor field">§</a>`for_columns: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Analyze.field.columns" class="anchor field">§</a>`columns: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Analyze.field.cache_metadata" class="anchor field">§</a>`cache_metadata: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Analyze.field.noscan" class="anchor field">§</a>`noscan: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Analyze.field.compute_statistics" class="anchor field">§</a>`compute_statistics: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Analyze.field.has_table_keyword" class="anchor field">§</a>`has_table_keyword: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Set" class="anchor">§</a>

### Set(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Truncate" class="anchor">§</a>

### Truncate

``` sql
TRUNCATE
```

Truncate (Hive)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Truncate.field.table_names" class="anchor field">§</a>`table_names: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.TruncateTableTarget.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::TruncateTableTarget"><code>TruncateTableTarget</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Truncate.field.partitions" class="anchor field">§</a>`partitions: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Truncate.field.table" class="anchor field">§</a>`table: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

TABLE - optional keyword;

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Truncate.field.identity" class="anchor field">§</a>`identity: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TruncateIdentityOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TruncateIdentityOption"><code>TruncateIdentityOption</code></a>`>`

Postgres-specific option \[ RESTART IDENTITY \| CONTINUE IDENTITY \]

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Truncate.field.cascade" class="anchor field">§</a>`cascade: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CascadeOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CascadeOption"><code>CascadeOption</code></a>`>`

Postgres-specific option \[ CASCADE \| RESTRICT \]

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Truncate.field.on_cluster" class="anchor field">§</a>`on_cluster: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

ClickHouse-specific option \[ ON CLUSTER cluster_name \]

[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/truncate/)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Msck" class="anchor">§</a>

### Msck

``` sql
MSCK
```

Msck (Hive)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Msck.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Msck.field.repair" class="anchor field">§</a>`repair: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Msck.field.partition_action" class="anchor field">§</a>`partition_action: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.AddDropSync.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::AddDropSync"><code>AddDropSync</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Query" class="anchor">§</a>

### Query(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Query.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Query">Query</a>\>)

``` sql
SELECT
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Insert" class="anchor">§</a>

### Insert(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Insert.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Insert">Insert</a>)

``` sql
INSERT
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Install" class="anchor">§</a>

### Install

``` sql
INSTALL
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Install.field.extension_name" class="anchor field">§</a>`extension_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

Only for DuckDB

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Load" class="anchor">§</a>

### Load

``` sql
LOAD
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Load.field.extension_name" class="anchor field">§</a>`extension_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

Only for DuckDB

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Directory" class="anchor">§</a>

### Directory

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Directory.field.overwrite" class="anchor field">§</a>`overwrite: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Directory.field.local" class="anchor field">§</a>`local: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Directory.field.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Directory.field.file_format" class="anchor field">§</a>`file_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.FileFormat.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::FileFormat"><code>FileFormat</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Directory.field.source" class="anchor field">§</a>`source: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Query.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Query"><code>Query</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Case" class="anchor">§</a>

### Case(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.CaseStatement.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::CaseStatement">CaseStatement</a>)

A `CASE` statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.If" class="anchor">§</a>

### If(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.IfStatement.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::IfStatement">IfStatement</a>)

An `IF` statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.While" class="anchor">§</a>

### While(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.WhileStatement.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::WhileStatement">WhileStatement</a>)

A `WHILE` statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Raise" class="anchor">§</a>

### Raise(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.RaiseStatement.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::RaiseStatement">RaiseStatement</a>)

A `RAISE` statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Call" class="anchor">§</a>

### Call(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Function.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Function">Function</a>)

``` sql
CALL <function>
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Copy" class="anchor">§</a>

### Copy

``` sql
COPY [TO | FROM] ...
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Copy.field.source" class="anchor field">§</a>`source: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CopySource.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CopySource"><code>CopySource</code></a>

The source of ‘COPY TO’, or the target of ‘COPY FROM’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Copy.field.to" class="anchor field">§</a>`to: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

If true, is a ‘COPY TO’ statement. If false is a ‘COPY FROM’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Copy.field.target" class="anchor field">§</a>`target: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CopyTarget.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CopyTarget"><code>CopyTarget</code></a>

The target of ‘COPY TO’, or the source of ‘COPY FROM’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Copy.field.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CopyOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CopyOption"><code>CopyOption</code></a>`>`

WITH options (from PostgreSQL version 9.0)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Copy.field.legacy_options" class="anchor field">§</a>`legacy_options: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CopyLegacyOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CopyLegacyOption"><code>CopyLegacyOption</code></a>`>`

WITH options (before PostgreSQL version 9.0)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Copy.field.values" class="anchor field">§</a>`values: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>>`

VALUES a vector of values to be copied

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake" class="anchor">§</a>

### CopyIntoSnowflake

``` sql
COPY INTO <table> | <location>
```

See: <https://docs.snowflake.com/en/sql-reference/sql/copy-into-table> <https://docs.snowflake.com/en/sql-reference/sql/copy-into-location>

Copy Into syntax available for Snowflake is different than the one implemented in Postgres. Although they share common prefix, it is reasonable to implement them in different enums. This can be refactored later once custom dialects are allowed to have custom Statements.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.kind" class="anchor field">§</a>`kind: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CopyIntoSnowflakeKind.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CopyIntoSnowflakeKind"><code>CopyIntoSnowflakeKind</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.into" class="anchor field">§</a>`into: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.into_columns" class="anchor field">§</a>`into_columns: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.from_obj" class="anchor field">§</a>`from_obj: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.from_obj_alias" class="anchor field">§</a>`from_obj_alias: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.stage_params" class="anchor field">§</a>`stage_params: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/stmt_data_loading/struct.StageParamsObject.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::stmt_data_loading::StageParamsObject"><code>StageParamsObject</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.from_transformations" class="anchor field">§</a>`from_transformations: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/stmt_data_loading/enum.StageLoadSelectItemKind.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::helpers::stmt_data_loading::StageLoadSelectItemKind"><code>StageLoadSelectItemKind</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.from_query" class="anchor field">§</a>`from_query: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Query.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Query"><code>Query</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.files" class="anchor field">§</a>`files: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.pattern" class="anchor field">§</a>`pattern: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.file_format" class="anchor field">§</a>`file_format: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/key_value_options/struct.KeyValueOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::key_value_options::KeyValueOptions"><code>KeyValueOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.copy_options" class="anchor field">§</a>`copy_options: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/key_value_options/struct.KeyValueOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::key_value_options::KeyValueOptions"><code>KeyValueOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.validation_mode" class="anchor field">§</a>`validation_mode: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake.field.partition" class="anchor field">§</a>`partition: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Open" class="anchor">§</a>

### Open(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.OpenStatement.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::OpenStatement">OpenStatement</a>)

``` sql
OPEN cursor_name
```

Opens a cursor.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Close" class="anchor">§</a>

### Close

``` sql
CLOSE
```

Closes the portal underlying an open cursor.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Close.field.cursor" class="anchor field">§</a>`cursor: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CloseCursor.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CloseCursor"><code>CloseCursor</code></a>

Cursor name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Update" class="anchor">§</a>

### Update

``` sql
UPDATE
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Update.field.table" class="anchor field">§</a>`table: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.TableWithJoins.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::TableWithJoins"><code>TableWithJoins</code></a>

TABLE

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Update.field.assignments" class="anchor field">§</a>`assignments: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Assignment.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Assignment"><code>Assignment</code></a>`>`

Column assignments

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Update.field.from" class="anchor field">§</a>`from: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.UpdateTableFromKind.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::UpdateTableFromKind"><code>UpdateTableFromKind</code></a>`>`

Table which provide value to be set

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Update.field.selection" class="anchor field">§</a>`selection: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

WHERE

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Update.field.returning" class="anchor field">§</a>`returning: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SelectItem.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SelectItem"><code>SelectItem</code></a>`>>`

RETURNING

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Update.field.or" class="anchor field">§</a>`or: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SqliteOnConflict.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SqliteOnConflict"><code>SqliteOnConflict</code></a>`>`

SQLite-specific conflict resolution clause

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Delete" class="anchor">§</a>

### Delete(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Delete.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Delete">Delete</a>)

``` sql
DELETE
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView" class="anchor">§</a>

### CreateView

``` sql
CREATE VIEW
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.or_alter" class="anchor field">§</a>`or_alter: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

True if this is a `CREATE OR ALTER VIEW` statement

[MsSql](https://learn.microsoft.com/en-us/sql/t-sql/statements/create-view-transact-sql)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.or_replace" class="anchor field">§</a>`or_replace: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.materialized" class="anchor field">§</a>`materialized: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

View name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.columns" class="anchor field">§</a>`columns: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ViewColumnDef.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ViewColumnDef"><code>ViewColumnDef</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.query" class="anchor field">§</a>`query: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Query.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Query"><code>Query</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.options" class="anchor field">§</a>`options: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CreateTableOptions.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CreateTableOptions"><code>CreateTableOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.cluster_by" class="anchor field">§</a>`cluster_by: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.comment" class="anchor field">§</a>`comment: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Snowflake: Views can have comments in Snowflake. <https://docs.snowflake.com/en/sql-reference/sql/create-view#syntax>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.with_no_schema_binding" class="anchor field">§</a>`with_no_schema_binding: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

if true, has RedShift \[`WITH NO SCHEMA BINDING`\] clause <https://docs.aws.amazon.com/redshift/latest/dg/r_CREATE_VIEW.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

if true, has SQLite `IF NOT EXISTS` clause <https://www.sqlite.org/lang_createview.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.temporary" class="anchor field">§</a>`temporary: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

if true, has SQLite `TEMP` or `TEMPORARY` clause <https://www.sqlite.org/lang_createview.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.to" class="anchor field">§</a>`to: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

if not None, has Clickhouse `TO` clause, specify the table into which to insert results <https://clickhouse.com/docs/en/sql-reference/statements/create/view#materialized-view>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateView.field.params" class="anchor field">§</a>`params: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.CreateViewParams.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::CreateViewParams"><code>CreateViewParams</code></a>`>`

MySQL: Optional parameters for the view algorithm, definer, and security context

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTable" class="anchor">§</a>

### CreateTable(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.CreateTable.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::CreateTable">CreateTable</a>)

``` sql
CREATE TABLE
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateVirtualTable" class="anchor">§</a>

### CreateVirtualTable

``` sql
CREATE VIRTUAL TABLE .. USING <module_name> (<module_args>)`
```

Sqlite specific statement

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateVirtualTable.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateVirtualTable.field.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateVirtualTable.field.module_name" class="anchor field">§</a>`module_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateVirtualTable.field.module_args" class="anchor field">§</a>`module_args: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateIndex" class="anchor">§</a>

### CreateIndex(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.CreateIndex.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::CreateIndex">CreateIndex</a>)

``` sql
`CREATE INDEX`
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole" class="anchor">§</a>

### CreateRole

``` sql
CREATE ROLE
```

See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createrole.html)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.names" class="anchor field">§</a>`names: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.login" class="anchor field">§</a>`login: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.inherit" class="anchor field">§</a>`inherit: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.bypassrls" class="anchor field">§</a>`bypassrls: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.password" class="anchor field">§</a>`password: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Password.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Password"><code>Password</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.superuser" class="anchor field">§</a>`superuser: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.create_db" class="anchor field">§</a>`create_db: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.create_role" class="anchor field">§</a>`create_role: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.replication" class="anchor field">§</a>`replication: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.connection_limit" class="anchor field">§</a>`connection_limit: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.valid_until" class="anchor field">§</a>`valid_until: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.in_role" class="anchor field">§</a>`in_role: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.in_group" class="anchor field">§</a>`in_group: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.role" class="anchor field">§</a>`role: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.user" class="anchor field">§</a>`user: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.admin" class="anchor field">§</a>`admin: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole.field.authorization_owner" class="anchor field">§</a>`authorization_owner: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSecret" class="anchor">§</a>

### CreateSecret

``` sql
CREATE SECRET
```

See [DuckDB](https://duckdb.org/docs/sql/statements/create_secret.html)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSecret.field.or_replace" class="anchor field">§</a>`or_replace: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSecret.field.temporary" class="anchor field">§</a>`temporary: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSecret.field.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSecret.field.name" class="anchor field">§</a>`name: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSecret.field.storage_specifier" class="anchor field">§</a>`storage_specifier: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSecret.field.secret_type" class="anchor field">§</a>`secret_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSecret.field.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.SecretOption.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::SecretOption"><code>SecretOption</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateServer" class="anchor">§</a>

### CreateServer(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.CreateServerStatement.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::CreateServerStatement">CreateServerStatement</a>)

A `CREATE SERVER` statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreatePolicy" class="anchor">§</a>

### CreatePolicy

``` sql
CREATE POLICY
```

See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createpolicy.html)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreatePolicy.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreatePolicy.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreatePolicy.field.policy_type" class="anchor field">§</a>`policy_type: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CreatePolicyType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CreatePolicyType"><code>CreatePolicyType</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreatePolicy.field.command" class="anchor field">§</a>`command: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CreatePolicyCommand.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CreatePolicyCommand"><code>CreatePolicyCommand</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreatePolicy.field.to" class="anchor field">§</a>`to: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Owner.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Owner"><code>Owner</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreatePolicy.field.using" class="anchor field">§</a>`using: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreatePolicy.field.with_check" class="anchor field">§</a>`with_check: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateConnector" class="anchor">§</a>

### CreateConnector(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.CreateConnector.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::CreateConnector">CreateConnector</a>)

``` sql
CREATE CONNECTOR
```

See [Hive](https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=27362034#LanguageManualDDL-CreateDataConnectorCreateConnector)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterTable" class="anchor">§</a>

### AlterTable

``` sql
ALTER TABLE
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterTable.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

Table name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterTable.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterTable.field.only" class="anchor field">§</a>`only: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterTable.field.operations" class="anchor field">§</a>`operations: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.AlterTableOperation.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::AlterTableOperation"><code>AlterTableOperation</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterTable.field.location" class="anchor field">§</a>`location: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.HiveSetLocation.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::HiveSetLocation"><code>HiveSetLocation</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterTable.field.on_cluster" class="anchor field">§</a>`on_cluster: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

ClickHouse dialect supports `ON CLUSTER` clause for ALTER TABLE For example: `ALTER TABLE table_name ON CLUSTER cluster_name ADD COLUMN c UInt32` [ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/alter/update)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterTable.field.iceberg" class="anchor field">§</a>`iceberg: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Snowflake “ICEBERG” clause for Iceberg tables <https://docs.snowflake.com/en/sql-reference/sql/alter-iceberg-table>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterIndex" class="anchor">§</a>

### AlterIndex

``` sql
ALTER INDEX
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterIndex.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterIndex.field.operation" class="anchor field">§</a>`operation: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.AlterIndexOperation.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::AlterIndexOperation"><code>AlterIndexOperation</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterView" class="anchor">§</a>

### AlterView

``` sql
ALTER VIEW
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterView.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

View name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterView.field.columns" class="anchor field">§</a>`columns: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterView.field.query" class="anchor field">§</a>`query: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Query.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Query"><code>Query</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterView.field.with_options" class="anchor field">§</a>`with_options: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SqlOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SqlOption"><code>SqlOption</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterType" class="anchor">§</a>

### AlterType(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.AlterType.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::AlterType">AlterType</a>)

``` sql
ALTER TYPE
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-altertype.html)
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterRole" class="anchor">§</a>

### AlterRole

``` sql
ALTER ROLE
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterRole.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterRole.field.operation" class="anchor field">§</a>`operation: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.AlterRoleOperation.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::AlterRoleOperation"><code>AlterRoleOperation</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterPolicy" class="anchor">§</a>

### AlterPolicy

``` sql
ALTER POLICY <NAME> ON <TABLE NAME> [<OPERATION>]
```

(Postgresql-specific)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterPolicy.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterPolicy.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterPolicy.field.operation" class="anchor field">§</a>`operation: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.AlterPolicyOperation.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::AlterPolicyOperation"><code>AlterPolicyOperation</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterConnector" class="anchor">§</a>

### AlterConnector

``` sql
ALTER CONNECTOR connector_name SET DCPROPERTIES(property_name=property_value, ...);
or
ALTER CONNECTOR connector_name SET URL new_url;
or
ALTER CONNECTOR connector_name SET OWNER [USER|ROLE] user_or_role;
```

(Hive-specific)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterConnector.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterConnector.field.properties" class="anchor field">§</a>`properties: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SqlOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SqlOption"><code>SqlOption</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterConnector.field.url" class="anchor field">§</a>`url: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterConnector.field.owner" class="anchor field">§</a>`owner: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.AlterConnectorOwner.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::AlterConnectorOwner"><code>AlterConnectorOwner</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterSession" class="anchor">§</a>

### AlterSession

``` sql
ALTER SESSION SET sessionParam
ALTER SESSION UNSET <param_name> [ , <param_name> , ... ]
```

See <https://docs.snowflake.com/en/sql-reference/sql/alter-session>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterSession.field.set" class="anchor field">§</a>`set: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

true is to set for the session parameters, false is to unset

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterSession.field.session_params" class="anchor field">§</a>`session_params: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/key_value_options/struct.KeyValueOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::key_value_options::KeyValueOptions"><code>KeyValueOptions</code></a>

The session parameters to set or unset

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDatabase" class="anchor">§</a>

### AttachDatabase

``` sql
ATTACH DATABASE 'path/to/file' AS alias
```

(SQLite-specific)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDatabase.field.schema_name" class="anchor field">§</a>`schema_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

The name to bind to the newly attached database

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDatabase.field.database_file_name" class="anchor field">§</a>`database_file_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>

An expression that indicates the path to the database file

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDatabase.field.database" class="anchor field">§</a>`database: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

true if the syntax is ‘ATTACH DATABASE’, false if it’s just ‘ATTACH’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDuckDBDatabase" class="anchor">§</a>

### AttachDuckDBDatabase

(DuckDB-specific)

``` sql
ATTACH 'sqlite_file.db' AS sqlite_db (READ_ONLY, TYPE SQLITE);
```

See <https://duckdb.org/docs/sql/statements/attach.html>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDuckDBDatabase.field.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDuckDBDatabase.field.database" class="anchor field">§</a>`database: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

true if the syntax is ‘ATTACH DATABASE’, false if it’s just ‘ATTACH’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDuckDBDatabase.field.database_path" class="anchor field">§</a>`database_path: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

An expression that indicates the path to the database file

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDuckDBDatabase.field.database_alias" class="anchor field">§</a>`database_alias: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDuckDBDatabase.field.attach_options" class="anchor field">§</a>`attach_options: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.AttachDuckDBDatabaseOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::AttachDuckDBDatabaseOption"><code>AttachDuckDBDatabaseOption</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DetachDuckDBDatabase" class="anchor">§</a>

### DetachDuckDBDatabase

(DuckDB-specific)

``` sql
DETACH db_alias;
```

See <https://duckdb.org/docs/sql/statements/attach.html>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DetachDuckDBDatabase.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DetachDuckDBDatabase.field.database" class="anchor field">§</a>`database: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

true if the syntax is ‘DETACH DATABASE’, false if it’s just ‘DETACH’

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DetachDuckDBDatabase.field.database_alias" class="anchor field">§</a>`database_alias: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Drop" class="anchor">§</a>

### Drop

``` sql
DROP [TABLE, VIEW, ...]
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Drop.field.object_type" class="anchor field">§</a>`object_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ObjectType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ObjectType"><code>ObjectType</code></a>

The type of the object to drop: TABLE, VIEW, etc.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Drop.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

An optional `IF EXISTS` clause. (Non-standard.)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Drop.field.names" class="anchor field">§</a>`names: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

One or more objects to drop. (ANSI SQL requires exactly one.)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Drop.field.cascade" class="anchor field">§</a>`cascade: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether `CASCADE` was specified. This will be `false` when `RESTRICT` or no drop behavior at all was specified.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Drop.field.restrict" class="anchor field">§</a>`restrict: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether `RESTRICT` was specified. This will be `false` when `CASCADE` or no drop behavior at all was specified.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Drop.field.purge" class="anchor field">§</a>`purge: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Hive allows you specify whether the table’s stored data will be deleted along with the dropped table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Drop.field.temporary" class="anchor field">§</a>`temporary: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

MySQL-specific “TEMPORARY” keyword

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Drop.field.table" class="anchor field">§</a>`table: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

MySQL-specific drop index syntax, which requires table specification See <https://dev.mysql.com/doc/refman/8.4/en/drop-index.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropFunction" class="anchor">§</a>

### DropFunction

``` sql
DROP FUNCTION
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropFunction.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropFunction.field.func_desc" class="anchor field">§</a>`func_desc: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.FunctionDesc.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::FunctionDesc"><code>FunctionDesc</code></a>`>`

One or more function to drop

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropFunction.field.drop_behavior" class="anchor field">§</a>`drop_behavior: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DropBehavior.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DropBehavior"><code>DropBehavior</code></a>`>`

`CASCADE` or `RESTRICT`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropDomain" class="anchor">§</a>

### DropDomain(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.DropDomain.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::DropDomain">DropDomain</a>)

``` sql
DROP DOMAIN
```

See [PostgreSQL](https://www.postgresql.org/docs/current/sql-dropdomain.html)

DROP DOMAIN \[ IF EXISTS \] name \[, …\] \[ CASCADE \| RESTRICT \]

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropProcedure" class="anchor">§</a>

### DropProcedure

``` sql
DROP PROCEDURE
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropProcedure.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropProcedure.field.proc_desc" class="anchor field">§</a>`proc_desc: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.FunctionDesc.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::FunctionDesc"><code>FunctionDesc</code></a>`>`

One or more function to drop

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropProcedure.field.drop_behavior" class="anchor field">§</a>`drop_behavior: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DropBehavior.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DropBehavior"><code>DropBehavior</code></a>`>`

`CASCADE` or `RESTRICT`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropSecret" class="anchor">§</a>

### DropSecret

``` sql
DROP SECRET
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropSecret.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropSecret.field.temporary" class="anchor field">§</a>`temporary: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropSecret.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropSecret.field.storage_specifier" class="anchor field">§</a>`storage_specifier: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropPolicy" class="anchor">§</a>

### DropPolicy

``` sql
 DROP POLICY
```

See [PostgreSQL](https://www.postgresql.org/docs/current/sql-droppolicy.html)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropPolicy.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropPolicy.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropPolicy.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropPolicy.field.drop_behavior" class="anchor field">§</a>`drop_behavior: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DropBehavior.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DropBehavior"><code>DropBehavior</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropConnector" class="anchor">§</a>

### DropConnector

``` sql
DROP CONNECTOR
```

See [Hive](https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=27362034#LanguageManualDDL-DropConnector)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropConnector.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropConnector.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Declare" class="anchor">§</a>

### Declare

``` sql
DECLARE
```

Declare Cursor Variables

Note: this is a PostgreSQL-specific statement, but may also compatible with other SQL.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Declare.field.stmts" class="anchor field">§</a>`stmts: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Declare.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Declare"><code>Declare</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateExtension" class="anchor">§</a>

### CreateExtension

``` sql
CREATE EXTENSION [ IF NOT EXISTS ] extension_name
    [ WITH ] [ SCHEMA schema_name ]
             [ VERSION version ]
             [ CASCADE ]
```

Note: this is a PostgreSQL-specific statement,

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateExtension.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateExtension.field.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateExtension.field.cascade" class="anchor field">§</a>`cascade: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateExtension.field.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateExtension.field.version" class="anchor field">§</a>`version: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropExtension" class="anchor">§</a>

### DropExtension

``` sql
DROP EXTENSION [ IF EXISTS ] name [, ...] [ CASCADE | RESTRICT ]

Note: this is a PostgreSQL-specific statement.
https://www.postgresql.org/docs/current/sql-dropextension.html
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropExtension.field.names" class="anchor field">§</a>`names: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropExtension.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropExtension.field.cascade_or_restrict" class="anchor field">§</a>`cascade_or_restrict: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ReferentialAction.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ReferentialAction"><code>ReferentialAction</code></a>`>`

`CASCADE` or `RESTRICT`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Fetch" class="anchor">§</a>

### Fetch

``` sql
FETCH
```

Retrieve rows from a query using a cursor

Note: this is a PostgreSQL-specific statement, but may also compatible with other SQL.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Fetch.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

Cursor name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Fetch.field.direction" class="anchor field">§</a>`direction: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.FetchDirection.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::FetchDirection"><code>FetchDirection</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Fetch.field.position" class="anchor field">§</a>`position: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.FetchPosition.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::FetchPosition"><code>FetchPosition</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Fetch.field.into" class="anchor field">§</a>`into: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

Optional, It’s possible to fetch rows form cursor to the table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Flush" class="anchor">§</a>

### Flush

``` sql
FLUSH [NO_WRITE_TO_BINLOG | LOCAL] flush_option [, flush_option] ... | tables_option
```

Note: this is a Mysql-specific statement, but may also compatible with other SQL.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Flush.field.object_type" class="anchor field">§</a>`object_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.FlushType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::FlushType"><code>FlushType</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Flush.field.location" class="anchor field">§</a>`location: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.FlushLocation.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::FlushLocation"><code>FlushLocation</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Flush.field.channel" class="anchor field">§</a>`channel: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Flush.field.read_lock" class="anchor field">§</a>`read_lock: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Flush.field.export" class="anchor field">§</a>`export: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Flush.field.tables" class="anchor field">§</a>`tables: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Discard" class="anchor">§</a>

### Discard

``` sql
DISCARD [ ALL | PLANS | SEQUENCES | TEMPORARY | TEMP ]
```

Note: this is a PostgreSQL-specific statement, but may also compatible with other SQL.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Discard.field.object_type" class="anchor field">§</a>`object_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DiscardObject.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DiscardObject"><code>DiscardObject</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowFunctions" class="anchor">§</a>

### ShowFunctions

`SHOW FUNCTIONS`

Note: this is a Presto-specific statement.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowFunctions.field.filter" class="anchor field">§</a>`filter: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ShowStatementFilter.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ShowStatementFilter"><code>ShowStatementFilter</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowVariable" class="anchor">§</a>

### ShowVariable

``` sql
SHOW <variable>
```

Note: this is a PostgreSQL-specific statement.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowVariable.field.variable" class="anchor field">§</a>`variable: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowStatus" class="anchor">§</a>

### ShowStatus

``` sql
SHOW [GLOBAL | SESSION] STATUS [LIKE 'pattern' | WHERE expr]
```

Note: this is a MySQL-specific statement.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowStatus.field.filter" class="anchor field">§</a>`filter: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ShowStatementFilter.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ShowStatementFilter"><code>ShowStatementFilter</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowStatus.field.global" class="anchor field">§</a>`global: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowStatus.field.session" class="anchor field">§</a>`session: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowVariables" class="anchor">§</a>

### ShowVariables

``` sql
SHOW VARIABLES
```

Note: this is a MySQL-specific statement.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowVariables.field.filter" class="anchor field">§</a>`filter: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ShowStatementFilter.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ShowStatementFilter"><code>ShowStatementFilter</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowVariables.field.global" class="anchor field">§</a>`global: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowVariables.field.session" class="anchor field">§</a>`session: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowCreate" class="anchor">§</a>

### ShowCreate

``` sql
SHOW CREATE TABLE
```

Note: this is a MySQL-specific statement.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowCreate.field.obj_type" class="anchor field">§</a>`obj_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ShowCreateObject.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ShowCreateObject"><code>ShowCreateObject</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowCreate.field.obj_name" class="anchor field">§</a>`obj_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowColumns" class="anchor">§</a>

### ShowColumns

``` sql
SHOW COLUMNS
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowColumns.field.extended" class="anchor field">§</a>`extended: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowColumns.field.full" class="anchor field">§</a>`full: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowColumns.field.show_options" class="anchor field">§</a>`show_options: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ShowStatementOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ShowStatementOptions"><code>ShowStatementOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowDatabases" class="anchor">§</a>

### ShowDatabases

``` sql
SHOW DATABASES
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowDatabases.field.terse" class="anchor field">§</a>`terse: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowDatabases.field.history" class="anchor field">§</a>`history: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowDatabases.field.show_options" class="anchor field">§</a>`show_options: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ShowStatementOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ShowStatementOptions"><code>ShowStatementOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowSchemas" class="anchor">§</a>

### ShowSchemas

``` sql
SHOW SCHEMAS
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowSchemas.field.terse" class="anchor field">§</a>`terse: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowSchemas.field.history" class="anchor field">§</a>`history: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowSchemas.field.show_options" class="anchor field">§</a>`show_options: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ShowStatementOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ShowStatementOptions"><code>ShowStatementOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowObjects" class="anchor">§</a>

### ShowObjects(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ShowObjects.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ShowObjects">ShowObjects</a>)

``` sql
SHOW OBJECTS LIKE 'line%' IN mydb.public
```

Snowflake-specific statement <https://docs.snowflake.com/en/sql-reference/sql/show-objects>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowTables" class="anchor">§</a>

### ShowTables

``` sql
SHOW TABLES
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowTables.field.terse" class="anchor field">§</a>`terse: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowTables.field.history" class="anchor field">§</a>`history: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowTables.field.extended" class="anchor field">§</a>`extended: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowTables.field.full" class="anchor field">§</a>`full: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowTables.field.external" class="anchor field">§</a>`external: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowTables.field.show_options" class="anchor field">§</a>`show_options: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ShowStatementOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ShowStatementOptions"><code>ShowStatementOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowViews" class="anchor">§</a>

### ShowViews

``` sql
SHOW VIEWS
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowViews.field.terse" class="anchor field">§</a>`terse: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowViews.field.materialized" class="anchor field">§</a>`materialized: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowViews.field.show_options" class="anchor field">§</a>`show_options: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ShowStatementOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ShowStatementOptions"><code>ShowStatementOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowCollation" class="anchor">§</a>

### ShowCollation

``` sql
SHOW COLLATION
```

Note: this is a MySQL-specific statement.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowCollation.field.filter" class="anchor field">§</a>`filter: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ShowStatementFilter.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ShowStatementFilter"><code>ShowStatementFilter</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Use" class="anchor">§</a>

### Use(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Use.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Use">Use</a>)

``` sql
`USE ...`
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.StartTransaction" class="anchor">§</a>

### StartTransaction

``` sql
START  [ TRANSACTION | WORK ] | START TRANSACTION } ...
```

If `begin` is false.

``` sql
`BEGIN  [ TRANSACTION | WORK ] | START TRANSACTION } ...`
```

If `begin` is true

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.StartTransaction.field.modes" class="anchor field">§</a>`modes: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TransactionMode.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TransactionMode"><code>TransactionMode</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.StartTransaction.field.begin" class="anchor field">§</a>`begin: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.StartTransaction.field.transaction" class="anchor field">§</a>`transaction: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.BeginTransactionKind.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::BeginTransactionKind"><code>BeginTransactionKind</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.StartTransaction.field.modifier" class="anchor field">§</a>`modifier: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TransactionModifier.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TransactionModifier"><code>TransactionModifier</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.StartTransaction.field.statements" class="anchor field">§</a>`statements: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement"><code>Statement</code></a>`>`

List of statements belonging to the `BEGIN` block. Example:

``` sql
BEGIN
    SELECT 1;
    SELECT 2;
END;
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.StartTransaction.field.exception" class="anchor field">§</a>`exception: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ExceptionWhen.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ExceptionWhen"><code>ExceptionWhen</code></a>`>>`

Exception handling with exception clauses. Example:

``` sql
EXCEPTION
    WHEN EXCEPTION_1 THEN
        SELECT 2;
    WHEN EXCEPTION_2 OR EXCEPTION_3 THEN
        SELECT 3;
    WHEN OTHER THEN
        SELECT 4;
```

<https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#beginexceptionend> <https://docs.snowflake.com/en/sql-reference/snowflake-scripting/exception>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.StartTransaction.field.has_end_keyword" class="anchor field">§</a>`has_end_keyword: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

TRUE if the statement has an `END` keyword.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Comment" class="anchor">§</a>

### Comment

``` sql
COMMENT ON ...
```

Note: this is a PostgreSQL-specific statement.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Comment.field.object_type" class="anchor field">§</a>`object_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CommentObject.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CommentObject"><code>CommentObject</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Comment.field.object_name" class="anchor field">§</a>`object_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Comment.field.comment" class="anchor field">§</a>`comment: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Comment.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

An optional `IF EXISTS` clause. (Non-standard.) See <https://docs.snowflake.com/en/sql-reference/sql/comment>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Commit" class="anchor">§</a>

### Commit

``` sql
COMMIT [ TRANSACTION | WORK ] [ AND [ NO ] CHAIN ]
```

If `end` is false

``` sql
END [ TRY | CATCH ]
```

If `end` is true

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Commit.field.chain" class="anchor field">§</a>`chain: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Commit.field.end" class="anchor field">§</a>`end: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Commit.field.modifier" class="anchor field">§</a>`modifier: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TransactionModifier.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TransactionModifier"><code>TransactionModifier</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Rollback" class="anchor">§</a>

### Rollback

``` sql
ROLLBACK [ TRANSACTION | WORK ] [ AND [ NO ] CHAIN ] [ TO [ SAVEPOINT ] savepoint_name ]
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Rollback.field.chain" class="anchor field">§</a>`chain: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Rollback.field.savepoint" class="anchor field">§</a>`savepoint: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSchema" class="anchor">§</a>

### CreateSchema

``` sql
CREATE SCHEMA
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSchema.field.schema_name" class="anchor field">§</a>`schema_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SchemaName.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SchemaName"><code>SchemaName</code></a>

`<schema name> | AUTHORIZATION <schema authorization identifier> | <schema name> AUTHORIZATION <schema authorization identifier>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSchema.field.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSchema.field.with" class="anchor field">§</a>`with: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SqlOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SqlOption"><code>SqlOption</code></a>`>>`

Schema properties.

``` sql
CREATE SCHEMA myschema WITH (key1='value1');
```

[Trino](https://trino.io/docs/current/sql/create-schema.html)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSchema.field.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SqlOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SqlOption"><code>SqlOption</code></a>`>>`

Schema options.

``` sql
CREATE SCHEMA myschema OPTIONS(key1='value1');
```

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_schema_statement)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSchema.field.default_collate_spec" class="anchor field">§</a>`default_collate_spec: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

Default collation specification for the schema.

``` sql
CREATE SCHEMA myschema DEFAULT COLLATE 'und:ci';
```

[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_schema_statement)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateDatabase" class="anchor">§</a>

### CreateDatabase

``` sql
CREATE DATABASE
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateDatabase.field.db_name" class="anchor field">§</a>`db_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateDatabase.field.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateDatabase.field.location" class="anchor field">§</a>`location: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateDatabase.field.managed_location" class="anchor field">§</a>`managed_location: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateFunction" class="anchor">§</a>

### CreateFunction(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.CreateFunction.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::CreateFunction">CreateFunction</a>)

``` sql
CREATE FUNCTION
```

Supported variants:

1.  [Hive](https://cwiki.apache.org/confluence/display/hive/languagemanual+ddl#LanguageManualDDL-Create/Drop/ReloadFunction)
2.  [PostgreSQL](https://www.postgresql.org/docs/15/sql-createfunction.html)
3.  [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_function_statement)
4.  [MsSql](https://learn.microsoft.com/en-us/sql/t-sql/statements/create-function-transact-sql)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger" class="anchor">§</a>

### CreateTrigger

CREATE TRIGGER

Examples:

``` sql
CREATE TRIGGER trigger_name
BEFORE INSERT ON table_name
FOR EACH ROW
EXECUTE FUNCTION trigger_function();
```

Postgres: <https://www.postgresql.org/docs/current/sql-createtrigger.html> SQL Server: <https://learn.microsoft.com/en-us/sql/t-sql/statements/create-trigger-transact-sql>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.or_alter" class="anchor field">§</a>`or_alter: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

True if this is a `CREATE OR ALTER TRIGGER` statement

[MsSql](https://learn.microsoft.com/en-us/sql/t-sql/statements/create-trigger-transact-sql?view=sql-server-ver16#arguments)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.or_replace" class="anchor field">§</a>`or_replace: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

The `OR REPLACE` clause is used to re-create the trigger if it already exists.

Example:

``` sql
CREATE OR REPLACE TRIGGER trigger_name
AFTER INSERT ON table_name
FOR EACH ROW
EXECUTE FUNCTION trigger_function();
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.is_constraint" class="anchor field">§</a>`is_constraint: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

The `CONSTRAINT` keyword is used to create a trigger as a constraint.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

The name of the trigger to be created.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.period" class="anchor field">§</a>`period: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TriggerPeriod.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TriggerPeriod"><code>TriggerPeriod</code></a>

Determines whether the function is called before, after, or instead of the event.

Example of BEFORE:

``` sql
CREATE TRIGGER trigger_name
BEFORE INSERT ON table_name
FOR EACH ROW
EXECUTE FUNCTION trigger_function();
```

Example of AFTER:

``` sql
CREATE TRIGGER trigger_name
AFTER INSERT ON table_name
FOR EACH ROW
EXECUTE FUNCTION trigger_function();
```

Example of INSTEAD OF:

``` sql
CREATE TRIGGER trigger_name
INSTEAD OF INSERT ON table_name
FOR EACH ROW
EXECUTE FUNCTION trigger_function();
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.events" class="anchor field">§</a>`events: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TriggerEvent.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TriggerEvent"><code>TriggerEvent</code></a>`>`

Multiple events can be specified using OR, such as `INSERT`, `UPDATE`, `DELETE`, or `TRUNCATE`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

The table on which the trigger is to be created.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.referenced_table_name" class="anchor field">§</a>`referenced_table_name: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

The optional referenced table name that can be referenced via the `FROM` keyword.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.referencing" class="anchor field">§</a>`referencing: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.TriggerReferencing.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::TriggerReferencing"><code>TriggerReferencing</code></a>`>`

This keyword immediately precedes the declaration of one or two relation names that provide access to the transition relations of the triggering statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.trigger_object" class="anchor field">§</a>`trigger_object: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TriggerObject.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TriggerObject"><code>TriggerObject</code></a>

This specifies whether the trigger function should be fired once for every row affected by the trigger event, or just once per SQL statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.include_each" class="anchor field">§</a>`include_each: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to include the `EACH` term of the `FOR EACH`, as it is optional syntax.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.condition" class="anchor field">§</a>`condition: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

Triggering conditions

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.exec_body" class="anchor field">§</a>`exec_body: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.TriggerExecBody.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::TriggerExecBody"><code>TriggerExecBody</code></a>`>`

Execute logic block

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.statements" class="anchor field">§</a>`statements: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ConditionalStatements.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ConditionalStatements"><code>ConditionalStatements</code></a>`>`

For SQL dialects with statement(s) for a body

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger.field.characteristics" class="anchor field">§</a>`characteristics: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ConstraintCharacteristics.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ConstraintCharacteristics"><code>ConstraintCharacteristics</code></a>`>`

The characteristic of the trigger, which include whether the trigger is `DEFERRABLE`, `INITIALLY DEFERRED`, or `INITIALLY IMMEDIATE`,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropTrigger" class="anchor">§</a>

### DropTrigger

DROP TRIGGER

``` sql
DROP TRIGGER [ IF EXISTS ] name ON table_name [ CASCADE | RESTRICT ]
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropTrigger.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropTrigger.field.trigger_name" class="anchor field">§</a>`trigger_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropTrigger.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropTrigger.field.option" class="anchor field">§</a>`option: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ReferentialAction.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ReferentialAction"><code>ReferentialAction</code></a>`>`

`CASCADE` or `RESTRICT`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateProcedure" class="anchor">§</a>

### CreateProcedure

``` sql
CREATE PROCEDURE
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateProcedure.field.or_alter" class="anchor field">§</a>`or_alter: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateProcedure.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateProcedure.field.params" class="anchor field">§</a>`params: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ProcedureParam.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ProcedureParam"><code>ProcedureParam</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateProcedure.field.language" class="anchor field">§</a>`language: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateProcedure.field.body" class="anchor field">§</a>`body: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.ConditionalStatements.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::ConditionalStatements"><code>ConditionalStatements</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateMacro" class="anchor">§</a>

### CreateMacro

``` sql
CREATE MACRO
```

Supported variants:

1.  [DuckDB](https://duckdb.org/docs/sql/statements/create_macro)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateMacro.field.or_replace" class="anchor field">§</a>`or_replace: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateMacro.field.temporary" class="anchor field">§</a>`temporary: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateMacro.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateMacro.field.args" class="anchor field">§</a>`args: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.MacroArg.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::MacroArg"><code>MacroArg</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateMacro.field.definition" class="anchor field">§</a>`definition: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.MacroDefinition.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::MacroDefinition"><code>MacroDefinition</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage" class="anchor">§</a>

### CreateStage

``` sql
CREATE STAGE
```

See <https://docs.snowflake.com/en/sql-reference/sql/create-stage>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage.field.or_replace" class="anchor field">§</a>`or_replace: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage.field.temporary" class="anchor field">§</a>`temporary: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage.field.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage.field.stage_params" class="anchor field">§</a>`stage_params: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/stmt_data_loading/struct.StageParamsObject.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::stmt_data_loading::StageParamsObject"><code>StageParamsObject</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage.field.directory_table_params" class="anchor field">§</a>`directory_table_params: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/key_value_options/struct.KeyValueOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::key_value_options::KeyValueOptions"><code>KeyValueOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage.field.file_format" class="anchor field">§</a>`file_format: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/key_value_options/struct.KeyValueOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::key_value_options::KeyValueOptions"><code>KeyValueOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage.field.copy_options" class="anchor field">§</a>`copy_options: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/key_value_options/struct.KeyValueOptions.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::key_value_options::KeyValueOptions"><code>KeyValueOptions</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage.field.comment" class="anchor field">§</a>`comment: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Assert" class="anchor">§</a>

### Assert

``` sql
ASSERT <condition> [AS <message>]
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Assert.field.condition" class="anchor field">§</a>`condition: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Assert.field.message" class="anchor field">§</a>`message: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Grant" class="anchor">§</a>

### Grant

``` sql
GRANT privileges ON objects TO grantees
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Grant.field.privileges" class="anchor field">§</a>`privileges: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Privileges.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Privileges"><code>Privileges</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Grant.field.objects" class="anchor field">§</a>`objects: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.GrantObjects.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::GrantObjects"><code>GrantObjects</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Grant.field.grantees" class="anchor field">§</a>`grantees: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Grantee.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Grantee"><code>Grantee</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Grant.field.with_grant_option" class="anchor field">§</a>`with_grant_option: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Grant.field.as_grantor" class="anchor field">§</a>`as_grantor: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Grant.field.granted_by" class="anchor field">§</a>`granted_by: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Grant.field.current_grants" class="anchor field">§</a>`current_grants: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CurrentGrantsKind.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CurrentGrantsKind"><code>CurrentGrantsKind</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Deny" class="anchor">§</a>

### Deny(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.DenyStatement.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::DenyStatement">DenyStatement</a>)

``` sql
DENY privileges ON object TO grantees
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Revoke" class="anchor">§</a>

### Revoke

``` sql
REVOKE privileges ON objects FROM grantees
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Revoke.field.privileges" class="anchor field">§</a>`privileges: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Privileges.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Privileges"><code>Privileges</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Revoke.field.objects" class="anchor field">§</a>`objects: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.GrantObjects.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::GrantObjects"><code>GrantObjects</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Revoke.field.grantees" class="anchor field">§</a>`grantees: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Grantee.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Grantee"><code>Grantee</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Revoke.field.granted_by" class="anchor field">§</a>`granted_by: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Revoke.field.cascade" class="anchor field">§</a>`cascade: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.CascadeOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::CascadeOption"><code>CascadeOption</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Deallocate" class="anchor">§</a>

### Deallocate

``` sql
DEALLOCATE [ PREPARE ] { name | ALL }
```

Note: this is a PostgreSQL-specific statement.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Deallocate.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Deallocate.field.prepare" class="anchor field">§</a>`prepare: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Execute" class="anchor">§</a>

### Execute

``` sql
An `EXECUTE` statement
```

Postgres: <https://www.postgresql.org/docs/current/sql-execute.html> MSSQL: <https://learn.microsoft.com/en-us/sql/relational-databases/stored-procedures/execute-a-stored-procedure> BigQuery: <https://cloud.google.com/bigquery/docs/reference/standard-sql/procedural-language#execute_immediate> Snowflake: <https://docs.snowflake.com/en/sql-reference/sql/execute-immediate>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Execute.field.name" class="anchor field">§</a>`name: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Execute.field.parameters" class="anchor field">§</a>`parameters: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Execute.field.has_parentheses" class="anchor field">§</a>`has_parentheses: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Execute.field.immediate" class="anchor field">§</a>`immediate: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Is this an `EXECUTE IMMEDIATE`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Execute.field.into" class="anchor field">§</a>`into: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Execute.field.using" class="anchor field">§</a>`using: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ExprWithAlias.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ExprWithAlias"><code>ExprWithAlias</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Execute.field.output" class="anchor field">§</a>`output: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether the last parameter is the return value of the procedure MSSQL: <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/execute-transact-sql?view=sql-server-ver17#output>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Execute.field.default" class="anchor field">§</a>`default: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Whether to invoke the procedure with the default parameter values MSSQL: <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/execute-transact-sql?view=sql-server-ver17#default>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Prepare" class="anchor">§</a>

### Prepare

``` sql
PREPARE name [ ( data_type [, ...] ) ] AS statement
```

Note: this is a PostgreSQL-specific statement.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Prepare.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Prepare.field.data_types" class="anchor field">§</a>`data_types: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType"><code>DataType</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Prepare.field.statement" class="anchor field">§</a>`statement: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement"><code>Statement</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Kill" class="anchor">§</a>

### Kill

``` sql
KILL [CONNECTION | QUERY | MUTATION]
```

See <https://clickhouse.com/docs/en/sql-reference/statements/kill/> See <https://dev.mysql.com/doc/refman/8.0/en/kill.html>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Kill.field.modifier" class="anchor field">§</a>`modifier: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.KillType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::KillType"><code>KillType</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Kill.field.id" class="anchor field">§</a>`id: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ExplainTable" class="anchor">§</a>

### ExplainTable

``` sql
[EXPLAIN | DESC | DESCRIBE] TABLE
```

Note: this is a MySQL-specific statement. See <https://dev.mysql.com/doc/refman/8.0/en/explain.html>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ExplainTable.field.describe_alias" class="anchor field">§</a>`describe_alias: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DescribeAlias.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DescribeAlias"><code>DescribeAlias</code></a>

`EXPLAIN | DESC | DESCRIBE`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ExplainTable.field.hive_format" class="anchor field">§</a>`hive_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.HiveDescribeFormat.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::HiveDescribeFormat"><code>HiveDescribeFormat</code></a>`>`

Hive style `FORMATTED | EXTENDED`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ExplainTable.field.has_table_keyword" class="anchor field">§</a>`has_table_keyword: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Snowflake and ClickHouse support `DESC|DESCRIBE TABLE <table_name>` syntax

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/desc-table.html) [ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/describe-table)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ExplainTable.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

Table name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Explain" class="anchor">§</a>

### Explain

``` sql
[EXPLAIN | DESC | DESCRIBE]  <statement>
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Explain.field.describe_alias" class="anchor field">§</a>`describe_alias: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DescribeAlias.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DescribeAlias"><code>DescribeAlias</code></a>

`EXPLAIN | DESC | DESCRIBE`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Explain.field.analyze" class="anchor field">§</a>`analyze: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Carry out the command and show actual run times and other statistics.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Explain.field.verbose" class="anchor field">§</a>`verbose: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Explain.field.query_plan" class="anchor field">§</a>`query_plan: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

`EXPLAIN QUERY PLAN` Display the query plan without running the query.

[SQLite](https://sqlite.org/lang_explain.html)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Explain.field.estimate" class="anchor field">§</a>`estimate: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

`EXPLAIN ESTIMATE` [Clickhouse](https://clickhouse.com/docs/en/sql-reference/statements/explain#explain-estimate)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Explain.field.statement" class="anchor field">§</a>`statement: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement"><code>Statement</code></a>`>`

A SQL query that specifies what to explain

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Explain.field.format" class="anchor field">§</a>`format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.AnalyzeFormat.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::AnalyzeFormat"><code>AnalyzeFormat</code></a>`>`

Optional output format of explain

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Explain.field.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.UtilityOption.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::UtilityOption"><code>UtilityOption</code></a>`>>`

Postgres style utility options, `(analyze, verbose true)`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Savepoint" class="anchor">§</a>

### Savepoint

``` sql
SAVEPOINT
```

Define a new savepoint within the current transaction

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Savepoint.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ReleaseSavepoint" class="anchor">§</a>

### ReleaseSavepoint

``` sql
RELEASE [ SAVEPOINT ] savepoint_name
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ReleaseSavepoint.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Merge" class="anchor">§</a>

### Merge

A `MERGE` statement.

``` sql
MERGE INTO <target_table> USING <source> ON <join_expr> { matchedClause | notMatchedClause } [ ... ]
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/merge) [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/dml-syntax#merge_statement) [MSSQL](https://learn.microsoft.com/en-us/sql/t-sql/statements/merge-transact-sql?view=sql-server-ver16)

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Merge.field.into" class="anchor field">§</a>`into: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

optional INTO keyword

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Merge.field.table" class="anchor field">§</a>`table: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TableFactor.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TableFactor"><code>TableFactor</code></a>

Specifies the table to merge

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Merge.field.source" class="anchor field">§</a>`source: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.TableFactor.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::TableFactor"><code>TableFactor</code></a>

Specifies the table or subquery to join with the target table

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Merge.field.on" class="anchor field">§</a>`on: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

Specifies the expression on which to join the target table and source

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Merge.field.clauses" class="anchor field">§</a>`clauses: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.MergeClause.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::MergeClause"><code>MergeClause</code></a>`>`

Specifies the actions to perform when values match or do not match.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Merge.field.output" class="anchor field">§</a>`output: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.OutputClause.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::OutputClause"><code>OutputClause</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Cache" class="anchor">§</a>

### Cache

``` sql
CACHE [ FLAG ] TABLE <table_name> [ OPTIONS('K1' = 'V1', 'K2' = V2) ] [ AS ] [ <query> ]
```

See [Spark SQL docs](https://docs.databricks.com/spark/latest/spark-sql/language-manual/sql-ref-syntax-aux-cache-cache-table.html) for more details.

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Cache.field.table_flag" class="anchor field">§</a>`table_flag: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

Table flag

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Cache.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

Table name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Cache.field.has_as" class="anchor field">§</a>`has_as: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Cache.field.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SqlOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SqlOption"><code>SqlOption</code></a>`>`

Table confs

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Cache.field.query" class="anchor field">§</a>`query: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Query.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Query"><code>Query</code></a>`>>`

Cache table as a Query

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.UNCache" class="anchor">§</a>

### UNCache

``` sql
UNCACHE TABLE [ IF EXISTS ]  <table_name>
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.UNCache.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

Table name

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.UNCache.field.if_exists" class="anchor field">§</a>`if_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSequence" class="anchor">§</a>

### CreateSequence

``` sql
CREATE [ { TEMPORARY | TEMP } ] SEQUENCE [ IF NOT EXISTS ] <sequence_name>
```

Define a new sequence:

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSequence.field.temporary" class="anchor field">§</a>`temporary: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSequence.field.if_not_exists" class="anchor field">§</a>`if_not_exists: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSequence.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSequence.field.data_type" class="anchor field">§</a>`data_type: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.DataType.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::DataType"><code>DataType</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSequence.field.sequence_options" class="anchor field">§</a>`sequence_options: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SequenceOptions.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SequenceOptions"><code>SequenceOptions</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSequence.field.owned_by" class="anchor field">§</a>`owned_by: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateDomain" class="anchor">§</a>

### CreateDomain(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.CreateDomain.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::CreateDomain">CreateDomain</a>)

A `CREATE DOMAIN` statement.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateType" class="anchor">§</a>

### CreateType

``` sql
CREATE TYPE <name>
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateType.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateType.field.representation" class="anchor field">§</a>`representation: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.UserDefinedTypeRepresentation.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::UserDefinedTypeRepresentation"><code>UserDefinedTypeRepresentation</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Pragma" class="anchor">§</a>

### Pragma

``` sql
PRAGMA <schema-name>.<pragma-name> = <pragma-value>
```

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Pragma.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Pragma.field.value" class="anchor field">§</a>`value: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Value.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Value"><code>Value</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Pragma.field.is_eq" class="anchor field">§</a>`is_eq: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LockTables" class="anchor">§</a>

### LockTables

``` sql
LOCK TABLES <table_name> [READ [LOCAL] | [LOW_PRIORITY] WRITE]
```

Note: this is a MySQL-specific statement. See <https://dev.mysql.com/doc/refman/8.0/en/lock-tables.html>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LockTables.field.tables" class="anchor field">§</a>`tables: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.LockTable.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::LockTable"><code>LockTable</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.UnlockTables" class="anchor">§</a>

### UnlockTables

``` sql
UNLOCK TABLES
```

Note: this is a MySQL-specific statement. See <https://dev.mysql.com/doc/refman/8.0/en/lock-tables.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Unload" class="anchor">§</a>

### Unload

``` sql
UNLOAD(statement) TO <destination> [ WITH options ]
```

See Redshift <https://docs.aws.amazon.com/redshift/latest/dg/r_UNLOAD.html> and

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Unload.field.query" class="anchor field">§</a>`query: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Query.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Query"><code>Query</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Unload.field.to" class="anchor field">§</a>`to: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Unload.field.with" class="anchor field">§</a>`with: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.SqlOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::SqlOption"><code>SqlOption</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.OptimizeTable" class="anchor">§</a>

### OptimizeTable

``` sql
OPTIMIZE TABLE [db.]name [ON CLUSTER cluster] [PARTITION partition | PARTITION ID 'partition_id'] [FINAL] [DEDUPLICATE [BY expression]]
```

See ClickHouse <https://clickhouse.com/docs/en/sql-reference/statements/optimize>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.OptimizeTable.field.name" class="anchor field">§</a>`name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.OptimizeTable.field.on_cluster" class="anchor field">§</a>`on_cluster: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.OptimizeTable.field.partition" class="anchor field">§</a>`partition: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Partition.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Partition"><code>Partition</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.OptimizeTable.field.include_final" class="anchor field">§</a>`include_final: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.OptimizeTable.field.deduplicate" class="anchor field">§</a>`deduplicate: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Deduplicate.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Deduplicate"><code>Deduplicate</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LISTEN" class="anchor">§</a>

### LISTEN

``` sql
LISTEN
```

listen for a notification channel

See Postgres <https://www.postgresql.org/docs/current/sql-listen.html>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LISTEN.field.channel" class="anchor field">§</a>`channel: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.UNLISTEN" class="anchor">§</a>

### UNLISTEN

``` sql
UNLISTEN
```

stop listening for a notification

See Postgres <https://www.postgresql.org/docs/current/sql-unlisten.html>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.UNLISTEN.field.channel" class="anchor field">§</a>`channel: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.NOTIFY" class="anchor">§</a>

### NOTIFY

``` sql
NOTIFY channel [ , payload ]
```

send a notification event together with an optional “payload” string to channel

See Postgres <https://www.postgresql.org/docs/current/sql-notify.html>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.NOTIFY.field.channel" class="anchor field">§</a>`channel: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.Ident.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::Ident"><code>Ident</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.NOTIFY.field.payload" class="anchor field">§</a>`payload: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LoadData" class="anchor">§</a>

### LoadData

``` sql
LOAD DATA [LOCAL] INPATH 'filepath' [OVERWRITE] INTO TABLE tablename
[PARTITION (partcol1=val1, partcol2=val2 ...)]
[INPUTFORMAT 'inputformat' SERDE 'serde']
```

Loading files into tables

See Hive <https://cwiki.apache.org/confluence/pages/viewpage.action?pageId=27362036#LanguageManualDML-Loadingfilesintotables>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LoadData.field.local" class="anchor field">§</a>`local: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LoadData.field.inpath" class="anchor field">§</a>`inpath: `<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LoadData.field.overwrite" class="anchor field">§</a>`overwrite: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LoadData.field.table_name" class="anchor field">§</a>`table_name: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ObjectName.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ObjectName"><code>ObjectName</code></a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LoadData.field.partitioned" class="anchor field">§</a>`partitioned: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LoadData.field.table_format" class="anchor field">§</a>`table_format: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.HiveLoadDataFormat.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::HiveLoadDataFormat"><code>HiveLoadDataFormat</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.RenameTable" class="anchor">§</a>

### RenameTable(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.RenameTable.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::RenameTable">RenameTable</a>\>)

``` sql
Rename TABLE tbl_name TO new_tbl_name[, tbl_name2 TO new_tbl_name2] ...
```

Renames one or more tables

See Mysql <https://dev.mysql.com/doc/refman/9.1/en/rename-table.html>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.List" class="anchor">§</a>

### List(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/stmt_data_loading/struct.FileStagingCommand.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand">FileStagingCommand</a>)

Snowflake `LIST` See: <https://docs.snowflake.com/en/sql-reference/sql/list>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Remove" class="anchor">§</a>

### Remove(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/stmt_data_loading/struct.FileStagingCommand.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::stmt_data_loading::FileStagingCommand">FileStagingCommand</a>)

Snowflake `REMOVE` See: <https://docs.snowflake.com/en/sql-reference/sql/remove>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.RaisError" class="anchor">§</a>

### RaisError

RaiseError (MSSQL) RAISERROR ( { msg_id \| msg_str \| @local_variable } { , severity , state } \[ , argument \[ , …n \] \] ) \[ WITH option \[ , …n \] \] See <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/raiserror-transact-sql?view=sql-server-ver16>

#### Fields

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.RaisError.field.message" class="anchor field">§</a>`message: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.RaisError.field.severity" class="anchor field">§</a>`severity: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.RaisError.field.state" class="anchor field">§</a>`state: `<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box"><code>Box</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.RaisError.field.arguments" class="anchor field">§</a>`arguments: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Expr.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Expr"><code>Expr</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.RaisError.field.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.RaisErrorOption.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::RaisErrorOption"><code>RaisErrorOption</code></a>`>`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Print" class="anchor">§</a>

### Print(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.PrintStatement.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::PrintStatement">PrintStatement</a>)

``` sql
PRINT msg_str | @local_variable | string_expr
```

See: <https://learn.microsoft.com/en-us/sql/t-sql/statements/print-transact-sql>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Return" class="anchor">§</a>

### Return(<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ReturnStatement.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::ReturnStatement">ReturnStatement</a>)

``` sql
RETURN [ expression ]
```

See [ReturnStatement](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/struct.ReturnStatement.html "struct datafusion::logical_expr::sqlparser::ast::ReturnStatement")

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-Clone-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-Debug-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-Display-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html" class="trait" title="trait core::fmt::Display">Display</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.fmt-1" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats a SQL statement with support for pretty printing.

When using the alternate flag (`{:#}`), the statement will be formatted with proper indentation and line breaks. For example:

``` rust
let sql = "SELECT a, b FROM table_1";
let ast = Parser::parse_sql(&GenericDialect, sql).unwrap();

// Regular formatting
assert_eq!(format!("{}", ast[0]), "SELECT a, b FROM table_1");

// Pretty printing
assert_eq!(format!("{:#}", ast[0]),
r#"SELECT
  a,
  b
FROM
  table_1"#);
```

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-From%3CSet%3E-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

Convert a `Set` into a `Statement`. Convenience function, instead of writing `Statement::Set(Set::Set...{...})`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(set: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Set.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Set">Set</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

Converts to this type from the input type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-Hash-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-Ord-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html" class="trait" title="trait core::cmp::Ord">Ord</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp" class="fn">cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>

This method returns an [`Ordering`](https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html "enum core::cmp::Ordering") between `self` and `other`. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#tymethod.cmp)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1021-1023" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.max" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max" class="fn">max</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the maximum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.max)

1.21.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1060-1062" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.min" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min" class="fn">min</a>(self, other: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Compares and returns the minimum of two values. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.min)

1.50.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1086-1088" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.clamp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp" class="fn">clamp</a>(self, min: Self, max: Self) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Restrict a value to a certain interval. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html#method.clamp)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-PartialEq-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-PartialOrd-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html" class="trait" title="trait core::cmp::PartialOrd">PartialOrd</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.partial_cmp" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp" class="fn">partial_cmp</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/cmp/enum.Ordering.html" class="enum" title="enum core::cmp::Ordering">Ordering</a>\>

This method returns an ordering between `self` and `other` values if one exists. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#tymethod.partial_cmp)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1398" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.lt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt" class="fn">lt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than (for `self` and `other`) and is used by the `<` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.lt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1416" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.le" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le" class="fn">le</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests less than or equal to (for `self` and `other`) and is used by the `<=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.le)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1434" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.gt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt" class="fn">gt</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than (for `self` and `other`) and is used by the `>` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.gt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#1452" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.ge" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge" class="fn">ge</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests greater than or equal to (for `self` and `other`) and is used by the `>=` operator. [Read more](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialOrd.html#method.ge)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-Spanned-for-Statement" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Spanned.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Spanned">Spanned</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

#### <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#partial-span" class="doc-anchor">§</a>partial span

Missing spans:

- [Statement::CopyIntoSnowflake](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CopyIntoSnowflake "variant datafusion::logical_expr::sqlparser::ast::Statement::CopyIntoSnowflake")
- [Statement::CreateSecret](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSecret "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateSecret")
- [Statement::CreateRole](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateRole "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateRole")
- [Statement::AlterType](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterType "variant datafusion::logical_expr::sqlparser::ast::Statement::AlterType")
- [Statement::AlterRole](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AlterRole "variant datafusion::logical_expr::sqlparser::ast::Statement::AlterRole")
- [Statement::AttachDatabase](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDatabase "variant datafusion::logical_expr::sqlparser::ast::Statement::AttachDatabase")
- [Statement::AttachDuckDBDatabase](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.AttachDuckDBDatabase "variant datafusion::logical_expr::sqlparser::ast::Statement::AttachDuckDBDatabase")
- [Statement::DetachDuckDBDatabase](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DetachDuckDBDatabase "variant datafusion::logical_expr::sqlparser::ast::Statement::DetachDuckDBDatabase")
- [Statement::Drop](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Drop "variant datafusion::logical_expr::sqlparser::ast::Statement::Drop")
- [Statement::DropFunction](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropFunction "variant datafusion::logical_expr::sqlparser::ast::Statement::DropFunction")
- [Statement::DropProcedure](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropProcedure "variant datafusion::logical_expr::sqlparser::ast::Statement::DropProcedure")
- [Statement::DropSecret](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropSecret "variant datafusion::logical_expr::sqlparser::ast::Statement::DropSecret")
- [Statement::Declare](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Declare "variant datafusion::logical_expr::sqlparser::ast::Statement::Declare")
- [Statement::CreateExtension](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateExtension "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateExtension")
- [Statement::Fetch](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Fetch "variant datafusion::logical_expr::sqlparser::ast::Statement::Fetch")
- [Statement::Flush](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Flush "variant datafusion::logical_expr::sqlparser::ast::Statement::Flush")
- [Statement::Discard](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Discard "variant datafusion::logical_expr::sqlparser::ast::Statement::Discard")
- [Statement::Set](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Set "variant datafusion::logical_expr::sqlparser::ast::Statement::Set")
- [Statement::ShowFunctions](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowFunctions "variant datafusion::logical_expr::sqlparser::ast::Statement::ShowFunctions")
- [Statement::ShowVariable](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowVariable "variant datafusion::logical_expr::sqlparser::ast::Statement::ShowVariable")
- [Statement::ShowStatus](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowStatus "variant datafusion::logical_expr::sqlparser::ast::Statement::ShowStatus")
- [Statement::ShowVariables](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowVariables "variant datafusion::logical_expr::sqlparser::ast::Statement::ShowVariables")
- [Statement::ShowCreate](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowCreate "variant datafusion::logical_expr::sqlparser::ast::Statement::ShowCreate")
- [Statement::ShowColumns](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowColumns "variant datafusion::logical_expr::sqlparser::ast::Statement::ShowColumns")
- [Statement::ShowTables](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowTables "variant datafusion::logical_expr::sqlparser::ast::Statement::ShowTables")
- [Statement::ShowCollation](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ShowCollation "variant datafusion::logical_expr::sqlparser::ast::Statement::ShowCollation")
- [Statement::StartTransaction](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.StartTransaction "variant datafusion::logical_expr::sqlparser::ast::Statement::StartTransaction")
- [Statement::Comment](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Comment "variant datafusion::logical_expr::sqlparser::ast::Statement::Comment")
- [Statement::Commit](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Commit "variant datafusion::logical_expr::sqlparser::ast::Statement::Commit")
- [Statement::Rollback](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Rollback "variant datafusion::logical_expr::sqlparser::ast::Statement::Rollback")
- [Statement::CreateSchema](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSchema "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateSchema")
- [Statement::CreateDatabase](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateDatabase "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateDatabase")
- [Statement::CreateFunction](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateFunction "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateFunction")
- [Statement::CreateTrigger](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateTrigger "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateTrigger")
- [Statement::DropTrigger](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.DropTrigger "variant datafusion::logical_expr::sqlparser::ast::Statement::DropTrigger")
- [Statement::CreateProcedure](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateProcedure "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateProcedure")
- [Statement::CreateMacro](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateMacro "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateMacro")
- [Statement::CreateStage](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateStage "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateStage")
- [Statement::Assert](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Assert "variant datafusion::logical_expr::sqlparser::ast::Statement::Assert")
- [Statement::Grant](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Grant "variant datafusion::logical_expr::sqlparser::ast::Statement::Grant")
- [Statement::Revoke](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Revoke "variant datafusion::logical_expr::sqlparser::ast::Statement::Revoke")
- [Statement::Deallocate](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Deallocate "variant datafusion::logical_expr::sqlparser::ast::Statement::Deallocate")
- [Statement::Execute](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Execute "variant datafusion::logical_expr::sqlparser::ast::Statement::Execute")
- [Statement::Prepare](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Prepare "variant datafusion::logical_expr::sqlparser::ast::Statement::Prepare")
- [Statement::Kill](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Kill "variant datafusion::logical_expr::sqlparser::ast::Statement::Kill")
- [Statement::ExplainTable](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ExplainTable "variant datafusion::logical_expr::sqlparser::ast::Statement::ExplainTable")
- [Statement::Explain](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Explain "variant datafusion::logical_expr::sqlparser::ast::Statement::Explain")
- [Statement::Savepoint](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Savepoint "variant datafusion::logical_expr::sqlparser::ast::Statement::Savepoint")
- [Statement::ReleaseSavepoint](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.ReleaseSavepoint "variant datafusion::logical_expr::sqlparser::ast::Statement::ReleaseSavepoint")
- [Statement::Merge](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Merge "variant datafusion::logical_expr::sqlparser::ast::Statement::Merge")
- [Statement::Cache](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Cache "variant datafusion::logical_expr::sqlparser::ast::Statement::Cache")
- [Statement::UNCache](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.UNCache "variant datafusion::logical_expr::sqlparser::ast::Statement::UNCache")
- [Statement::CreateSequence](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateSequence "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateSequence")
- [Statement::CreateType](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.CreateType "variant datafusion::logical_expr::sqlparser::ast::Statement::CreateType")
- [Statement::Pragma](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Pragma "variant datafusion::logical_expr::sqlparser::ast::Statement::Pragma")
- [Statement::LockTables](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.LockTables "variant datafusion::logical_expr::sqlparser::ast::Statement::LockTables")
- [Statement::UnlockTables](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.UnlockTables "variant datafusion::logical_expr::sqlparser::ast::Statement::UnlockTables")
- [Statement::Unload](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.Unload "variant datafusion::logical_expr::sqlparser::ast::Statement::Unload")
- [Statement::OptimizeTable](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#variant.OptimizeTable "variant datafusion::logical_expr::sqlparser::ast::Statement::OptimizeTable")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.span" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Spanned.html#tymethod.span" class="fn">span</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html" class="struct" title="struct datafusion::logical_expr::sqlparser::tokenizer::Span">Span</a>

Return the [`Span`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Span.html "struct datafusion::logical_expr::sqlparser::tokenizer::Span") (the minimum and maximum [`Location`](https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/tokenizer/struct.Location.html "struct datafusion::logical_expr::sqlparser::tokenizer::Location")) for this AST node, by recursively combining the spans of its children.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-TryFrom%3CStatement%3E-for-CreateTableBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/stmt_create_table/struct.CreateTableBuilder.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder">CreateTableBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#associatedtype.Error" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype">Error</a> = <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/parser/enum.ParserError.html" class="enum" title="enum datafusion::logical_expr::sqlparser::parser::ParserError">ParserError</a>

The type returned in the event of a conversion error.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.try_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#tymethod.try_from" class="fn">try_from</a>( stmt: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/stmt_create_table/struct.CreateTableBuilder.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder">CreateTableBuilder</a>, \<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/helpers/stmt_create_table/struct.CreateTableBuilder.html" class="struct" title="struct datafusion::logical_expr::sqlparser::ast::helpers::stmt_create_table::CreateTableBuilder">CreateTableBuilder</a> as <a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html" class="trait" title="trait core::convert::TryFrom">TryFrom</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>\>\>::<a href="https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html#associatedtype.Error" class="associatedtype" title="type core::convert::TryFrom::Error">Error</a>\>

Performs the conversion.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-Visit-for-Statement" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visit">Visit</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.visit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visit.html#tymethod.visit" class="fn">visit</a>\<V\>(&self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::Visitor::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.Visitor.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::Visitor">Visitor</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-VisitMut-for-Statement" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitMut">VisitMut</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#method.visit-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitMut.html#tymethod.visit" class="fn">visit</a>\<V\>(&mut self, visitor: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut V</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/ops/control_flow/enum.ControlFlow.html" class="enum" title="enum core::ops::control_flow::ControlFlow">ControlFlow</a>\<\<V as <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html#associatedtype.Break" class="associatedtype" title="type datafusion::logical_expr::sqlparser::ast::VisitorMut::Break">Break</a>\>

where V: <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/trait.VisitorMut.html" class="trait" title="trait datafusion::logical_expr::sqlparser::ast::VisitorMut">VisitorMut</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-Eq-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#impl-StructuralPartialEq-for-Statement" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html" class="enum" title="enum datafusion::logical_expr::sqlparser::ast::Statement">Statement</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/sqlparser/ast/enum.Statement.html#blanket-implementations" class="anchor">§</a>
