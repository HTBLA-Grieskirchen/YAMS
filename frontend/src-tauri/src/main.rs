use std::sync::Arc;

use error_stack::Report;
use tauri::Manager;
use yams_api::YamsAppApi;
use yams_core::{App, ports::RepositoryError};
use yams_filesystemstore::FileSystemObjectStore;
use yams_scheduler::{SQLiteStateStore, YamsSchedulerHandle, start};
use yams_sqlite::{SQLiteInstance, SharedSQLiteInstance};
use yams_typstreports::TypstPdfRenderer;

mod commands;
mod config;
mod tracing_setup;

use crate::config::{DeploymentMode, FrontendConfigDto};

#[allow(dead_code)]
struct SchedulerState(YamsSchedulerHandle);

#[tauri::command]
fn frontend_config(config: tauri::State<'_, FrontendConfigDto>) -> FrontendConfigDto {
    (*config).clone()
}

fn main() {
    tracing_setup::init_tracing(&config::resolve_log_dir());

    let config = config::load().expect("failed to load Tauri config");

    tauri::Builder::default()
        .setup(move |tauri_app| {
            match &config.deployment {
                DeploymentMode::Embedded {
                    database_url,
                    object_store_dir,
                } => {
                    let scheduler_config = config.scheduler.clone();
                    let (api, scheduler_handle) = tauri::async_runtime::block_on(async {
                        let sqlite = Arc::new(SQLiteInstance::local(database_url).await?);
                        sqlite.migrate_repos_to_latest().await?;

                        let object_store = FileSystemObjectStore::new(object_store_dir)
                            .expect("failed to initialize object store");
                        let object_store = Arc::new(object_store);
                        let pdf_renderer = Arc::new(TypstPdfRenderer::new());

                        let build_app = |sqlite: Arc<SQLiteInstance>| {
                            App::builder()
                                .uow_provider(Box::new(SharedSQLiteInstance::new(sqlite)))
                                .object_store(object_store.clone())
                                .pdf_renderer(pdf_renderer.clone())
                                .build()
                        };

                        let app = build_app(sqlite.clone());
                        let scheduler_handle = if scheduler_config.enabled {
                            let store = SQLiteStateStore::new(sqlite.clone());
                            Some(
                                start(
                                    Arc::new(build_app(sqlite.clone())),
                                    store,
                                    &scheduler_config,
                                )
                                .await
                                .expect("failed to start scheduler"),
                            )
                        } else {
                            None
                        };

                        Ok::<_, Report<RepositoryError>>((
                            YamsAppApi::new(app),
                            scheduler_handle,
                        ))
                    })
                    .expect("failed to initialize LibSQL adapter");

                    tauri_app.manage(api.inner_app());
                    tauri_app.manage(api);
                    if let Some(handle) = scheduler_handle {
                        tauri_app.manage(SchedulerState(handle));
                    }
                }
                DeploymentMode::Remote { remote_api_url } => {
                    tracing::info!(%remote_api_url, "Tauri running in remote mode; skipping embedded backend");
                }
            }

            tauri_app.manage(config.frontend_dto());
            tauri_app.manage(config);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            frontend_config,
            commands::klient_erstellen,
            commands::haustier_erstellen,
            commands::alle_haustiere,
            commands::alle_klienten,
            commands::alle_produkte,
            commands::alle_behandlungen,
            commands::alle_leistungen,
            commands::alle_rechnungen,
            commands::alle_seminare,
            commands::alle_seminar_termine,
            commands::haustier_by_id,
            commands::produkt_erstellen,
            commands::behandlung_erstellen,
            commands::leistung_aus_produkt_buchen,
            commands::leistung_aus_behandlung_buchen,
            commands::leistung_manuell_erfassen,
            commands::tagesabschluss_durchführen,
            commands::rechnungen_für_klient,
            commands::rechnung_pdf,
            commands::rechnung_als_bezahlt_markieren,
            commands::teilnahmebestaetigung_pdf,
            commands::seminar_erstellen,
            commands::seminar_by_id,
            commands::seminar_termin_planen,
            commands::seminar_termin_by_id,
            commands::seminar_termin_aktualisieren,
            commands::seminar_buchung_anlegen,
            commands::seminar_buchung_stornieren,
            commands::seminar_termin_absagen,
            commands::seminar_termin_abgehalten,
            commands::seminar_umsatz_vorschau,
            commands::seminar_umsatz_prognose,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
