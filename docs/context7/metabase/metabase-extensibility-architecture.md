# Metabase Extensibility Architecture: Custom Drivers, Visualizations, and Query Processors

## Overview

Metabase provides a comprehensive extensibility architecture that allows developers to create custom drivers, visualization types, and query processor extensions. The system is designed to maintain stability and compatibility as extensions evolve through versioning, migration systems, and hierarchical inheritance.

## Custom Driver Extensibility

### Driver Plugin System

Metabase drivers are implemented as plugins with a manifest-based configuration system:

#### Plugin Manifest (YAML)

Drivers are defined using a `metabase-plugin.yaml` manifest:

```yaml
info:
  name: Metabase SQLite Driver
  version: 1.0.0-SNAPSHOT-3.25.2
  description: Allows Metabase to connect to SQLite databases.

contact-info:
  name: Toucan McBird
  address: toucan.mcbird@example.com

driver:
  name: sqlite
  display-name: SQLite
  lazy-load: true
  parent: sql-jdbc
  connection-properties:
    - name: db
      display-name: Filename
      placeholder: /home/camsaul/toucan_sightings.sqlite
      required: true

init:
  - step: load-namespace
    namespace: metabase.driver.sqlite
  - step: register-jdbc-driver
    class: org.sqlite.JDBC
```

#### Plugin Registration

Plugins are registered with lazy loading support:

```clojure
(defn register-lazy-loaded-driver!
  [{:keys [add-to-classpath!]
    init-steps :init
    {driver-name :name, :keys [abstract display-name parent]} :driver}]
  (let [driver (keyword driver-name)
        connection-props (parse-connection-properties driver-info)]
    ;; Validate required properties
    (when-not (seq driver-name)
      (throw (ex-info "Cannot initialize plugin: missing required property `driver-name`"
                      driver-info)))
    
    ;; Register driver multimethods
    (doseq [[^MultiFn multifn, f]
            {driver/initialize!           (make-initialize! driver add-to-classpath! init-steps)
             driver/display-name          (when display-name (constantly display-name))
             driver/connection-properties (constantly connection-props)}]
      (when f
        (.addMethod multifn driver f)))
    
    ;; Register the driver
    (driver/register! driver 
                      :parent (set (map keyword (u/one-or-many parent))) 
                      :abstract? abstract)))
```

### Driver Multimethod System

Drivers use Clojure's multimethod dispatch for extensibility:

#### Multimethod Definition

```clojure
(defmulti display-name
  {:added "0.32.0" :arglists '([driver])}
  dispatch-on-uninitialized-driver
  :hierarchy #'hierarchy)
```

#### Driver Implementation

```clojure
(ns com.mycompany.metabase.driver.foxpro98
  (:require [metabase.driver :as driver]))

(defmethod driver/display-name :foxpro98 [_]
  "Visual FoxPro '98")
```

### Driver Hierarchy and Inheritance

Metabase supports hierarchical driver inheritance:

#### Hierarchy Structure

```
:metabase.driver/driver (root)
  ├─ :sql (abstract)
  │   ├─ :sql-jdbc (abstract)
  │   │   ├─ :mysql
  │   │   ├─ :postgres
  │   │   └─ :h2
  │   └─ :bigquery
  └─ :mongo
```

#### Driver Registration

```clojure
;; Register with single parent
(driver/register! :h2, :parent :sql-jdbc)

;; Register with multiple parents
(driver/register! :bigquery, :parent #{:sql :google})

;; Register abstract driver
(driver/register! :sql-jdbc, :parent :sql, :abstract? true)
```

#### Parent Method Calling

Correct way to call parent implementations:

```clojure
(defmethod driver/mbql->native :bigquery [driver query]
  ;; CORRECT: Pass the driver argument to parent
  ((get-method driver/mbql->native :sql) driver query))
```

**Incorrect approaches:**
```clojure
;; BAD: Hardcoded parent driver
((get-method driver/mbql->native :sql) :sql query)

;; BAD: Hardcoded child driver
((get-method driver/mbql->native :sql) :bigquery query)
```

