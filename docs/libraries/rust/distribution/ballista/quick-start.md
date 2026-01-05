# Ballista Quickstart — Apache DataFusion Ballista  documentation
On this page

*   [Build the project](#build-the-project)
*   [Running the examples](#running-the-examples)
    *   [Distributed SQL Example](#distributed-sql-example)
        *   [Source code for distributed SQL example](#source-code-for-distributed-sql-example)
    *   [Distributed DataFrame Example](#distributed-dataframe-example)
        *   [Source code for distributed DataFrame example](#source-code-for-distributed-dataframe-example)

A simple way to start a local cluster for testing purposes is to use cargo to build the project and then run the scheduler and executor binaries directly.

Project Requirements:

*   [Rust](https://www.rust-lang.org/tools/install)
    
*   [Protobuf Compiler](https://protobuf.dev/downloads/)
    

Build the project
---------------------------------------------------------------

From the root of the project, build release binaries.

Start a Ballista scheduler process in a new terminal session.

```
RUST_LOG=info ./target/release/ballista-scheduler

```


Start one or more Ballista executor processes in new terminal sessions. When starting more than one executor, a unique port number must be specified for each executor.

```
RUST_LOG=info ./target/release/ballista-executor -c 2 -p 50051 --bind-grpc-port 50052

RUST_LOG=info ./target/release/ballista-executor -c 2 -p 50053 --bind-grpc-port 50054

```


Running the examples
---------------------------------------------------------------------

The examples can be run using the `cargo run --bin` syntax. Open a new terminal session and run the following commands.

### Distributed SQL Example

```
cd examples
cargo run --release --example remote-sql

```


#### Source code for distributed SQL example

```
use ballista::prelude::*;
use ballista_examples::test_util;
use datafusion::{
    execution::SessionStateBuilder,
    prelude::{CsvReadOptions, SessionConfig, SessionContext},
};

/// This example demonstrates executing a simple query against an Arrow data source (CSV) and
/// fetching results, using SQL
#[tokio::main]
async fn main() -> Result<()> {
    let config = SessionConfig::new_with_ballista()
        .with_target_partitions(4)
        .with_ballista_job_name("Remote SQL Example");

    let state = SessionStateBuilder::new()
        .with_config(config)
        .with_default_features()
        .build();

    let ctx = SessionContext::remote_with_state("df://localhost:50050", state).await?;

    let test_data = test_util::examples_test_data();

    ctx.register_csv(
        "test",
        &format!("{test_data}/aggregate_test_100.csv"),
        CsvReadOptions::new(),
    )
    .await?;

    let df = ctx
        .sql(
            "SELECT c1, MIN(c12), MAX(c12) \
        FROM test \
        WHERE c11 > 0.1 AND c11 < 0.9 \
        GROUP BY c1",
        )
        .await?;

    df.show().await?;

    Ok(())
}

```


### Distributed DataFrame Example

```
cd examples
cargo run --release --example remote-dataframe

```


#### Source code for distributed DataFrame example

```
use ballista::prelude::*;
use ballista_examples::test_util;
use datafusion::{
    prelude::{col, lit, ParquetReadOptions, SessionContext},
};

#[tokio::main]
async fn main() -> Result<()> {
    // creating SessionContext with default settings
    let ctx = SessionContext::remote("df://localhost:50050").await?;

    let test_data = test_util::examples_test_data();
    let filename = format!("{test_data}/alltypes_plain.parquet");

    let df = ctx
        .read_parquet(filename, ParquetReadOptions::default())
        .await?
        .select_columns(&["id", "bool_col", "timestamp_col"])?
        .filter(col("id").gt(lit(1)))?;

    df.show().await?;

    Ok(())
}

```
# Deploying a standalone Ballista cluster using cargo install — Apache DataFusion Ballista  documentation
A simple way to start a local cluster for testing purposes is to use cargo to install the scheduler and executor crates.

```
cargo install --locked ballista-scheduler
cargo install --locked ballista-executor

```


With these crates installed, it is now possible to start a scheduler process.

```
RUST_LOG=info ballista-scheduler

```


The scheduler will bind to port 50050 by default.

Next, start an executor processes in a new terminal session.

```
RUST_LOG=info ballista-executor

```


The executor will bind to port 50051 by default. Additional executors can be started by manually specifying a bind port. For example:

```
RUST_LOG=info ballista-executor --bind-port 50052

```
# Starting a Ballista Cluster using Docker — Apache DataFusion Ballista  documentation
Build Docker Images
-------------------------------------------------------------------

Run the following commands to download the [official Docker image](https://github.com/apache/datafusion-ballista/pkgs/container/datafusion-ballista-standalone):

```
docker pull ghcr.io/apache/datafusion-ballista-standalone:latest

```


Altenatively run the following commands to clone the source repository and build the Docker images from source:

```
git clone git@github.com:apache/datafusion-ballista.git
cd datafusion-ballista
./dev/build-ballista-docker.sh

```


This will create the following images:

*   `apache/datafusion-ballista-benchmarks:latest`
    
*   `apache/datafusion-ballista-cli:latest`
    
*   `apache/datafusion-ballista-executor:latest`
    
*   `apache/datafusion-ballista-scheduler:latest`
    
*   `apache/datafusion-ballista-standalone:latest`
    

Start a Cluster
-----------------------------------------------------------

### Start a Scheduler

Start a scheduler using the following syntax:

```
docker run --network=host \
 -d apache/datafusion-ballista-scheduler:latest \
 --bind-port 50050

```


Run `docker ps` to check that the process is running:

```
$ docker ps
CONTAINER ID   IMAGE                                    COMMAND                  CREATED         STATUS         PORTS     NAMES
a756055576f3   apache/datafusion-ballista-scheduler:latest   "/root/scheduler-ent…"   8 seconds ago   Up 8 seconds             xenodochial_carson

```


Run `docker logs CONTAINER_ID` to check the output from the process:

```
$ docker logs a756055576f3
2024-02-03T14:49:47.904571Z  INFO main ThreadId(01) ballista_scheduler::cluster: Initializing Sled database in temp directory

2024-02-03T14:49:47.924679Z  INFO main ThreadId(01) ballista_scheduler::scheduler_process: Ballista v0.12.0 Scheduler listening on 0.0.0.0:50050
2024-02-03T14:49:47.924709Z  INFO main ThreadId(01) ballista_scheduler::scheduler_process: Starting Scheduler grpc server with task scheduling policy of PullStaged
2024-02-03T14:49:47.925261Z  INFO main ThreadId(01) ballista_scheduler::cluster::kv: Initializing heartbeat listener
2024-02-03T14:49:47.925476Z  INFO main ThreadId(01) ballista_scheduler::scheduler_server::query_stage_scheduler: Starting QueryStageScheduler
2024-02-03T14:49:47.925587Z  INFO tokio-runtime-worker ThreadId(47) ballista_core::event_loop: Starting the event loop query_stage

```


### Start Executors

Start one or more executor processes. Each executor process will need to listen on a different port.

```
docker run --network=host \
  -d apache/datafusion-ballista-executor:latest \
  --external-host localhost --bind-port 50051

```


Use `docker ps` to check that both the scheduler and executor(s) are now running:

```
$ docker ps
CONTAINER ID   IMAGE                                    COMMAND                  CREATED         STATUS         PORTS     NAMES
fb8b530cee6d   apache/datafusion-ballista-executor:latest    "/root/executor-entr…"   2 seconds ago   Up 1 second              gallant_galois
a756055576f3   apache/datafusion-ballista-scheduler:latest   "/root/scheduler-ent…"   8 seconds ago   Up 8 seconds             xenodochial_carson

```


Use `docker logs CONTAINER_ID` to check the output from the executor(s):

```
$ docker logs fb8b530cee6d
2024-02-03T14:50:24.061607Z  INFO main ThreadId(01) ballista_executor::executor_process: Running with config:
2024-02-03T14:50:24.061649Z  INFO main ThreadId(01) ballista_executor::executor_process: work_dir: /tmp/.tmpAkP3pZ
2024-02-03T14:50:24.061655Z  INFO main ThreadId(01) ballista_executor::executor_process: concurrent_tasks: 48
2024-02-03T14:50:24.063256Z  INFO tokio-runtime-worker ThreadId(44) ballista_executor::executor_process: Ballista v0.12.0 Rust Executor Flight Server listening on 0.0.0.0:50051
2024-02-03T14:50:24.063281Z  INFO tokio-runtime-worker ThreadId(47) ballista_executor::execution_loop: Starting poll work loop with scheduler

```


Connect from the CLI
---------------------------------------------------------------------

```
docker run --network=host -it apache/datafusion-ballista-cli:latest --host localhost --port 50050

```
# Starting a Ballista Cluster using Docker Compose — Apache DataFusion Ballista  documentation
Docker Compose is a convenient way to launch a cluster when testing locally.

Build Docker Images
-------------------------------------------------------------------

Run the following commands to download the [official Docker image](https://github.com/apache/datafusion-ballista/pkgs/container/datafusion-ballista-standalone):

```
docker pull ghcr.io/apache/datafusion-ballista-standalone:latest

```


Altenatively run the following commands to clone the source repository and build the Docker images from source:

```
git clone git@github.com:apache/datafusion-ballista.git -b latest
cd datafusion-ballista
./dev/build-ballista-docker.sh

```


This will create the following images:

*   `apache/datafusion-ballista-benchmarks:latest`
    
*   `apache/datafusion-ballista-cli:latest`
    
*   `apache/datafusion-ballista-executor:latest`
    
*   `apache/datafusion-ballista-scheduler:latest`
    
*   `apache/datafusion-ballista-standalone:latest`
    

Start a Cluster
-----------------------------------------------------------

Using the [docker-compose.yml](https://github.com/apache/datafusion-ballista/blob/main/docker-compose.yml) from the source repository, run the following command to start a cluster:

```
docker-compose up --build

```


This should show output similar to the following:

```
$ docker-compose up
Creating network "ballista-benchmarks_default" with the default driver
Creating ballista-benchmarks_etcd_1 ... done
Creating ballista-benchmarks_ballista-scheduler_1 ... done
Creating ballista-benchmarks_ballista-executor_1  ... done
Attaching to ballista-benchmarks_etcd_1, ballista-benchmarks_ballista-scheduler_1, ballista-benchmarks_ballista-executor_1
ballista-executor_1   | [2021-08-28T15:55:22Z INFO  ballista_executor] Running with config:
ballista-executor_1   | [2021-08-28T15:55:22Z INFO  ballista_executor] work_dir: /tmp/.tmpLVx39c
ballista-executor_1   | [2021-08-28T15:55:22Z INFO  ballista_executor] concurrent_tasks: 4
ballista-scheduler_1  | [2021-08-28T15:55:22Z INFO  ballista_scheduler] Ballista v0.12.0 Scheduler listening on 0.0.0.0:50050
ballista-executor_1   | [2021-08-28T15:55:22Z INFO  ballista_executor] Ballista v0.12.0 Rust Executor listening on 0.0.0.0:50051

```


The scheduler listens on port 50050 and this is the port that clients will need to connect to.

Connect from the Ballista CLI
---------------------------------------------------------------------------------------

```
docker run --network=host -it apache/datafusion-ballista-cli:latest --host localhost --port 50050

```
# Deploying Ballista with Kubernetes — Apache DataFusion Ballista  documentation
Ballista can be deployed to any Kubernetes cluster using the following instructions. These instructions assume that you are already comfortable managing Kubernetes deployments.

The Ballista deployment consists of:

*   k8s deployment for one or more scheduler processes
    
*   k8s deployment for one or more executor processes
    
*   k8s service to route traffic to the schedulers
    
*   k8s persistent volume and persistent volume claims to make local data accessible to Ballista
    
*   _(optional)_ a [keda](http://keda.sh/) instance for autoscaling the number of executors
    

Testing Locally
-----------------------------------------------------------

[Microk8s](https://microk8s.io/) is recommended for installing a local k8s cluster. Once Microk8s is installed, DNS must be enabled using the following command.

Build Docker Images
-------------------------------------------------------------------

Run the following commands to download the [official Docker image](https://github.com/apache/datafusion-ballista/pkgs/container/datafusion-ballista-standalone):

```
docker pull ghcr.io/apache/datafusion-ballista-standalone:0.12.0-rc4

```


Altenatively run the following commands to clone the source repository and build the Docker images from source:

```
git clone git@github.com:apache/datafusion-ballista.git -b 0.12.0
cd datafusion-ballista
./dev/build-ballista-docker.sh

```


This will create the following images:

*   `apache/datafusion-ballista-benchmarks:0.12.0`
    
*   `apache/datafusion-ballista-cli:0.12.0`
    
*   `apache/datafusion-ballista-executor:0.12.0`
    
*   `apache/datafusion-ballista-scheduler:0.12.0`
    
*   `apache/datafusion-ballista-standalone:0.12.0`
    

Publishing Docker Images
-----------------------------------------------------------------------------

Once the images have been built, you can retag them and can push them to your favourite Docker registry.

```
docker tag apache/datafusion-ballista-scheduler:0.12.0 <your-repo>/datafusion-ballista-scheduler:0.12.0
docker tag apache/datafusion-ballista-executor:0.12.0 <your-repo>/datafusion-ballista-executor:0.12.0
docker push <your-repo>/datafusion-ballista-scheduler:0.12.0
docker push <your-repo>/datafusion-ballista-executor:0.12.0

```


Create Persistent Volume and Persistent Volume Claim
-------------------------------------------------------------------------------------------------------------------------------------

Copy the following yaml to a `pv.yaml` file and apply to the cluster to create a persistent volume and a persistent volume claim so that the specified host directory is available to the containers. This is where any data should be located so that Ballista can execute queries against it.

```
apiVersion: v1
kind: PersistentVolume
metadata:
  name: data-pv
  labels:
    type: local
spec:
  storageClassName: manual
  capacity:
    storage: 10Gi
  accessModes:
    - ReadWriteOnce
  hostPath:
    path: "/mnt"
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: data-pv-claim
spec:
  storageClassName: manual
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 3Gi

```


To apply this yaml:

You should see the following output:

```
persistentvolume/data-pv created
persistentvolumeclaim/data-pv-claim created

```


Deploying a Ballista Cluster
-------------------------------------------------------------------------------------

Copy the following yaml to a `cluster.yaml` file and change `<your-image>` with the name of your Ballista Docker image.

```
apiVersion: v1
kind: Service
metadata:
  name: ballista-scheduler
  labels:
    app: ballista-scheduler
spec:
  ports:
    - port: 50050
      name: scheduler
    - port: 80
      name: scheduler-ui
  selector:
    app: ballista-scheduler
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ballista-scheduler
spec:
  replicas: 1
  selector:
    matchLabels:
      app: ballista-scheduler
  template:
    metadata:
      labels:
        app: ballista-scheduler
        ballista-cluster: ballista
    spec:
      containers:
        - name: ballista-scheduler
          image: <your-repo>/datafusion-ballista-scheduler:0.12.0
          args: ["--bind-port=50050"]
          ports:
            - containerPort: 50050
              name: flight
          volumeMounts:
            - mountPath: /mnt
              name: data
      volumes:
        - name: data
          persistentVolumeClaim:
            claimName: data-pv-claim
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ballista-executor
spec:
  replicas: 2
  selector:
    matchLabels:
      app: ballista-executor
  template:
    metadata:
      labels:
        app: ballista-executor
        ballista-cluster: ballista
    spec:
      containers:
        - name: ballista-executor
          image: <your-repo>/datafusion-ballista-executor:0.12.0
          args:
            - "--bind-port=50051"
            - "--scheduler-host=ballista-scheduler"
            - "--scheduler-port=50050"
          ports:
            - containerPort: 50051
              name: flight
          volumeMounts:
            - mountPath: /mnt
              name: data
      volumes:
        - name: data
          persistentVolumeClaim:
            claimName: data-pv-claim

```


```
kubectl apply -f cluster.yaml

```


This should show the following output:

```
service/ballista-scheduler created
deployment.apps/ballista-scheduler created
deployment.apps/ballista-executor created

```


You can also check status by running `kubectl get pods`:

```
$ kubectl get pods
NAME                                 READY   STATUS    RESTARTS   AGE
ballista-executor-78cc5b6486-4rkn4   0/1     Pending   0          42s
ballista-executor-78cc5b6486-7crdm   0/1     Pending   0          42s
ballista-scheduler-879f874c5-rnbd6   0/1     Pending   0          42s

```


You can view the scheduler logs with `kubectl logs ballista-scheduler-0`:

```
$ kubectl logs ballista-scheduler-0
[2021-02-19T00:24:01Z INFO  scheduler] Ballista v0.7.0 Scheduler listening on 0.0.0.0:50050
[2021-02-19T00:24:16Z INFO  ballista::scheduler] Received register_executor request for ExecutorMetadata { id: "b5e81711-1c5c-46ec-8522-d8b359793188", host: "10.1.23.149", port: 50051 }
[2021-02-19T00:24:17Z INFO  ballista::scheduler] Received register_executor request for ExecutorMetadata { id: "816e4502-a876-4ed8-b33f-86d243dcf63f", host: "10.1.23.150", port: 50051 }

```


Port Forwarding
-----------------------------------------------------------

If you want to run applications outside of the cluster and have them connect to the scheduler then it is necessary to set up port forwarding.

First, check that the `ballista-scheduler` service is running.

```
$ kubectl get services
NAME                 TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)     AGE
kubernetes           ClusterIP   10.152.183.1    <none>        443/TCP     26h
ballista-scheduler   ClusterIP   10.152.183.21   <none>        50050/TCP   24m

```


Use the following command to set up port-forwarding.

```
kubectl port-forward service/ballista-scheduler 50050:50050

```


Deleting the Ballista Cluster
---------------------------------------------------------------------------------------

Run the following kubectl command to delete the cluster.

```
kubectl delete -f cluster.yaml

```


Autoscaling Executors
-----------------------------------------------------------------------

Ballista supports autoscaling for executors through [Keda](http://keda.sh/). Keda allows scaling a deployment through custom metrics which are exposed through the Ballista scheduler, and it can even scale the number of executors down to 0 if there is no activity in the cluster.

Keda can be installed in your kubernetes cluster through a single command line:

```
kubectl apply -f https://github.com/kedacore/keda/releases/download/v2.7.1/keda-2.7.1.yaml

```


Once you have deployed Keda on your cluster, you can now deploy a new kubernetes object called `ScaledObject` which will let Keda know how to scale your executors. In order to do that, copy the following YAML into a `scale.yaml` file:

```
apiVersion: keda.sh/v1alpha1
kind: ScaledObject
metadata:
  name: ballista-executor
spec:
  scaleTargetRef:
    name: ballista-executor
  minReplicaCount: 0
  maxReplicaCount: 5
  triggers:
    - type: external
      metadata:
        # Change this DNS if the scheduler isn't deployed in the "default" namespace
        scalerAddress: ballista-scheduler.default.svc.cluster.local:50050

```


And then deploy it into the cluster:

```
kubectl apply -f scale.yaml

```


If the cluster is inactive, Keda will now scale the number of executors down to 0, and will scale them up when you launch a query. Please note that Keda will perform a scan once every 30 seconds, so it might take a bit to scale the executors.

Please visit Keda’s [documentation page](https://keda.sh/docs/2.7/concepts/scaling-deployments/) for more information.