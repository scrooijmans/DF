# Configuration — Apache DataFusion Ballista  documentation
Ballista Configuration Settings
-------------------------------------------------------------------------------------------

Configuring Ballista is quite similar to configuring DataFusion. Most settings are identical, with only a few configurations specific to Ballista.

_Example: Specifying configuration options when creating a context_

```
use ballista::extension::{SessionConfigExt, SessionContextExt};

let session_config = SessionConfig::new_with_ballista()
    .with_information_schema(true)
    .with_ballista_job_name("Super Cool Ballista App");

let state = SessionStateBuilder::new()
    .with_default_features()
    .with_config(session_config)
    .build();

let ctx: SessionContext = SessionContext::remote_with_state(&url,state).await?;

```


`SessionConfig::new_with_ballista()` will setup `SessionConfig` for use with ballista. This is not required, `SessionConfig::new` could be used, but it’s advised as it will set up some sensible configuration defaults .

`SessionConfigExt` expose set of `SessionConfigExt::with_ballista_` and `SessionConfigExt::ballista_` methods which can tune retrieve ballista specific options.

Notable `SessionConfigExt` configuration methods would be:

```
/// Overrides ballista's [LogicalExtensionCodec]
fn with_ballista_logical_extension_codec(
    self,
    codec: Arc<dyn LogicalExtensionCodec>,
) -> SessionConfig;

/// Overrides ballista's [PhysicalExtensionCodec]
fn with_ballista_physical_extension_codec(
    self,
    codec: Arc<dyn PhysicalExtensionCodec>,
) -> SessionConfig;

/// Overrides ballista's [QueryPlanner]
fn with_ballista_query_planner(
    self,
    planner: Arc<dyn QueryPlanner + Send + Sync + 'static>,
) -> SessionConfig;

```


which could be used to change default ballista behavior.

If information schema is enabled all configuration parameters could be retrieved or set using SQL;

```
let ctx: SessionContext = SessionContext::remote_with_state(&url, state).await?;

let result = ctx
    .sql("select name, value from information_schema.df_settings where name like 'ballista'")
    .await?
    .collect()
    .await?;

let expected = [
    "+-------------------+-------------------------+",
    "| name              | value                   |",
    "+-------------------+-------------------------+",
    "| ballista.job.name | Super Cool Ballista App |",
    "+-------------------+-------------------------+",
];

```


Ballista Scheduler Configuration Settings
---------------------------------------------------------------------------------------------------------------

Besides the BallistaContext configuration settings, a few configuration settings for the Ballista scheduler to better manage the whole cluster are also needed to be taken care of.

_Example: Specifying configuration options when starting the scheduler_

```
./ballista-scheduler --scheduler-policy push-staged --event-loop-buffer-size 1000000 --executor-slots-policy
round-robin-local

```




* key: scheduler-policy
  * type: Utf8
  * default: pull-staged
  * description: Sets the task scheduling policy for the scheduler, possible values: pull-staged, push-staged.
* key: event-loop-buffer-size
  * type: UInt32
  * default: 10000
  * description: Sets the event loop buffer size. for a system of high throughput, a larger value like 1000000 is recommended.
* key: executor-slots-policy
  * type: Utf8
  * default: bias
  * description: Sets the executor slots policy for the scheduler, possible values: bias, round-robin, round-robin-local. For a cluster with single scheduler, round-robin-local is recommended.
* key: finished-job-data-clean-up-interval-seconds
  * type: UInt64
  * default: 300
  * description: Sets the delayed interval for cleaning up finished job data, mainly the shuffle data, 0 means the cleaning up is disabled.
* key: finished-job-state-clean-up-interval-seconds
  * type: UInt64
  * default: 3600
  * description: Sets the delayed interval for cleaning up finished job state stored in the backend, 0 means the cleaning up is disabled.