### Driver Initialization

Drivers support lazy initialization:

#### Initialization Function

```clojure
(defn- make-initialize! [driver add-to-classpath! init-steps]
  (fn [_]
    ;; Add driver to classpath
    (when add-to-classpath!
      (add-to-classpath!))
    
    ;; Remove this implementation to prevent recursion
    (remove-method driver/initialize! driver)
    
    ;; Execute initialization steps
    (init-steps/do-init-steps! init-steps)
    
    ;; Call driver's actual initialize! if it exists
    (driver/initialize! driver)))
```

#### Initialization Steps

Initialization steps are defined in the manifest:

```yaml
init:
  - step: load-namespace
    namespace: metabase.driver.sqlite
  - step: register-jdbc-driver
    class: org.sqlite.JDBC
```

#### Conditional Initialization

Drivers are initialized only when needed:

```clojure
(defn initialize-if-needed!
  [driver init-fn]
  (when-not (initialized? driver)
    (locking initialization-lock
      (when-not (initialized? driver)
        ;; Initialize parents first
        (doseq [parent (parents hierarchy driver)]
          (initialize-if-needed! parent init-fn))
        
        (log/info "Initializing driver %s..." driver)
        (init-fn driver)
        (swap! initialized-drivers conj driver)))))
```

### Driver Dependencies

Plugins can declare dependencies:

#### Dependency Types

```yaml
dependencies:
  # Class dependency - checks for class on classpath
  - class: oracle.jdbc.OracleDriver
    message: >
      Metabase requires the Oracle JDBC driver to connect to JDBC databases.
      See https://metabase.com/docs/... for more details
  
  # Plugin dependency - checks for other Metabase plugins
  - plugin: Metabase SQLHeavy Driver
```

#### Dependency Resolution

Dependencies are resolved lazily:

```clojure
;; If dependency not available, plugin initialization is deferred
;; Once dependency is loaded, plugin is retried
```

### Driver Supersession

Drivers can be deprecated and superseded:

```clojure
(defmulti superseded-by
  {:added "0.41.0" :arglists '([driver])}
  dispatch-on-uninitialized-driver
  :hierarchy #'hierarchy)

(defmethod superseded-by :default [_] nil)
```

**Usage:**
```yaml
driver:
  name: old-driver
  superseded-by: new-driver
```

## Query Processor Extensibility

### Middleware System

The query processor uses a middleware-based architecture:

#### Middleware List

```clojure
(def ^:private setup-middleware
  [#'do-with-canceled-chan
   #'do-with-database-local-settings
   #'do-with-driver
   #'do-with-metadata-provider
   #'do-with-resolved-database])
```

#### Middleware Application

```clojure
(defn- apply-middleware [qp middleware-fns]
  (reduce
   (fn [qp middleware]
     (if middleware
       (middleware qp)
       qp))
   qp
   middleware-fns))
```

### Middleware Wrapping

Middleware can wrap query processors:

#### PMBQL Wrapper

```clojure
(defn- ensure-pmbql [middleware-fn]
  (-> (fn [query]
        (let [query (cond->> query
                      (not (:lib/type query)) 
                      (lib/query (qp.store/metadata-provider)))]
          (vary-meta (middleware-fn query)
                     assoc :converted-form query)))
      (with-meta (meta middleware-fn))))
```

#### Legacy MBQL Wrapper

```clojure
(defn- ^:deprecated ensure-legacy [middleware-fn]
  (-> (fn [query]
        (let [query (cond-> query
                      (:lib/type query) ->legacy)]
          (vary-meta (middleware-fn query)
                     assoc :converted-form query)))
      (with-meta (meta middleware-fn))))
```

### Dynamic Middleware Updates

Middleware can be watched for changes:

```clojure
(doseq [varr around-middleware
        :when varr]
  (add-watch varr ::reload 
    (fn [_key _ref _old-state _new-state]
      (log/infof "%s changed, rebuilding %s" varr `process-query*)
      (rebuild-process-query-fn!))))
