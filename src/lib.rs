use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use validator::Validate;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Buchungsstapel{
    header: String,
}

impl Default for Buchungsstapel{
    fn default() -> Self {
        Buchungsstapel{
            header: String::from(""),
        }
    }
}

lazy_static! {
    static ref KENNZEICHEN: Regex = Regex::new(r#"^(EXTF|DTVF)$"#).unwrap();
    static ref FORMATNAME: Regex = Regex::new(r#"^(Buchungsstapel|Wiederkehrende Buchungen|Debitoren/Kreditoren|Sachkontenbeschriftungen|Zahlungsbedingungen|Diverse Adressen)$"#).unwrap();
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq, Validate, Deserialize, Serialize)]
pub struct Header{
    #[validate(regex = "KENNZEICHEN")]
    kennzeichen: String,
    versionsnummer: u32,
    /// 16 = Debitoren-/Kreditoren
    /// 20 = Sachkontenbeschriftungen
    /// 21 = Buchungsstapel
    /// 46 = Zahlungsbedingungen
    /// 48 = Diverse Adressen
    /// 65 = Wiederkehrende Buchungen
    format_kategorie: u16,
    #[validate(regex = "FORMATNAME")]
    format_name: String,
    /// Debitoren-/Kreditoren = 5
    /// Sachkontenbeschriftungen = 3
    /// Buchungsstapel = 12
    /// Zahlungsbedingungen = 2
    /// Wiederkehrende Buchungen = 4
    /// Diverse Adressen = 2
    format_version: u16,
    /// Zeitstempel:
    /// YYYYMMDDHHMMSSFFF
    erzeugt_am: u64,
    leerfeld1: String,
    leerfeld2: String,
    leerfeld3: String,
    leerfeld4: String,
    /// Bereich 1001-9999999
    beraternummer: u32,
    /// Bereich 1-99999
    mandantennummer: u32,
    /// Wirtschaftsjahresbeginn
    /// Format: YYYYMMDD
    wj_beginn: u64,
    /// Nummernlänge der Sachkonten.
    /// Wert muss beim Import mit Konfiguration des Mandats in der DATEV App übereinstimmen.
    sachkontenlänge: u32,
    /// Beginn der Periode des Stapels
    /// Format: YYYYMMDD
    datum_von: u32,
    /// Ende der Periode des Stapels
    /// Format: YYYYMMDD
    datum_bis: u32,
    /// Bezeichnung des Stapels
    /// z.B. „Rechnungsausgang 09/2019“
    #[validate(length(min = 0, max = 30))]
    bezeichnung: String,
    /// Kürzel in Großbuchstaben des Bearbeiters
    /// z.B. "MM" für Max Mustermann
    #[validate(length(min = 0, max = 2))]
    diktatkürzel: String,
    /// 1 = Finanzbuchführung (default)
    /// 2 = Jahresabschluss
    buchungstyp: Option<u8>,
    /// 0 = unabhängig (default)
    /// 30 = Steuerrecht
    /// 40 = Kalkulatorik
    /// 50 = Handelsrecht
    /// 64 = IFRS
    rechnungslegungszweck: Option<u8>,
    /// 0 = keine Festschreibung
    /// 1 = Festschreibung (default)
    festschreibung: Festschreibung,
    /// ISO-Code der Währung "EUR" = default
    #[validate(length(min = 0, max = 3))]
    wkz: String,
    leerfeld5: String,
    derivatskennzeichen: String,
    leerfeld6: String,
    leerfeld7: String,
    /// Sachkontenrahmen der für die Bewegungsdaten verwendet wurde
    sachkontenrahmen: String,
    /// Falls eine spezielle DATEV Branchenlösung genutzt wird.
    #[validate(length(min = 0, max = 4))]
    id_der_branchenlösung: String, 	
    leerfeld8: String,
    leerfeld9: String,
    /// Verarbeitungskennzeichen der abgebenden Anwendung
    // z.B. „09/2019“
    #[validate(length(min = 0, max = 16))]
    anwendungsinformation: String,
}

#[derive(Clone, PartialEq, Debug, Eq, Deserialize, Serialize)]
#[repr(u8)]
pub enum Festschreibung{
    KeineFestschreibung = 0,
    Festschreibung = 1,
}

impl Default for Festschreibung{
    fn default() -> Self {
        Festschreibung::Festschreibung
    }
}

#[wasm_bindgen]
impl Buchungsstapel {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Buchungsstapel {
        Buchungsstapel { ..Default::default() }
    }

}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Eq)]
pub struct Buchung{
    datum: String,
    kategorie: String,
    betrag: i64,
}

#[wasm_bindgen]
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GP_Stamm{
    x: String,
    t: String,
}

#[test]
fn valid_header() {
    let str = r#""EXTF";510;21;"Buchungsstapel";7;20211106165314647;;"";"";"";1000;1;20190101;4;20190101;20191231;"";"";;;;"";;"";;;"";;;"";"""#;
    
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';')
        .has_headers(false).from_reader(str.as_bytes());
    let mut iter = rdr.deserialize();
    if let Some(result) = iter.next() {
        let header: Header = result.unwrap();
        header.validate().unwrap();
        assert_eq!(header.kennzeichen, "EXTF");
        assert_eq!(header.versionsnummer, 510);
        assert_eq!(header.format_kategorie, 21);
        assert_eq!(header.format_name, "Buchungsstapel");
        assert_eq!(header.format_version, 7);
        assert_eq!(header.erzeugt_am, 20211106165314647);
        assert_eq!(header.beraternummer, 1000);
        assert_eq!(header.mandantennummer, 1);
        assert_eq!(header.wj_beginn, 20190101);
        assert_eq!(header.sachkontenlänge, 4);
        assert_eq!(header.datum_von, 20190101);
        assert_eq!(header.datum_bis, 20191231);
        assert_eq!(header.buchungstyp, Some(1));
        assert_eq!(header.rechnungslegungszweck, Some(0));
        assert_eq!(header.festschreibung, Festschreibung::Festschreibung);
        
    }
}

#[test]
#[should_panic]
fn invalid_header() {
    let str = r#""BLAH";510;21;"Buchungsstapel";7;20211106165314647;;"";"";"";1000;1;20190101;4;20190101;20191231;"";"";;;;"";;"";;;"";;;"";"""#;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';')
    .has_headers(false).from_reader(str.as_bytes());
    let mut iter = rdr.deserialize();
    let mut kennzeichen: String ="".to_string();
    if let Some(result) = iter.next() {
        let header: Header = result.unwrap();
        header.validate().unwrap();
        kennzeichen = header.kennzeichen.clone();
    }
    assert_ne!(kennzeichen,"EXTF");
}