use tauri::State;
use uuid::Uuid;
use yams_api::{
    YamsAppApi,
    requests::{
        BehandlungErstellung, HaustierErstellung, KlientErstellung,
        LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung,
        ProduktErstellung, TagesabschlussErstellung,
    },
    schema::{Behandlung, Haustier, Klient, Leistung, Produkt, Rechnung},
};
use yams_core::ResultReport;

fn map_report<T, E: std::fmt::Display>(result: ResultReport<T, E>) -> Result<T, String> {
    result.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn klient_erstellen(
    erstellung: KlientErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<Klient, String> {
    map_report(ctx.klient_erstellen(erstellung).await)
}

#[tauri::command]
pub async fn haustier_erstellen(
    erstellung: HaustierErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<Haustier, String> {
    map_report(ctx.haustier_erstellen(erstellung).await)
}

#[tauri::command]
pub async fn alle_haustiere(ctx: State<'_, YamsAppApi>) -> Result<Vec<Haustier>, String> {
    map_report(ctx.alle_haustiere().await)
}

#[tauri::command]
pub async fn haustier_by_id(id: Uuid, ctx: State<'_, YamsAppApi>) -> Result<Haustier, String> {
    map_report(ctx.haustier_by_id(id).await)
}

#[tauri::command]
pub async fn produkt_erstellen(
    erstellung: ProduktErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<Produkt, String> {
    map_report(ctx.produkt_erstellen(erstellung).await)
}

#[tauri::command]
pub async fn behandlung_erstellen(
    erstellung: BehandlungErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<Behandlung, String> {
    map_report(ctx.behandlung_erstellen(erstellung).await)
}

#[tauri::command]
pub async fn leistung_aus_produkt_buchen(
    erstellung: LeistungAusProduktErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<Leistung, String> {
    map_report(ctx.leistung_aus_produkt_buchen(erstellung).await)
}

#[tauri::command]
pub async fn leistung_aus_behandlung_buchen(
    erstellung: LeistungAusBehandlungErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<Leistung, String> {
    map_report(ctx.leistung_aus_behandlung_buchen(erstellung).await)
}

#[tauri::command]
pub async fn leistung_manuell_erfassen(
    erstellung: LeistungManuelleErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<Leistung, String> {
    map_report(ctx.leistung_manuell_erfassen(erstellung).await)
}

#[tauri::command]
pub async fn tagesabschluss_durchführen(
    erstellung: TagesabschlussErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<Vec<Rechnung>, String> {
    map_report(ctx.tagesabschluss_durchführen(erstellung).await)
}

#[tauri::command]
pub async fn rechnungen_für_klient(
    klient_id: Uuid,
    ctx: State<'_, YamsAppApi>,
) -> Result<Vec<Rechnung>, String> {
    map_report(ctx.rechnungen_für_klient(klient_id).await)
}
