use error_stack::Report;
use std::path::Path;
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

use crate::config::{YAMSFileConfig, YAMSFrontendConfig};

#[tauri::command]
fn frontend_config(config: tauri::State<'_, YAMSFrontendConfig>) -> YAMSFrontendConfig {
    (*config).clone()
}

fn main() {
    tracing_setup::init_tracing(&config::log_dir());

    let (backend_config, _frontend_config) = YAMSFileConfig::load();

    tauri::Builder::default()
        .setup(move |tauri_app| {
            // Initialize LibSQL Adapter using config
            let db_url = &backend_config.local_database_location;
            let db_instance = tauri::async_runtime::block_on(async {
                let mut sqlite = SQLiteInstance::local(db_url).await?;
                sqlite.migrate_to_latest().await?;
                Ok::<_, Report<RepositoryError>>(sqlite)
            })
            .expect("failed to initialize LibSQL adapter");

            let pdf_dir = Path::new(db_url)
                .parent()
                .unwrap_or(Path::new("."))
                .join("pdfs");
            let object_store =
                FileSystemObjectStore::new(pdf_dir).expect("failed to initialize pdf object store");
            let app = App::builder()
                .uow_provider(Box::new(db_instance))
                .object_store(Arc::new(object_store))
                .pdf_renderer(Arc::new(TypstPdfRenderer::new()))
                .build();
            let api = YamsAppApi::new(app);

            tauri_app.manage(api.inner_app());
            tauri_app.manage(api);

            tauri_app.manage(backend_config);
            tauri_app.manage(_frontend_config);
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
