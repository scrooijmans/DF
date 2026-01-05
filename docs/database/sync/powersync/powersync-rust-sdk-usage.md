Title: PowerSync – Product Updates | Improved Sync Performance in Our Client

Description: The release of v0.4.0 of the PowerSync SQLite Extension ( powersync-sqlite-core ) in our client SDKs introduces a new sync client implemented natively in the...

Keywords: launchnotes, release notes, product updates, changelog

Back to Announcements

**DATE:**

June 20, 2025

**AUTHOR:**

PowerSync Product Team

Flutter SDK React Native SDK Kotlin Multiplatform SDK JavaScript/Web SDK Swift SDK Node.js SDK

Improved Sync Performance in Our Client SDKs
============================================

**DATE:** June 20, 2025

**AUTHOR:** PowerSync Product Team

The release of v0.4.0 of the PowerSync SQLite Extension (`powersync-sqlite-core`) in our client SDKs introduces a new sync client implemented natively in the extension, which significantly improves sync performance - especially with large datasets.

Users will also experience smoother UI interactions during large sync operations, particularly noticeable on React Native. This new client also lays the foundation for future capabilities, including BLOB column support.

### New Rust-based Sync Client (Experimental)

The new sync client removes bottlenecks from JSON/BSON decoding in the client SDKs by shifting expensive parts of the sync logic into the SQLite extension. SDKs still handle network connections and stream raw sync data to the extension, but the extension now manages the sync state and passes instructions back to the SDKs. This offloads most of the processing from the SDK, resulting in faster, more consistent sync performance. Initial benchmarks:

*   **React Native:** ~35% faster sync, with significantly better UI responsiveness

*   **Kotlin (1M rows):** ~25% faster sync

#### Additional Optimizations

*   **Reduced Temporary Storage:** Sync speeds on large datasets are improved by reducing unnecessary temporary storage in `sync_local`. See the PR for more details.

### Availability

These updates are available in:

*   React Native: `1.22.0`

*   Web: `1.23.0`

*   Node.js: `0.6.0`

*   Flutter/Dart: `1.14.0`

*   Kotlin: `1.2.0`

*   Swift: `1.2.0`

*   _Not yet released for .NET_

### Enabling the New Sync Client

This implementation will eventually become the default, but we encourage users to try it out by opting in:

#### JavaScript (Web / React Native / Node.js)

**Important:** Once you enable the new sync client in our JavaScript-based SDKs, your local database is migrated. You cannot downgrade your SDK to a version _earlier_ than those listed above, because older builds do not understand the new format.

In all our JavaScript SDKs, the new implementation can be enabled with the `clientImplementation` option when connecting:

```

await db.connect(new MyConnector(), {
clientImplementation: SyncClientImplementation.RUST
});
```

You can migrate back to the JavaScript client later by removing the option.

#### Flutter/Dart

Opt in to the experimental APIs by passing the following option when connecting:

```
database.connect(
connector: YourConnector(),
options: const SyncOptions(
syncImplementation: SyncClientImplementation.rust,
),
);
```

You can migrate back to the Dart client later by removing the option.

#### Kotlin

Opt in to the experimental APIs by passing the following option when connecting:

```kotlin
//@file:OptIn(ExperimentalPowerSyncAPI::class)
database.connect(MyConnector(), options = SyncOptions(
newClientImplementation = true,
))
```

You can migrate back to the Kotlin client later by removing the option.

#### Swift

Opt in to the experimental APIs by passing the following option when connecting:

```swift
@_spi(PowerSyncExperimental) import PowerSync

try await db.connect(connector: connector, options: ConnectOptions(
newClientImplementation: true,
))
```

You can migrate back to the Swift client later by removing the option.

Powered by LaunchNotes

##### Subscribe to updates

×

Email      

Subscribe By clicking subscribe, you accept our privacy policy and terms and conditions. reCAPTCHA privacy and terms apply