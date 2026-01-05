# Metabase Mutation Handling: Versioning, Saving, and Propagation

## Overview

Metabase implements a comprehensive revision system for tracking changes to questions (cards) and dashboards. This document explains how edits are versioned, saved, and propagated to dependent visualizations, ensuring data consistency and providing a complete audit trail.

## Revision System Architecture

### Core Components

The revision system consists of three main components:

1. **Revision Table** (`revision`) - Stores historical snapshots of entities
2. **Event System** - Publishes events when entities are created or updated
3. **Revision Handlers** - Process events and create revision records

### Revision Table Schema

The `revision` table stores complete snapshots of entity state:

```clojure
;; Key fields in the revision table:
:model        ;; Entity type (e.g., "Card", "Dashboard")
:model_id     ;; ID of the entity being versioned
:user_id      ;; User who made the change
:object       ;; Complete serialized state of the entity (JSON)
:timestamp    ;; When the revision was created
:is_creation  ;; Boolean: true if this is the initial creation
:is_reversion ;; Boolean: true if this revision was created by reverting
:message      ;; Optional message describing the change
:most_recent  ;; Boolean: true for the latest revision
:metabase_version ;; Metabase version when revision was created
```

### Immutability

Revisions are **immutable** - once created, they cannot be updated:

```clojure
(t2/define-before-update :model/Revision
  [_revision]
  (fn [& _] (throw (Exception. (tru "You cannot update a Revision!")))))
```

This ensures a complete, unalterable audit trail.

## How Edits Are Versioned

### Event-Driven Revision Creation

Metabase uses an event-driven architecture where entity updates trigger revision creation:

#### 1. Entity Update Triggers Event

When a card or dashboard is updated, an event is published:

```clojure
;; Card update event
(events/publish-event! :event/card-update 
  {:object card 
   :user-id api/*current-user-id*})

;; Dashboard update event  
(events/publish-event! :event/dashboard-update 
  {:object dashboard 
   :user-id api/*current-user-id*})
```

#### 2. Event Handler Creates Revision

Event handlers listen for these events and create revisions:

```clojure
;; Card event handler
(methodical/defmethod events/publish-event! ::card-event
  [topic event]
  (push-revision! :model/Card event 
    {:is-creation? (= topic :event/card-create)}))

;; Dashboard event handler
(methodical/defmethod events/publish-event! ::dashboard-event
  [topic event]
  (push-revision! :model/Dashboard event 
    {:is-creation? (= topic :event/dashboard-create)}))
```

#### 3. Revision Creation Logic

The `push-revision!` function creates a revision only if the entity has actually changed:

```clojure
(mu/defn push-revision!
  [{:keys [id entity user-id object is-creation? message]}]
  (let [entity-name (name entity)
        serialized-object (serialize-instance entity id 
                          (dissoc object :message))
        last-object (t2/select-one-fn :object :model/Revision 
                      :model entity-name :model_id id 
                      {:order-by [[:id :desc]]})]
    ;; Only create revision if object has changed
    (when-not (= (json/encode serialized-object)
                 (json/encode last-object))
      (t2/insert! :model/Revision
        :model        entity-name
        :model_id     id
        :user_id      user-id
        :object       serialized-object
        :is_creation  is-creation?
        :is_reversion false
        :message      message)
      object)))
```

**Key Points:**
- Revisions are only created when the serialized object differs from the last revision
- The entire entity state is serialized and stored
- A database lock prevents concurrent revision creation for the same entity

### Revision Comparison

Before creating a revision, Metabase:

1. **Serializes the current state** - Converts the entity to a normalized JSON representation
2. **Retrieves the last revision** - Gets the most recent revision for comparison
3. **Compares serialized objects** - Uses JSON encoding for comparison (handles nested objects correctly)
4. **Creates revision only if changed** - Prevents duplicate revisions for identical states

### Revision Lifecycle Hooks

The revision model includes lifecycle hooks:

