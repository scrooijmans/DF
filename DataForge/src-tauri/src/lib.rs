mod checkshot_commands;
mod commands;
mod ingest_commands;
mod marker_commands;
mod state;
mod surface_commands;
mod sync_commands;
mod trajectory_commands;

use state::AppState;
use std::sync::Mutex;
use tauri::{Emitter, Manager};
use tracing::info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(AppState::default()))
        .invoke_handler(tauri::generate_handler![
            // Utilities
            commands::greet,
            commands::parse_las_file,
            // Authentication (ColaNode pattern)
            commands::is_ready,
            commands::register,
            commands::login,
            commands::logout,
            commands::get_session,
            // Workspaces
            commands::create_workspace,
            commands::select_workspace,
            commands::get_workspaces,
            // Workspace members
            commands::get_workspace_members,
            commands::add_workspace_member,
            commands::update_workspace_member_role,
            commands::remove_workspace_member,
            commands::get_my_workspace_role,
            commands::delete_workspace,
            commands::update_workspace,
            commands::transfer_workspace_ownership,
            // Account
            commands::update_account,
            // Sync (ColaNode-style Git-like sync)
            sync_commands::get_sync_state,
            sync_commands::init_sync,
            sync_commands::set_sync_server,
            sync_commands::set_sync_auth_token,
            sync_commands::has_sync_auth_token,
            sync_commands::sync_workspace,
            sync_commands::test_sync_connection,
            sync_commands::get_conflicts,
            sync_commands::resolve_conflict,
            sync_commands::get_pending_sync_count,
            sync_commands::clear_synced_queue,
            // Data Ingestion
            ingest_commands::parse_las_for_ingest,
            ingest_commands::parse_las_files_for_ingest,
            ingest_commands::ingest_las_files,
            ingest_commands::get_supported_ingest_types,
            ingest_commands::get_workspace_wells,
            // Inspector (DBeaver-style SQLite browser)
            ingest_commands::inspector_get_tables,
            ingest_commands::inspector_get_columns,
            ingest_commands::inspector_query_table,
            ingest_commands::inspector_update_cell,
            ingest_commands::inspector_delete_row,
            ingest_commands::inspector_insert_row,
            ingest_commands::inspector_get_fk_lookup_values,
            ingest_commands::inspector_cleanup_orphaned_blobs,
            ingest_commands::inspector_preview_orphaned_blobs,
            // Home page - Recent activity
            ingest_commands::get_recent_activity,
            ingest_commands::get_workspace_stats,
            // Well Detail Page
            ingest_commands::get_well_details,
            ingest_commands::get_well_activity,
            // Curve Viewer (DuckDB-powered Parquet queries)
            ingest_commands::get_well_curves,
            ingest_commands::query_curve_data,
            ingest_commands::get_curve_coverage,
            // Curve Editing (Blob data modification)
            ingest_commands::curve_update_cells,
            ingest_commands::curve_add_rows,
            ingest_commands::curve_delete_rows,
            ingest_commands::curve_get_versions,
            ingest_commands::curve_revert_to_version,
            // Trajectory Ingestion (CSV)
            trajectory_commands::parse_trajectory_for_ingest,
            trajectory_commands::ingest_trajectory_files,
            trajectory_commands::get_well_trajectories,
            trajectory_commands::get_trajectory_stations,
            // Marker/Well Tops Ingestion (CSV)
            marker_commands::parse_markers_for_ingest,
            marker_commands::ingest_marker_files,
            marker_commands::get_well_marker_sets,
            marker_commands::get_markers,
            marker_commands::get_well_markers,
            // Surface Ingestion (CSV - workspace-level 3D surfaces)
            surface_commands::parse_surface_for_ingest,
            surface_commands::ingest_surface_files,
            surface_commands::get_workspace_surfaces,
            surface_commands::get_surface,
            surface_commands::delete_surface,
            // Checkshot Ingestion (CSV - well-level time-depth data)
            checkshot_commands::parse_checkshot_for_ingest,
            checkshot_commands::ingest_checkshot_files,
            checkshot_commands::get_well_checkshots,
            checkshot_commands::get_checkshot_data,
            checkshot_commands::delete_checkshot_run,
        ])
        .setup(|app| {
            // Get app data directory
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            info!("🚀 Initializing DataForge with data directory: {:?}", data_dir);

            // Get the main window (it's hidden initially from tauri.conf.json)
            let window = app.get_webview_window("main").unwrap();

            // Initialize state synchronously during setup (ColaNode pattern: backend-first)
            let state = app.state::<Mutex<AppState>>();
            let init_result = {
                let mut state = state.lock().expect("Failed to lock app state");
                state.initialize(data_dir)
            };

            match init_result {
                Ok(_) => {
                    info!("✅ App state initialized successfully");
                    
                    // Validate session during initialization (ColaNode pattern)
                    // This ensures we have a valid session state before frontend loads
                    let state = app.state::<Mutex<AppState>>();
                    let session_valid = {
                        let state = state.lock().expect("Failed to lock app state");
                        if let Some(db) = state.db.as_ref() {
                            if let Some(ref token) = state.session.token {
                                // Validate session token
                                match dataforge_core::auth::validate_session(db, token) {
                                    Ok(_) => {
                                        info!("✅ Valid session found, user authenticated");
                                        true
                                    }
                                    Err(e) => {
                                        info!("⚠️ Session validation failed: {}, clearing session", e);
                                        // Session invalid - will be cleared when frontend calls get_session
                                        false
                                    }
                                }
                            } else {
                                info!("ℹ️ No session token found");
                                false
                            }
                        } else {
                            false
                        }
                    };

                    if !session_valid {
                        // Clear invalid session
                        let mut state = state.lock().expect("Failed to lock app state");
                        let _ = state.clear_session();
                    }

                    // Emit backend-ready event for frontend to know initialization succeeded
                    info!("📡 Emitting backend-ready event (success)");
                    let _ = app.emit("backend-ready", serde_json::json!({
                        "success": true
                    }));
                }
                Err(e) => {
                    tracing::error!("❌ Failed to initialize app state: {}", e);
                    // Emit backend-ready event with error for frontend to handle
                    info!("📡 Emitting backend-ready event (error)");
                    let _ = app.emit("backend-ready", serde_json::json!({
                        "success": false,
                        "error": e.to_string()
                    }));
                }
            }

            // Open devtools in debug mode
            #[cfg(debug_assertions)]
            {
                window.open_devtools();
            }

            // Show window after initialization is complete (ColaNode pattern)
            // This provides a better UX by avoiding blank window flash
            info!("👁️ Showing main window");
            window.show().expect("Failed to show main window");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
