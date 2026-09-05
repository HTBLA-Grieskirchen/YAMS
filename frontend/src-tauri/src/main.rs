use error_stack::Report;
use std::sync::Arc;
use tauri::Manager;
use yams_api::YamsAppApi;
use yams_core::{App, ports::RepositoryError};
use yams_filesystemstore::FileSystemObjectStore;
use yams_persistence::SQLiteInstance;
use yams_typstreports::TypstPdfRenderer;

mod commands;
mod config;
mod tracing_setup;

use crate::config::{DeploymentMode, FrontendConfigDto};

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
                    let db_instance = tauri::async_runtime::block_on(async {
                        let mut sqlite = SQLiteInstance::local(database_url).await?;
                        sqlite.migrate_to_latest().await?;
                        Ok::<_, Report<RepositoryError>>(sqlite)
                    })
                    .expect("failed to initialize LibSQL adapter");

                    let object_store = FileSystemObjectStore::new(object_store_dir)
                        .expect("failed to initialize object store");
                    let app = App::builder()
                        .uow_provider(Box::new(db_instance))
                        .object_store(Arc::new(object_store))
                        .pdf_renderer(Arc::new(TypstPdfRenderer::new()))
                        .build();
                    let api = YamsAppApi::new(app);

                    tauri_app.manage(api.inner_app());
                    tauri_app.manage(api);
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
            commands::teilnahmebestätigung_pdf,
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