```clojure
(t2/define-before-insert :model/Revision
  [{:keys [model model_id] :as revision}]
  ;; Obtain a lock on existing revisions to prevent concurrent inserts
  (t2/query {:select [:id]
             :from [:revision]
             :where [:and
                     [:= :model model]
                     [:= :model_id model_id]]
             :for :update})
  (assoc revision
         :timestamp (or (:timestamp revision) :%now)
         :metabase_version config/mb-version-string
         :most_recent true))

(t2/define-after-select :model/Revision
  ;; Normalize the object when reading from DB
  ;; Important for Cards where dataset_query needs normalization
  [{:keys [model] :as revision}]
  (let [model (u/ignore-exceptions 
                (t2.model/resolve-model (symbol model)))]
    (cond-> revision
      model (update :object (partial mi/do-after-select model)))))
```

## How Edits Are Saved

### Card Update Process

The card update process involves multiple steps within a transaction:

```clojure
(defn update-card!
  [{:keys [card-before-update card-updates actor delete-old-dashcards?]}]
  (t2/with-transaction [_conn]
    ;; 1. Handle collection position changes
    (api/maybe-reconcile-collection-position! 
      card-before-update card-updates)
    
    ;; 2. Handle dashcard auto-placement/removal
    (autoplace-or-remove-dashcards-for-card! 
      card-before-update card-updates delete-old-dashcards?)
    
    ;; 3. Validate dashboard internal card updates
    (assert-is-valid-dashboard-internal-update 
      card-updates card-before-update)
    
    ;; 4. Handle moderation reviews (if card was verified)
    (when (and (card-is-verified? card-before-update)
               (changed? card-compare-keys 
                        card-before-update card-updates))
      (moderation-review/create-review! 
        {:moderated_item_id   (:id card-before-update)
         :moderated_item_type "card"
         :moderator_id        (:id actor)
         :status              nil
         :text                (tru "Unverified due to edit")}))
    
    ;; 5. Update the Card in the database
    (t2/update! :model/Card (:id card-before-update)
      (u/select-keys-when card-updates
        :present #{:collection_id :collection_position 
                   :description :cache_ttl :archived_directly 
                   :dashboard_id}
        :non-nil #{:dataset_query :display :name 
                   :visualization_settings :archived
                   :enable_embedding :type :parameters 
                   :parameter_mappings :embedding_params
                   :result_metadata :collection_preview 
                   :verified-result-metadata?}))
    
    ;; 6. Update dependent dashcard parameters
    (try
      (update-associated-parameters! 
        card-before-update card-updates)
      (catch Throwable e
        (log/error "Update of dependent card parameters failed!")
        (log/debug e ...))))
  
  ;; 7. Fetch updated card and publish event
  (let [card (t2/select-one :model/Card 
                            :id (:id card-before-update))]
    (pulse/delete-alerts-if-needed! 
      :old-card card-before-update 
      :new-card card 
      :actor actor)
    
    ;; Skip event if only collection position changed
    (when-not (= #{:collection_position} 
                 (set (keys card-updates)))
      (events/publish-event! :event/card-update 
        {:object card 
         :user-id api/*current-user-id*}))
    card))
```

### Dashboard Update Process

Dashboard updates follow a similar pattern but also handle dashcards and tabs:

```clojure
;; Dashboard update includes:
;; 1. Update dashboard properties
;; 2. Update/create/delete tabs
;; 3. Update/create/delete dashcards
;; 4. Handle tab ID remapping
;; 5. Archive/unarchive internal dashboard questions
;; 6. Publish event (which triggers revision creation)
```

### Transaction Safety

All updates occur within database transactions:

```clojure
(t2/with-transaction [_conn]
  ;; All updates happen atomically
  ;; If any step fails, all changes are rolled back
)
```

This ensures:
- **Atomicity** - All changes succeed or fail together
- **Consistency** - Database remains in a valid state
- **Isolation** - Concurrent updates don't interfere

## How Changes Propagate to Dependent Visualizations

