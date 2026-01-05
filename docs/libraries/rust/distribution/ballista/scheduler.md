# Ballista Scheduler — Apache DataFusion Ballista  documentation
REST API
---------------------------------------------

The scheduler also provides a REST API that allows jobs to be monitored.

> This is optional scheduler feature which should be enabled with `rest-api` feature



* API: /api/jobs
  * Method: GET
  * Description: Get a list of jobs that have been submitted to the cluster.
* API: /api/job/{job_id}
  * Method: GET
  * Description: Get a summary of a submitted job.
* API: /api/job/{job_id}/dot
  * Method: GET
  * Description: Produce a query plan in DOT (graphviz) format.
* API: /api/job/:job_id/dot_svg
  * Method: GET
  * Description: Produce a query plan in SVG format. (graphviz-support required)
* API: /api/job/{job_id}
  * Method: PATCH
  * Description: Cancel a currently running job
* API: /api/job/:job_id/stage/:stage_id/dot
  * Method: GET
  * Description: Produces stage plan in DOT (graphviz) format
* API: /api/metrics
  * Method: GET
  * Description: Return current scheduler metric set


# Ballista Scheduler Metrics — Apache DataFusion Ballista  documentation
Prometheus
-------------------------------------------------

> This is optional scheduler feature which should be enabled with `prometheus-metrics` feature

Built with default features, the ballista scheduler will automatically collect and expose a standard set of prometheus metrics. The metrics currently collected automatically include:

*   _job\_exec\_time\_seconds_ - Histogram of successful job execution time in seconds
    
*   _planning\_time\_ms_ - Histogram of job planning time in milliseconds
    
*   _failed_ - Counter of failed jobs
    
*   _job\_failed\_total_ - Counter of failed jobs
    
*   _job\_cancelled\_total_ - Counter of cancelled jobs
    
*   _job\_completed\_total_ - Counter of completed jobs
    
*   _job\_submitted\_total_ - Counter of submitted jobs
    
*   _pending\_task\_queue\_size_ - Number of pending tasks
    

**NOTE** Currently the histogram buckets for the above metrics are set to reasonable defaults. If the defaults are not appropriate for a given use case, the only workaround is to implement a customer `SchedulerMetricsCollector`. In the future the buckets should be made configurable.

The metrics are then exported through the scheduler REST API at `GET /api/metrics`. It should be sufficient to ingest metrics into an existing metrics system by point your chosen prometheus exporter at that endpoint.