```

### Custom Middleware Example

```clojure
(defn wrap-value-literals
  "Middleware that wraps raw value literals in :value clauses."
  [{query-type :type, :as query}]
  (if-not (= query-type :query)
    query
    (update query :query wrap-value-literals-in-mbql-query nil)))
```

## Visualization Extensibility

### Visualization Multimethods

Visualizations use multimethod dispatch:

#### Render Multimethod

```clojure
(defmulti render
  {:arglists '([chart-type render-type timezone-id card dashcard data])}
  (fn [chart-type & _] chart-type))
```

#### Custom Visualization Implementation

```clojure
(mu/defmethod render :funnel :- ::RenderedPartCard
  [_chart-type render-type timezone-id card dashcard data]
  (let [viz-settings (get card :visualization_settings)]
    (if (= (get viz-settings :funnel.type) "bar")
      (render :javascript_visualization render-type timezone-id card dashcard data)
      (render :funnel_normal render-type timezone-id card dashcard data))))
```

### JavaScript Visualizations

Custom JavaScript visualizations are supported:

```clojure
(mu/defmethod render :javascript_visualization :- ::RenderedPartCard
  [_chart-type render-type _timezone-id card dashcard data]
  (let [series-cards-results (:series-results dashcard)
        cards-with-data (->> series-cards-results
                            (map raise-data-one-level)
                            (cons {:card card :data data})
                            (map add-dashcard-timeline-events)
                            (m/distinct-by #(get-in % [:card :id])))
        viz-settings (or (get dashcard :visualization_settings)
                        (get card :visualization_settings))
        {rendered-type :type content :content} 
        (js.svg/javascript-visualization cards-with-data viz-settings)]
    (case rendered-type
      :svg (let [image-bundle (image-bundle/make-image-bundle
                                render-type
                                (js.svg/svg-string->bytes content))]
             {:attachments (when image-bundle
                            (image-bundle/image-bundle->attachment image-bundle))
              :content [:div [:img {:style {:display :block :width :100%}
                                   :src (:image-src image-bundle)}]]})
      :html {:content [:div content] :attachments nil})))
```

### Visualization Bundle Loading

Visualization bundles are loaded into a GraalJS context:

```clojure
(defn- load-viz-bundle [^Context context]
  (doto context
    (js.engine/load-resource bundle-path)
    (js.engine/load-resource interface-path)))
```

## Compatibility and Stability

### Version Migration System

Metabase uses a migration system to maintain compatibility:

#### Visualization Settings Migration

```clojure
(defmethod ^:private migrate-viz-settings* [1 2] [viz-settings _]
  (let [{percent? :pie.show_legend_perecent
         legend?  :pie.show_legend} viz-settings
        new-visibility (cond
                        legend?  "inside"
                        percent? "legend")
        new-linktype (when (= "page" (-> viz-settings :click_behavior :linkType))
                      "dashboard")]
    (cond-> viz-settings
      new-visibility (assoc :pie.percent_visibility new-visibility)
      new-linktype   (assoc-in [:click_behavior :linkType] new-linktype))))
```

#### Database Migration

```clojure
(define-migration MigrateStackedAreaBarComboDisplaySettings
  (let [update! (fn [{:keys [id display visualization_settings] :as card}]
                  (t2/query-one {:update :report_card
                                 :set    {:visualization_settings visualization_settings
                                          :display                display}
                                 :where  [:= :id id]}))]
    (run! update! 
         (eduction 
          (keep (fn [{:keys [id display visualization_settings]}]
                  (let [parsed-viz (json/decode+kw visualization_settings)
                        partial-card {:display display 
                                     :visualization_settings parsed-viz}
                        updated-partial-card (update-stacked-viz-cards partial-card)]
                    (when (not= partial-card updated-partial-card)
                      {:id id
                       :display (name (:display updated-partial-card))
                       :visualization_settings 
                       (json/encode (:visualization_settings updated-partial-card))}))))
          (t2/reducible-query {:select [:id :display :visualization_settings]
                               :from   [:report_card]
                               :where  [:like :visualization_settings "%stackable%"]})))))
```

### Driver Feature Support

Drivers can declare feature support:

```clojure
(defmulti database-supports?
  {:added "0.32.0" :arglists '([driver feature database])}
  (fn [driver feature _database] [driver feature])
  :hierarchy #'hierarchy)

;; Example: MySQL supports actions
(doseq [feature [:actions :actions/custom]]
  (defmethod driver/database-supports? [:mysql feature]
    [driver _feat _db]
    (= driver :mysql)))
```

### HoneySQL Version Migration

Drivers can opt into newer query compilation:

```clojure
(require '[metabase.driver.sql.query-processor :as sql.qp])

;; Opt into HoneySQL 2
(defmethod sql.qp/honey-sql-version :my-driver
  [_driver]
  2)
```

### SDK Version Compatibility

Embedding SDK maintains version compatibility:

```bash
# Install SDK matching Metabase version
npm install @metabase/embedding-sdk-react@53-stable
```

**Version Tagging:**
- `53-stable` - Latest SDK for Metabase 53.x
- Ensures compatibility between SDK and Metabase instance

## Plugin Architecture

### Plugin Discovery

Plugins are discovered via manifest files:

```yaml
# metabase-plugin.yaml
info:
  name: Metabase SQLite Driver
  version: 1.0.0-SNAPSHOT-3.25.2
  description: Allows Metabase to connect to SQLite databases.
```

### Plugin Initialization

Initialization follows a defined sequence:

1. **Dependency Check** - Verify all dependencies are available
2. **Classpath Addition** - Add plugin JAR to classpath (if lazy-load)
3. **Namespace Loading** - Load Clojure namespaces
4. **Driver Registration** - Register JDBC drivers (if needed)
5. **Multimethod Registration** - Register driver multimethods
6. **Driver Registration** - Register driver with Metabase

### Connection Properties

Connection properties can use defaults or be customized:

```yaml
connection-properties:
  # Use default property
  - dbname
  - host
  
  # Custom property
  - name: db
    display-name: Filename
    placeholder: /home/camsaul/toucan_sightings.sqlite
    required: true
  
  # Merge default with customizations
  - merge:
    - port
    - placeholder: 1433
```

### JDBC Proxy Drivers

Metabase creates proxy drivers for JDBC compatibility:

```clojure
(defn create-and-register-proxy-driver!
  [^String class-name]
  (let [klass (Class/forName class-name true (classloader/the-classloader))
        loaded-by-system-classloader? 
        (identical? (.getClassLoader klass) 
                    (ClassLoader/getSystemClassLoader))]
    (if loaded-by-system-classloader?
      (log/debug "Not creating proxy JDBC driver...")
      (let [driver (proxy-driver (.newInstance klass))]
        (log/debug "Registering JDBC proxy driver...")
        (DriverManager/registerDriver driver)
        ;; Deregister original driver
        (doseq [driver (enumeration-seq (DriverManager/getDrivers))
                :when  (instance? klass driver)]
          (DriverManager/deregisterDriver driver))))))
```

## Extension Stability Mechanisms

### Hierarchical Inheritance

Driver hierarchy provides stable extension points:

```clojure
;; Abstract parent provides stable interface
(driver/register! :sql, :parent :metabase.driver/driver, :abstract? true)

;; Concrete drivers inherit and extend
(driver/register! :mysql, :parent :sql-jdbc)
```

**Benefits:**
- Parent implementations provide fallbacks
- Child drivers only override what's needed
- Changes to parent affect all children consistently

### Multimethod Dispatch

Multimethods enable stable extension:

```clojure
;; Core defines dispatch
(defmulti mbql->native
  {:added "0.32.0"}
  dispatch-on-uninitialized-driver
  :hierarchy #'hierarchy)

;; Extensions provide implementations
(defmethod mbql->native :mysql [driver query]
  ;; MySQL-specific implementation
  ...)
```

**Benefits:**
- Extensions don't modify core code
- New drivers can be added without core changes
- Method resolution follows hierarchy

### Version Tracking

Extensions track versions:

```yaml
info:
  version: 1.0.0-SNAPSHOT-3.25.2
```

**Version Components:**
- Major.Minor.Patch
- SNAPSHOT indicates development
- Includes dependency version (3.25.2 = JDBC driver version)

### Migration System

Migrations ensure compatibility across versions:

```clojure
;; Migration from version 1 to 2
(defmethod migrate-viz-settings* [1 2] [viz-settings _]
  ;; Transform old format to new format
  ...)
```

**Migration Benefits:**
- Old data formats continue to work
- Gradual migration path
- Backward compatibility maintained

### Lazy Loading

Drivers load only when needed:

```clojure
(defn initialize-if-needed!
  [driver init-fn]
  (when-not (initialized? driver)
    ;; Initialize only when first used
    ...))
```

**Benefits:**
- Faster startup time
- Reduced memory usage
- Extensions don't affect core performance

## Extension Development

### Driver Development

#### Basic Driver Structure

```clojure
(ns com.mycompany.metabase.driver.foxpro98
  (:require [metabase.driver :as driver]))

;; Register driver
(driver/register! :foxpro98, :parent :sql)

;; Implement required multimethods
(defmethod driver/display-name :foxpro98 [_]
  "Visual FoxPro '98")

(defmethod driver/mbql->native :foxpro98 [driver query]
  ;; Convert MBQL to native SQL
  ...)
```

#### Driver Testing

```clojure
;; Register test extensions
(sql-jdbc.tx/add-test-extensions! :mysql)

;; Run tests
DRIVERS=mysql clojure -X:dev:drivers:drivers-dev:test
```

### Query Processor Extension

#### Custom Middleware

```clojure
(defn my-custom-middleware
  [query-processor]
  (fn [query]
    ;; Pre-process query
    (let [modified-query (modify-query query)]
      ;; Call next middleware
      (query-processor modified-query))))
```

#### Middleware Registration

```clojure
;; Add to middleware list
(def ^:private setup-middleware
  [#'do-with-canceled-chan
   #'my-custom-middleware  ; Add here
   #'do-with-driver])
```

### Visualization Extension

#### Custom Chart Type

```clojure
(mu/defmethod render :my-custom-chart :- ::RenderedPartCard
  [_chart-type render-type timezone-id card dashcard data]
  ;; Render custom visualization
  {:content [:div "My Custom Chart"]
   :attachments nil})
```

## Compatibility Best Practices

### Version Alignment

- Match SDK version to Metabase version
- Use stable version tags (`53-stable`)
- Test with target Metabase version

### Migration Strategy

- Provide migrations for breaking changes
- Maintain backward compatibility when possible
- Document migration paths

### Dependency Management

- Declare all dependencies in manifest
- Use version ranges when supported
- Test with dependency versions

### Testing

- Test with multiple Metabase versions
- Test parent driver changes
- Test migration paths

## Summary

### Extension Points

1. **Drivers** - Plugin-based, multimethod dispatch, hierarchical inheritance
2. **Query Processor** - Middleware system, dynamic updates, wrapping support
3. **Visualizations** - Multimethod dispatch, JavaScript support, bundle loading

### Stability Mechanisms

1. **Hierarchy** - Parent/child inheritance provides stable interfaces
2. **Multimethods** - Dispatch system enables extension without core modification
3. **Migrations** - Version migration system maintains compatibility
4. **Lazy Loading** - Extensions load only when needed
5. **Version Tracking** - Extensions track versions for compatibility

### Compatibility Features

1. **Driver Supersession** - Deprecated drivers can be replaced
2. **Feature Flags** - Drivers declare feature support
3. **Migration System** - Data format migrations maintain compatibility
4. **SDK Versioning** - Embedding SDK matches Metabase versions
5. **Dependency Resolution** - Plugins declare and resolve dependencies

This architecture ensures that Metabase extensions can evolve independently while maintaining stability and compatibility with the core system.