### Dependency Graph

When a card is updated, Metabase identifies and updates dependent entities:

1. **Dashboard Cards** - Cards displayed on dashboards
2. **Parameter Mappings** - Dashboard parameters linked to card fields
3. **Series Cards** - Cards used as additional series in visualizations
4. **Referenced Cards** - Cards referenced in SQL queries (CTEs)

### Automatic Parameter Mapping Updates

When a card's query structure changes (e.g., breakout fields), Metabase automatically updates dependent dashboard parameter mappings:

```clojure
(defn- update-associated-parameters!
  [card-before card-after]
  "Updates parameter mappings of dashcards when their targeted 
   card's breakout elements are modified."
  (let [card->breakout
        #(-> % :dataset_query mbql.normalize/normalize 
             :query :breakout)
        breakout-before (card->breakout card-before)
        breakout-after  (card->breakout card-after)]
    (when-some [identifier->action 
                (breakouts-->identifier->action 
                  breakout-before breakout-after)]
      (let [dashcards (t2/select :model/DashboardCard 
                        :card_id (some :id [card-after card-before]))
            updates   (updates-for-dashcards 
                        identifier->action dashcards)]
        (when (seq updates)
          (t2/with-transaction [_conn]
            (doseq [[id update] updates]
              (t2/update! :model/DashboardCard :id id update))))))))
```

**How It Works:**

1. **Extract Breakouts** - Gets breakout fields from before/after states
2. **Calculate Changes** - Determines which field references changed
3. **Find Dependent Dashcards** - Queries all dashcards using this card
4. **Update Mappings** - Updates parameter mappings to point to new field references

### Parameter Mapping Update Logic

```clojure
(defn- update-mapping
  [identifier->action {[_dim ref] :target :as mapping}]
  "Modifies a parameter mapping based on a provided action."
  (let [identifier (subvec ref 0 2)
        [action arg] (get identifier->action identifier)]
    (case action
      :update (assoc-in mapping [:target 1] arg)
      mapping)))

(defn- updates-for-dashcards
  [identifier->action dashcards]
  "Generates updates for dashcards whose parameter mappings need updating."
  (not-empty 
    (for [{:keys [id parameter_mappings]} dashcards
          :let [updated (into [] 
                        (map #(if (eligible-mapping? %)
                                (update-mapping identifier->action %)
                                %))
                        parameter_mappings)]
          :when (not= parameter_mappings updated)]
      [id {:parameter_mappings updated}])))
```

### Dashcard Auto-Placement/Removal

When a card's collection or dashboard association changes, Metabase automatically manages dashcard placement:

```clojure
(autoplace-or-remove-dashcards-for-card! 
  card-before-update card-updates delete-old-dashcards?)
```

This function:
- **Removes dashcards** when a card is moved out of a dashboard
- **Archives dashcards** when appropriate
- **Maintains referential integrity** between cards and dashboards

### Real-Time Propagation

Changes propagate **immediately** when a card is updated:

1. **Card Update** - Transaction completes
2. **Event Published** - `:event/card-update` event is published
3. **Revision Created** - Event handler creates revision
4. **Dependent Updates** - Parameter mappings updated in same transaction
5. **Cache Invalidation** - Query result caches invalidated
6. **UI Refresh** - Dashboards using the card reflect changes on next load

### Propagation Scope

The propagation system handles:

#### 1. Direct Dependencies
- **Dashboard Cards** - Cards directly added to dashboards
- **Parameter Mappings** - Dashboard filters linked to card fields
- **Series Cards** - Additional series in multi-series visualizations

#### 2. Indirect Dependencies
- **Referenced Cards** - Cards referenced in SQL queries via `{{#card-id}}` syntax
- **Model Dependencies** - Cards that use models as data sources
- **Nested Queries** - Cards that reference other cards in their queries

#### 3. Metadata Dependencies
- **Result Metadata** - Column structure changes affect visualizations
- **Field References** - Field ID changes require mapping updates
- **Semantic Types** - Type changes affect how fields are displayed

