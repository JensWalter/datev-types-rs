use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize,de::Deserializer};
use serde::ser::Serializer;
use validator::Validate;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Buchungsstapel{
    header: Header,
    buchungen: Vec<Buchung>,
}

impl Default for Buchungsstapel{
    fn default() -> Self {
        Buchungsstapel{
            header: Header::default(),
            buchungen: Vec::new(),
        }
    }
}

lazy_static! {
    static ref KENNZEICHEN: Regex = Regex::new(r#"^(EXTF|DTVF)$"#).unwrap();
    static ref FORMATNAME: Regex = Regex::new(r#"^(Buchungsstapel|Wiederkehrende Buchungen|Debitoren/Kreditoren|Sachkontenbeschriftungen|Zahlungsbedingungen|Diverse Adressen)$"#).unwrap();
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq, Validate, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
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
    rechnungslegungszweck: Option<String>,
    /// 0 = keine Festschreibung
    /// 1 = Festschreibung (default)
    // #[serde(default = "default_festschreibung")]
    // festschreibung: Option<Festschreibung>,
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

impl Default for Header{
    fn default() -> Self {
        Header{
            kennzeichen: String::from("EXTF"),
            versionsnummer: 700,
            format_kategorie: 21,
            format_name: String::from("Buchungsstapel"),
            format_version: 12,
            erzeugt_am: 0,
            leerfeld1: String::from(""),
            leerfeld2: String::from(""),
            leerfeld3: String::from(""),
            leerfeld4: String::from(""),
            beraternummer: 0,
            mandantennummer: 0,
            wj_beginn: 0,
            sachkontenlänge: 0,
            datum_von: 0,
            datum_bis: 0,
            bezeichnung: String::from(""),
            diktatkürzel: String::from(""),
            buchungstyp: None,
            rechnungslegungszweck: None,
            festschreibung: Festschreibung::default(),
            wkz: String::from("EUR"),
            leerfeld5: String::from(""),
            derivatskennzeichen: String::from(""),
            leerfeld6: String::from(""),
            leerfeld7: String::from(""),
            sachkontenrahmen: String::from(""),
            id_der_branchenlösung: String::from(""),
            leerfeld8: String::from(""),
            leerfeld9: String::from(""),
            anwendungsinformation: String::from(""),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Eq)]
pub enum Festschreibung{
    KeineFestschreibung,
    Festschreibung,
}
impl Default for Festschreibung{
    fn default() -> Self {
        Festschreibung::Festschreibung
    }
}
impl<'de> Deserialize<'de> for Festschreibung {
    fn deserialize<D>(deserializer: D) -> Result<Festschreibung, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        match s.as_str() {
            "0" => Ok(Festschreibung::KeineFestschreibung),
            "1" => Ok(Festschreibung::Festschreibung),
            _ => Ok(Festschreibung::default()),
        }
    }
}
impl Serialize for Festschreibung {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Festschreibung::KeineFestschreibung => serializer.serialize_u8(0),
            Festschreibung::Festschreibung => serializer.serialize_u8(1),
        }
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct Buchung{
    /// Umsatz/Betrag für den Datensatz
    /// z.B.: 1234567890,12
    /// Betrag muss immer positiv sein.
    umsatz: f64,
    /// Soll-/Haben-Kennzeichnung
    /// bezieht sich auf das Feld #7
    /// Konto
    /// S = SOLL (default)
    /// H = HABEN
    soll_haben_kennzeichen: String,
    /// ISO-Code der Währung
    /// #22 aus Header = default
    wkz_umsatz: String,
    /// Wenn Umsatz in Fremdwährung bei #1 angegeben wird
    /// #004, 005 und 006 sind zu übergeben
    /// z.B.: 1234,123456
    kurs: f64,

    basis_umsatz: f64,
    wkz_basis_umsatz: String,
    konto: u32,
    gegenkonto: u32,
    bu_schlüssel: u32,
    /// Format: TTMM, z.B. 0105
    /// Das Jahr wird immer aus
    /// dem Feld 13 des Headers ermittelt
    beleg_datum: u16,
    belegfeld1: String,
    belegfeld2: String,
    skonto: f64,
    #[validate(length(min = 0, max = 60))]
    buchungstext: String,
    posten_sperre: u8,
    diverse_adressnummer: String,
    geschäftspartner_bank: String,
    sachverhalt: String,
    zinssperre: String,
    beleg_link: String,
    beleg_info_art1: String,
    beleg_info_inhalt1: String,
    beleg_info_art2: String,
    beleg_info_inhalt2: String,
    beleg_info_art3: String,
    beleg_info_inhalt3: String,
    beleg_info_art4: String,
    beleg_info_inhalt4: String,
    beleg_info_art5: String,
    beleg_info_inhalt5: String,
    beleg_info_art6: String,
    beleg_info_inhalt6: String,
    beleg_info_art7: String,
    beleg_info_inhalt7: String,
    beleg_info_art8: String,
    beleg_info_inhalt8: String,
    kost1_kostenstelle: String,
    kost2_kostenstelle: String,
    kost_menge: f64,
    eu_ustid: String,
    eu_steuersatz: f64,
    abweichende_versteuerungsart: String,
    sachverhalt_l_l: String,
    funktionsergänzung_l_l: String,
    bu_49_hauptfunktiontyp: String,
    bu_49_hauptfunktionsnummer: String,
    bu_49_funktionsergänzung: String,
    zusatzinformation_art1: String,
    zusatzinformation_inhalt1: String,
    zusatzinformation_art2: String,
    zusatzinformation_inhalt2: String,
    zusatzinformation_art3: String,
    zusatzinformation_inhalt3: String,
    zusatzinformation_art4: String,
    zusatzinformation_inhalt4: String,
    zusatzinformation_art5: String,
    zusatzinformation_inhalt5: String,
    zusatzinformation_art6: String,
    zusatzinformation_inhalt6: String,
    zusatzinformation_art7: String,
    zusatzinformation_inhalt7: String,
    zusatzinformation_art8: String,
    zusatzinformation_inhalt8: String,
    zusatzinformation_art9: String,
    zusatzinformation_inhalt9: String,
    zusatzinformation_art10: String,    
    zusatzinformation_inhalt10: String,
    zusatzinformation_art11: String,
    zusatzinformation_inhalt11: String,
    zusatzinformation_art12: String,
    zusatzinformation_inhalt12: String,
    zusatzinformation_art13: String,
    zusatzinformation_inhalt13: String,
    zusatzinformation_art14: String,
    zusatzinformation_inhalt14: String,
    zusatzinformation_art15: String,
    zusatzinformation_inhalt15: String,
    zusatzinformation_art16: String,
    zusatzinformation_inhalt16: String,
    zusatzinformation_art17: String,
    zusatzinformation_inhalt17: String,
    zusatzinformation_art18: String,
    zusatzinformation_inhalt18: String,
    zusatzinformation_art19: String,
    zusatzinformation_inhalt19: String,
    zusatzinformation_art20: String,
    zusatzinformation_inhalt20: String,
    stück: f64,
    gewicht: f64,
    zahlweise: String,
    forderungsart: String,
    forderungsjahr: u16,
    veranlagungsjahr: u16,
    zugeordnete_fälligkeit: String,
    skonto_typ: String,
    auftragsnummer: String,
    buchungstyp: String,
    ust_schlüssel_anzahlung: String,
    eu_mitgliedstaat_anzahlung: String,
    sachverhalt_l_l_anzahlung: String,
    eu_steuersatz_anzahlung: f64,
    erlöskonto_anzahlung: f32,
    herkunft_kz: String,
    leerfeld: String,
    kost_datum: String,
    sepa_mandatsreferenz: String,
    skontosperre: String,
    gesellschaftername: String,
    beteiligtennummer: String,
    identifikationsnummer: String,
    zeichennummer: String,
    postensperre_bis: String,
    bezeichnung_so_bil_sachverhalt: String,
    kennzeichen_so_bil_buchung: String,
    festschreibung: String,
    leistungsdatum: String,
    datum_zuord_steuerperiode: String,
    fälligkeit: String,
    generalumkehr: String,
    steuersatz: f64,
    land: String,
    abrechnungsreferenz: String,
    bvv_position: String,
    eu_ustid_ursprung: String,
    eu_steuersatz_ursprung: f64,
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
        assert_eq!(header.buchungstyp, None);
        assert_eq!(header.rechnungslegungszweck, None);
        assert_eq!(header.festschreibung, Festschreibung::default());
    }
}

#[test]
fn full_cycle_header() {
    use csv::WriterBuilder;

    let header = Header{
        kennzeichen: "EXTF".to_string(),
        versionsnummer: 700,
        format_kategorie: 21,
        format_name: "Buchungsstapel".to_string(),
        format_version: 7,
        erzeugt_am: 20211106165314647,
        beraternummer: 1000,
        mandantennummer: 1,
        wj_beginn: 20190101,
        sachkontenlänge: 4,
        datum_von: 20190101,
        datum_bis: 20191231,
        ..Default::default()
    };
    let mut wtr = WriterBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_writer(Vec::new());
    wtr.serialize(header.clone()).unwrap();
    let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';')
        .has_headers(false).from_reader(data.as_bytes());
    let mut iter = rdr.deserialize();
    if let Some(result) = iter.next() {
        let header2: Header = result.unwrap();
        assert_eq!(header, header2);
    }else{
        panic!("no header");
    }
}
#[test]
#[should_panic]
fn invalid_header() {
    let str = r#""BLAH";510;21;"Buchungsstapel";7;20211106165314647;;"";"";"";1000;1;20190101;4;20190101;20191231;"";"";;;;"";;"";;;"";;;"";"""#;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';')
    .has_headers(false).from_reader(str.as_bytes());
    let mut iter = rdr.deserialize();
    if let Some(result) = iter.next() {
        let header: Header = result.unwrap();
        header.validate().unwrap();
    }
}