use tauri::State;
use uuid::Uuid;
use yams_api::{
    YamsAppApi,
    requests::{
        BehandlungErstellung, HaustierErstellung, KlientErstellung,
        LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung,
        ProduktErstellung, SeminarBuchungErstellung, SeminarErstellung, SeminarTerminAbsage,
        SeminarTerminAktualisierung, SeminarTerminErstellung, TagesabschlussErstellung,
    },
    schema::{
        Behandlung, Haustier, Klient, Leistung, Produkt, Rechnung, Seminar, SeminarTermin,
        SeminarUmsatzPrognose, SeminarUmsatzVorschau,
    },
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
pub async fn alle_klienten(ctx: State<'_, YamsAppApi>) -> Result<Vec<Klient>, String> {
    map_report(ctx.alle_klienten().await)
}

#[tauri::command]
pub async fn alle_produkte(ctx: State<'_, YamsAppApi>) -> Result<Vec<Produkt>, String> {
    map_report(ctx.alle_produkte().await)
}

#[tauri::command]
pub async fn alle_behandlungen(ctx: State<'_, YamsAppApi>) -> Result<Vec<Behandlung>, String> {
    map_report(ctx.alle_behandlungen().await)
}

#[tauri::command]
pub async fn alle_leistungen(ctx: State<'_, YamsAppApi>) -> Result<Vec<Leistung>, String> {
    map_report(ctx.alle_leistungen().await)
}

#[tauri::command]
pub async fn alle_rechnungen(ctx: State<'_, YamsAppApi>) -> Result<Vec<Rechnung>, String> {
    map_report(ctx.alle_rechnungen().await)
}

#[tauri::command]
pub async fn alle_seminare(ctx: State<'_, YamsAppApi>) -> Result<Vec<Seminar>, String> {
    map_report(ctx.alle_seminare().await)
}

#[tauri::command]
pub async fn alle_seminar_termine(
    ctx: State<'_, YamsAppApi>,
) -> Result<Vec<SeminarTermin>, String> {
    map_report(ctx.alle_seminar_termine().await)
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

#[tauri::command]
pub async fn rechnung_pdf(id: Uuid, ctx: State<'_, YamsAppApi>) -> Result<Vec<u8>, String> {
    let stream = map_report(ctx.rechnung_pdf(id).await)?;
    yams_core::ports::collect_object(stream)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn teilnahmebestätigung_pdf(
    termin_id: Uuid,
    buchung_id: Uuid,
    ctx: State<'_, YamsAppApi>,
) -> Result<Vec<u8>, String> {
    let stream = map_report(ctx.teilnahmebestätigung_pdf(termin_id, buchung_id).await)?;
    yams_core::ports::collect_object(stream)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn seminar_erstellen(
    erstellung: SeminarErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<Seminar, String> {
    map_report(ctx.seminar_erstellen(erstellung).await)
}

#[tauri::command]
pub async fn seminar_by_id(id: Uuid, ctx: State<'_, YamsAppApi>) -> Result<Seminar, String> {
    map_report(ctx.seminar_by_id(id).await)
}

#[tauri::command]
pub async fn seminar_termin_planen(
    erstellung: SeminarTerminErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<SeminarTermin, String> {
    map_report(ctx.seminar_termin_planen(erstellung).await)
}

#[tauri::command]
pub async fn seminar_termin_by_id(
    id: Uuid,
    ctx: State<'_, YamsAppApi>,
) -> Result<SeminarTermin, String> {
    map_report(ctx.seminar_termin_by_id(id).await)
}

#[tauri::command]
pub async fn seminar_termin_aktualisieren(
    id: Uuid,
    aktualisierung: SeminarTerminAktualisierung,
    ctx: State<'_, YamsAppApi>,
) -> Result<SeminarTermin, String> {
    map_report(ctx.seminar_termin_aktualisieren(id, aktualisierung).await)
}

#[tauri::command]
pub async fn seminar_buchung_anlegen(
    termin_id: Uuid,
    erstellung: SeminarBuchungErstellung,
    ctx: State<'_, YamsAppApi>,
) -> Result<SeminarTermin, String> {
    map_report(ctx.seminar_buchung_anlegen(termin_id, erstellung).await)
}

#[tauri::command]
pub async fn seminar_buchung_stornieren(
    termin_id: Uuid,
    buchung_id: Uuid,
    ctx: State<'_, YamsAppApi>,
) -> Result<SeminarTermin, String> {
    map_report(ctx.seminar_buchung_stornieren(termin_id, buchung_id).await)
}

#[tauri::command]
pub async fn seminar_termin_absagen(
    termin_id: Uuid,
    absage: SeminarTerminAbsage,
    ctx: State<'_, YamsAppApi>,
) -> Result<SeminarTermin, String> {
    map_report(ctx.seminar_termin_absagen(termin_id, absage).await)
}

#[tauri::command]
pub async fn seminar_termin_abgehalten(
    termin_id: Uuid,
    ctx: State<'_, YamsAppApi>,
) -> Result<SeminarTermin, String> {
    map_report(ctx.seminar_termin_abgehalten(termin_id).await)
}

#[tauri::command]
pub async fn seminar_umsatz_vorschau(
    termin_id: Uuid,
    ctx: State<'_, YamsAppApi>,
) -> Result<SeminarUmsatzVorschau, String> {
    map_report(ctx.seminar_umsatz_vorschau(termin_id).await)
}

#[tauri::command]
pub async fn seminar_umsatz_prognose(
    stichtag: chrono::NaiveDate,
    ctx: State<'_, YamsAppApi>,
) -> Result<SeminarUmsatzPrognose, String> {
    map_report(ctx.seminar_umsatz_prognose(stichtag).await)
}