## Revision Retrieval and Reversion

### Fetching Revisions

Revisions can be retrieved with full details:

```clojure
(mu/defn revisions+details
  [model id]
  "Fetches all revisions for a model/id and enriches with details."
  (when-let [revisions (revisions model id)]
    (loop [acc [], [r1 r2 & more] revisions]
      (if-not r2
        (conj acc (add-revision-details model r1 nil))
        (recur (conj acc (add-revision-details model r1 r2))
               (conj more r2))))))
```

The `add-revision-details` function enriches revisions with:
- **Diff** - Changes from previous revision
- **Description** - Human-readable change description
- **User Details** - Who made the change
- **Metadata** - Timestamp, version, etc.

### Reverting to a Previous Revision

Metabase supports reverting entities to previous revisions:

```clojure
(mu/defn revert!
  [{:keys [id user-id revision-id entity]}]
  "Reverts an entity to a specific revision."
  (let [model-name (name entity)
        serialized-instance (t2/select-one-fn :object :model/Revision 
                            :model model-name :model_id id 
                            :id revision-id)]
    (t2/with-transaction [_conn]
      ;; 1. Revert the object to the serialized state
      (revert-to-revision! entity id user-id serialized-instance)
      
      ;; 2. Create a new revision recording the reversion
      (let [last-revision (t2/select-one :model/Revision 
                        :model model-name :model_id id 
                        {:order-by [[:id :desc]]})
            new-revision  (first (t2/insert-returning-instances! 
                        :model/Revision
                        :model        model-name
                        :model_id     id
                        :user_id      user-id
                        :object       serialized-instance
                        :is_creation  false
                        :is_reversion true))]
        (add-revision-details entity new-revision last-revision)))))
```

**Reversion Process:**

1. **Retrieve Serialized State** - Gets the complete entity state from the revision
2. **Apply Reversion** - Model-specific reversion logic restores the state
3. **Create Revision** - Records the reversion as a new revision (with `is_reversion: true`)
4. **Propagate Changes** - Dependent entities are updated (same as regular updates)

### Dashboard Reversion

Dashboard reversion is more complex due to nested entities:

```clojure
(defmethod revision/revert-to-revision! :model/Dashboard
  [model dashboard-id user-id serialized-dashboard]
  ;; 1. Update dashboard core properties
  ((get-method revision/revert-to-revision! :default) 
    model dashboard-id user-id 
    (dissoc serialized-dashboard :cards :tabs))
  
  ;; 2. Handle tabs
  (let [current-tabs (t2/select-fn-vec ...)
        {:keys [old->new-tab-id]} 
        (dashboard-tab/do-update-tabs! dashboard-id 
          current-tabs (:tabs serialized-dashboard))
        
        ;; 3. Archive/unarchive internal dashboard questions
        _ (dashboard/archive-or-unarchive-internal-dashboard-questions! 
            dashboard-id serialized-dashcards)
        
        ;; 4. Remap tab IDs in dashcards
        serialized-dashcards (cond->> serialized-dashcards
                            (seq old->new-tab-id)
                            (map (fn [card]
                                   (if-let [new-tab-id 
                                            (get old->new-tab-id 
                                                 (:dashboard_tab_id card))]
                                     (assoc card :dashboard_tab_id new-tab-id)
                                     card))))]
    ;; 5. Revert dashcards
    (revert-dashcards dashboard-id serialized-dashcards))
  serialized-dashboard)
```

## Change Detection and Diff Calculation

### Revision Changes

Metabase calculates what changed between revisions:

```clojure
(defn- revision-changes
  [model prev-revision revision]
  (cond
    (:is_creation revision)  
    [(deferred-tru "created this")]
    
    (:is_reversion revision) 
    [(deferred-tru "reverted to an earlier version")]
    
    (nil? prev-revision)
    [(deferred-tru "modified this")]
    
    :else
    (diff-strings model 
      (:object prev-revision) 
      (:object revision))))
```

