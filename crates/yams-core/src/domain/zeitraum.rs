use chrono::{DateTime, Utc};
use error_stack::Report;

use crate::ResultReport;

#[derive(Debug, thiserror::Error)]
pub enum ZeitraumFehler {
    #[error("zeitraum-ende muss nach beginn liegen")]
    EndeNichtNachBeginn,
}

const CONSTRUCTING: &str = "while constructing zeitraum";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zeitraum {
    beginn: DateTime<Utc>,
    ende: DateTime<Utc>,
}

impl Zeitraum {
    pub fn neu(beginn: DateTime<Utc>, ende: DateTime<Utc>) -> ResultReport<Self, ZeitraumFehler> {
        if ende <= beginn {
            return Err(Report::new(ZeitraumFehler::EndeNichtNachBeginn).attach(CONSTRUCTING));
        }
        Ok(Self { beginn, ende })
    }

    pub fn beginn(&self) -> DateTime<Utc> {
        self.beginn
    }

    pub fn ende(&self) -> DateTime<Utc> {
        self.ende
    }
}

impl std::fmt::Display for Zeitraum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} – {}",
            self.beginn.format("%Y-%m-%d %H:%M UTC"),
            self.ende.format("%Y-%m-%d %H:%M UTC")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn utc(y: i32, m: u32, d: u32, h: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, h, 0, 0).unwrap()
    }

    #[test]
    fn zeitraum_accepts_ende_after_beginn() {
        let zeitraum = Zeitraum::neu(utc(2026, 8, 25, 10), utc(2026, 8, 25, 16)).unwrap();
        assert_eq!(zeitraum.beginn(), utc(2026, 8, 25, 10));
        assert_eq!(zeitraum.ende(), utc(2026, 8, 25, 16));
    }

    #[test]
    fn zeitraum_rejects_equal() {
        let t = utc(2026, 8, 25, 10);
        let err = Zeitraum::neu(t, t).unwrap_err();
        assert!(matches!(
            err.current_context(),
            ZeitraumFehler::EndeNichtNachBeginn
        ));
        assert!(format!("{err:?}").contains(CONSTRUCTING));
    }

    #[test]
    fn zeitraum_rejects_ende_before_beginn() {
        let err = Zeitraum::neu(utc(2026, 8, 25, 16), utc(2026, 8, 25, 10)).unwrap_err();
        assert!(matches!(
            err.current_context(),
            ZeitraumFehler::EndeNichtNachBeginn
        ));
    }
}