### Diff Calculation

The diff system:
- **Compares serialized objects** - Uses normalized JSON representations
- **Identifies field changes** - Tracks which fields were added, removed, or modified
- **Handles nested structures** - Correctly diffs complex nested objects like `dataset_query`
- **Provides human-readable descriptions** - Converts technical changes to readable text

## Event System Architecture

### Event Types

Metabase defines event hierarchies:

```clojure
;; Base event type
(derive ::event ::event)

;; Card events
(derive ::card-event ::event)
(derive :event/card-create ::card-event)
(derive :event/card-update ::card-event)

;; Dashboard events
(derive ::dashboard-event ::event)
(derive :event/dashboard-create ::dashboard-event)
(derive :event/dashboard-update ::dashboard-event)
```

### Event Publishing

Events are published after successful updates:

```clojure
;; Only publish if meaningful changes occurred
(when-not (= #{:collection_position} (set (keys card-updates)))
  (events/publish-event! :event/card-update 
    {:object card 
     :user-id api/*current-user-id*}))
```

**Skip Conditions:**
- Collection position-only changes (not considered meaningful)
- Failed transactions (no event published)
- Internal updates (some updates don't trigger events)

### Event Processing

Event handlers process events asynchronously:

```clojure
(defn- push-revision!
  [model {:keys [user-id] object :object :as event} 
   {:keys [is-creation?]}]
  (when event
    (try
      (when-not (t2/instance-of? model object)
        (throw (ex-info "object must be a model instance" ...)))
      (let [user-id (or user-id api/*current-user-id*)]
        (revision/push-revision! 
          {:entity       model
           :id           (:id object)
           :object       object
           :user-id      user-id
           :is-creation? is-creation?
           :message      (:revision-message event)}))
      (catch Throwable e
        (log/warnf e "Failed to process revision event for model %s" 
                   model)))))
```

## Performance Considerations

### Revision Storage

- **Complete Snapshots** - Each revision stores the full entity state
- **JSON Serialization** - Efficient storage format
- **Selective Comparison** - Only creates revisions when state actually changes
- **Database Locking** - Prevents concurrent revision creation issues

### Propagation Performance

- **Transaction Batching** - Multiple dependent updates in single transaction
- **Selective Updates** - Only updates dashcards that actually need changes
- **Efficient Queries** - Uses indexed lookups for finding dependent entities
- **Async Processing** - Some propagation happens asynchronously

### Revision Limits

Metabase maintains a maximum number of revisions per entity:

```clojure
;; Only keep max-revisions number of revisions per entity
;; Older revisions may be pruned
```

This prevents unbounded storage growth while maintaining useful history.

## Summary

### Key Principles

1. **Immutability** - Revisions are never modified once created
2. **Completeness** - Each revision stores the full entity state
3. **Change Detection** - Revisions only created when state actually changes
4. **Automatic Propagation** - Dependent entities updated automatically
5. **Transaction Safety** - All updates occur atomically
6. **Event-Driven** - Revision creation triggered by events
7. **Audit Trail** - Complete history of all changes

### Update Flow

1. **User Makes Change** → Card/Dashboard updated via API
2. **Transaction Executes** → All changes applied atomically
3. **Dependent Updates** → Parameter mappings, dashcards updated
4. **Event Published** → `:event/card-update` or `:event/dashboard-update`
5. **Revision Created** → Event handler creates revision snapshot
6. **Propagation Complete** → All dependent visualizations reflect changes

### Benefits

- **Data Consistency** - All dependent entities stay in sync
- **Audit Trail** - Complete history of all changes
- **Reversion Support** - Can restore any previous state
- **Change Tracking** - See exactly what changed and when
- **User Attribution** - Know who made each change
- **Version Control** - Track Metabase version for each revision

This architecture ensures that Metabase maintains data integrity while providing powerful versioning and change tracking capabilities.


