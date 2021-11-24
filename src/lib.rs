use serde::{Serialize, Deserialize,de::Deserializer};
use serde::ser::Serializer;
use validator::Validate;
use regex::Regex;
use std::fmt::Display;
use std::fmt::Formatter;
#[macro_use]
extern crate lazy_static;

#[derive(Clone, Debug, PartialEq, Validate, Serialize, Deserialize)]
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

impl Display for Buchungsstapel{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}\n",self.header))?;
        for buchung in &self.buchungen {
            buchung.fmt(f)?;
            // f.write_str(&format!("{}\n", buchung))?;
        }
        Ok(())
        // write!(f, "{}\n{}", self.header, buchungen_str)       
    }
}

impl TryFrom<&str> for Buchungsstapel {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split;
        if value.contains("\r\n") {
            split = value.split("\r\n");
        }else{
            split = value.split("\n");
        }
        let vec: Vec<&str> = split.collect();
        
        let mut rdr = csv::ReaderBuilder::new().delimiter(b';')
            .has_headers(false).from_reader(vec.get(0).unwrap().as_bytes());
        let mut iter = rdr.deserialize();
        let mut header: Header = Header::default();
        if let Some(result) = iter.next() {
            header = result.unwrap();
            header.validate().unwrap();
        }
        let input = vec.get(2).unwrap();
        let input2: String = input.to_string();
        let input3: String = input2.replace(',',".");
        let mut rdr = csv::ReaderBuilder::new().delimiter(b';')
            .has_headers(false).from_reader(input3.as_bytes());
        let mut iter = rdr.deserialize();
        let mut buchungen: Vec<Buchung> = Vec::new();
        if let Some(result) = iter.next() {
            let b: Buchung = result.unwrap();
            b.validate().unwrap();
            buchungen.push(b);
        }
        let stapel = Buchungsstapel{
            header,
            buchungen,
        };
        Ok(stapel)
        // }else{
        //     Err("Header not found")
        // }
    }
}

lazy_static! {
    static ref KENNZEICHEN: Regex = Regex::new(r#"^(EXTF|DTVF)$"#).unwrap();
    static ref FORMATNAME: Regex = Regex::new(r#"^(Buchungsstapel|Wiederkehrende Buchungen|Debitoren/Kreditoren|Sachkontenbeschriftungen|Zahlungsbedingungen|Diverse Adressen)$"#).unwrap();
}

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
    rechnungslegungszweck: Option<u8>,
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

impl Display for Header{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let buchungstyp= match self.buchungstyp {
            Some(1) => "1",
            Some(2) => "2",
            _ => "1",
        };
        let rechnungslegungszweck= match self.rechnungslegungszweck {
            Some(0) => "0",
            Some(30) => "30",
            Some(40) => "40",
            Some(50) => "50",
            Some(64) => "64",
            None => "0",
            _ => "0",
        };
        write!(f, r#""{kennzeichen}";{versionsnummer};{format_kategorie};"{format_name}";{format_version};{erzeugt_am};{leerfeld1};{leerfeld2};{leerfeld3};{leerfeld4};{beraternummer};{mandantennummer};{wj_beginn};{sachkontenlänge};{datum_von};{datum_bis};{bezeichnung};{diktatkürzel};{buchungstyp};{rechnungslegungszweck};{buchungstyp};{rechnungslegungszweck};{festschreibung};{wkz};{leerfeld5};{derivatskennzeichen};{leerfeld6};{leerfeld7};{sachkontenrahmen};{id_der_branchenlösung};{leerfeld8};{leerfeld9};{anwendungsinformation}\n"#,
            kennzeichen=self.kennzeichen,
            versionsnummer=self.versionsnummer,
            format_kategorie=self.format_kategorie,
            format_name=self.format_name,
            format_version=self.format_version,
            erzeugt_am=self.erzeugt_am,
            leerfeld1=self.leerfeld1,
            leerfeld2=self.leerfeld2,
            leerfeld3=self.leerfeld3,
            leerfeld4=self.leerfeld4,
            beraternummer=self.beraternummer,
            mandantennummer=self.mandantennummer,
            wj_beginn=self.wj_beginn,
            sachkontenlänge=self.sachkontenlänge,
            datum_von=self.datum_von,
            datum_bis=self.datum_bis,
            bezeichnung=self.bezeichnung,
            diktatkürzel=self.diktatkürzel,
            buchungstyp=buchungstyp,
            rechnungslegungszweck=rechnungslegungszweck,
            festschreibung=self.festschreibung,
            wkz=self.wkz,
            leerfeld5=self.leerfeld5,
            derivatskennzeichen=self.derivatskennzeichen,
            leerfeld6=self.leerfeld6,
            leerfeld7=self.leerfeld7,
            sachkontenrahmen=self.sachkontenrahmen,
            id_der_branchenlösung=self.id_der_branchenlösung,
            leerfeld8=self.leerfeld8,
            leerfeld9=self.leerfeld9,
            anwendungsinformation=self.anwendungsinformation,
        )
    }
}

impl TryFrom<&str> for Header {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut rdr = csv::ReaderBuilder::new().delimiter(b';')
            .has_headers(false).from_reader(value.as_bytes());
        let mut iter = rdr.deserialize();
        if let Some(result) = iter.next() {
            let header: Header = result.unwrap();
            header.validate().unwrap();
            Ok(header)
        }else{
            Err("Header not found")
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
impl Display for Festschreibung{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Festschreibung::KeineFestschreibung => "0",
            Festschreibung::Festschreibung => "1",
        })
    }
}

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

impl Default for Buchung {
    fn default() -> Self {
        Buchung {
            umsatz: 0.0,
            soll_haben_kennzeichen: "H".to_string(),
            wkz_umsatz: "EUR".to_string(),
            kurs: 0.0,
            basis_umsatz: 0.0,
            wkz_basis_umsatz: "".to_string(),
            konto: 0,
            gegenkonto: 0,
            bu_schlüssel: 0,
            beleg_datum: 0,
            belegfeld1: "".to_string(),
            belegfeld2: "".to_string(),
            skonto: 0.0,
            buchungstext: "".to_string(),
            posten_sperre: 0,
            diverse_adressnummer: "".to_string(),
            geschäftspartner_bank: "".to_string(),
            sachverhalt: "".to_string(),
            zinssperre: "".to_string(),
            beleg_link: "".to_string(),
            beleg_info_art1: "".to_string(),
            beleg_info_inhalt1: "".to_string(),
            beleg_info_art2: "".to_string(),
            beleg_info_inhalt2: "".to_string(),
            beleg_info_art3: "".to_string(),
            beleg_info_inhalt3: "".to_string(),
            beleg_info_art4: "".to_string(),
            beleg_info_inhalt4: "".to_string(),
            beleg_info_art5: "".to_string(),
            beleg_info_inhalt5: "".to_string(),
            beleg_info_art6: "".to_string(),
            beleg_info_inhalt6: "".to_string(),
            beleg_info_art7: "".to_string(),
            beleg_info_inhalt7: "".to_string(),
            beleg_info_art8: "".to_string(),
            beleg_info_inhalt8: "".to_string(),
            kost1_kostenstelle: "".to_string(),
            kost2_kostenstelle: "".to_string(),
            kost_menge: 0.0,
            eu_ustid: "".to_string(),
            eu_steuersatz: 0.0,
            abweichende_versteuerungsart: "".to_string(),
            sachverhalt_l_l: "".to_string(),
            funktionsergänzung_l_l: "".to_string(),
            bu_49_hauptfunktiontyp: "".to_string(),
            bu_49_hauptfunktionsnummer: "".to_string(),
            bu_49_funktionsergänzung: "".to_string(),
            zusatzinformation_art1: "".to_string(),
            zusatzinformation_inhalt1: "".to_string(),
            zusatzinformation_art2: "".to_string(),
            zusatzinformation_inhalt2: "".to_string(),
            zusatzinformation_art3: "".to_string(),
            zusatzinformation_inhalt3: "".to_string(),
            zusatzinformation_art4: "".to_string(),
            zusatzinformation_inhalt4: "".to_string(),
            zusatzinformation_art5: "".to_string(),
            zusatzinformation_inhalt5: "".to_string(),
            zusatzinformation_art6: "".to_string(),
            zusatzinformation_inhalt6: "".to_string(),
            zusatzinformation_art7: "".to_string(),
            zusatzinformation_inhalt7: "".to_string(),
            zusatzinformation_art8: "".to_string(),
            zusatzinformation_inhalt8: "".to_string(),
            zusatzinformation_art9: "".to_string(),
            zusatzinformation_inhalt9: "".to_string(),
            zusatzinformation_art10: "".to_string(),    
            zusatzinformation_inhalt10: "".to_string(),
            zusatzinformation_art11: "".to_string(),
            zusatzinformation_inhalt11: "".to_string(),
            zusatzinformation_art12: "".to_string(),
            zusatzinformation_inhalt12: "".to_string(),
            zusatzinformation_art13: "".to_string(),
            zusatzinformation_inhalt13: "".to_string(),
            zusatzinformation_art14: "".to_string(),
            zusatzinformation_inhalt14: "".to_string(),
            zusatzinformation_art15: "".to_string(),
            zusatzinformation_inhalt15: "".to_string(),
            zusatzinformation_art16: "".to_string(),
            zusatzinformation_inhalt16: "".to_string(),
            zusatzinformation_art17: "".to_string(),
            zusatzinformation_inhalt17: "".to_string(),
            zusatzinformation_art18: "".to_string(),
            zusatzinformation_inhalt18: "".to_string(),
            zusatzinformation_art19: "".to_string(),
            zusatzinformation_inhalt19: "".to_string(),
            zusatzinformation_art20: "".to_string(),
            zusatzinformation_inhalt20: "".to_string(),
            stück: 0.0,
            gewicht: 0.0,
            zahlweise: "".to_string(),
            forderungsart: "".to_string(),
            forderungsjahr: 0,
            veranlagungsjahr: 0,
            zugeordnete_fälligkeit: "".to_string(),
            skonto_typ: "".to_string(),
            auftragsnummer: "".to_string(),
            buchungstyp: "".to_string(),
            ust_schlüssel_anzahlung: "".to_string(),
            eu_mitgliedstaat_anzahlung: "".to_string(),
            sachverhalt_l_l_anzahlung: "".to_string(),
            eu_steuersatz_anzahlung: 0.0,
            erlöskonto_anzahlung: 0.0,
            herkunft_kz: "".to_string(),
            leerfeld: "".to_string(),
            kost_datum: "".to_string(),
            sepa_mandatsreferenz: "".to_string(),
            skontosperre: "".to_string(),
            gesellschaftername: "".to_string(),
            beteiligtennummer: "".to_string(),
            identifikationsnummer: "".to_string(),
            zeichennummer: "".to_string(),
            postensperre_bis: "".to_string(),
            bezeichnung_so_bil_sachverhalt: "".to_string(),
            kennzeichen_so_bil_buchung: "".to_string(),
            festschreibung: "".to_string(),
            leistungsdatum: "".to_string(),
            datum_zuord_steuerperiode: "".to_string(),
            fälligkeit: "".to_string(),
            generalumkehr: "".to_string(),
            steuersatz: 0.0,
            land: "".to_string(),
            abrechnungsreferenz: "".to_string(),
            bvv_position: "".to_string(),
            eu_ustid_ursprung: "".to_string(),
            eu_steuersatz_ursprung: 0.0,
        }
    }
}

impl Display for Buchung{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{umsatz};{soll_haben_kennzeichen};{wkz_umsatz};{kurs};{basis_umsatz};{wkz_basis_umsatz};{konto};{gegenkonto};{bu_schlüssel};{beleg_datum};{belegfeld1};{belegfeld2};{skonto};{buchungstext};{posten_sperre};{diverse_adressnummer};{geschäftspartner_bank};{sachverhalt};{zinssperre};{beleg_link};{beleg_info_art1};{beleg_info_inhalt1};{beleg_info_art2};{beleg_info_inhalt2};{beleg_info_art3};{beleg_info_inhalt3};{beleg_info_art4};{beleg_info_inhalt4};{beleg_info_art5};{beleg_info_inhalt5};{beleg_info_art6};{beleg_info_inhalt6};{beleg_info_art7};{beleg_info_inhalt7};{beleg_info_art8};{beleg_info_inhalt8};{kost1_kostenstelle};{kost2_kostenstelle};{kost_menge};{eu_ustid};{eu_steuersatz};{abweichende_versteuerungsart};{sachverhalt_l_l};{funktionsergänzung_l_l};{bu_49_hauptfunktiontyp};{bu_49_hauptfunktionsnummer};{bu_49_funktionsergänzung};{zusatzinformation_art1};{zusatzinformation_inhalt1};{zusatzinformation_art2};{zusatzinformation_inhalt2};{zusatzinformation_art3};{zusatzinformation_inhalt3};{zusatzinformation_art4};{zusatzinformation_inhalt4};{zusatzinformation_art5};{zusatzinformation_inhalt5};{zusatzinformation_art6};{zusatzinformation_inhalt6};{zusatzinformation_art7};{zusatzinformation_inhalt7};{zusatzinformation_art8};{zusatzinformation_inhalt8};{zusatzinformation_art9};{zusatzinformation_inhalt9};{zusatzinformation_art10};{zusatzinformation_inhalt10};{zusatzinformation_art11};{zusatzinformation_inhalt11};{zusatzinformation_art12};{zusatzinformation_inhalt12};{zusatzinformation_art13};{zusatzinformation_inhalt13};{zusatzinformation_art14};{zusatzinformation_inhalt14};{zusatzinformation_art15};{zusatzinformation_inhalt15};{zusatzinformation_art16};{zusatzinformation_inhalt16};{zusatzinformation_art17};{zusatzinformation_inhalt17};{zusatzinformation_art18};{zusatzinformation_inhalt18};{zusatzinformation_art19};{zusatzinformation_inhalt19};{zusatzinformation_art20};{zusatzinformation_inhalt20};{stück};{gewicht};{zahlweise};{forderungsart};{forderungsjahr};{veranlagungsjahr};{zugeordnete_fälligkeit};{skonto_typ};{auftragsnummer};{buchungstyp};{ust_schlüssel_anzahlung};{eu_mitgliedstaat_anzahlung};{sachverhalt_l_l_anzahlung};{eu_steuersatz_anzahlung};{erlöskonto_anzahlung};{herkunft_kz};{leerfeld};{kost_datum};{sepa_mandatsreferenz};{skontosperre};{gesellschaftername};{beteiligtennummer};{identifikationsnummer};{zeichennummer};{postensperre_bis};{bezeichnung_so_bil_sachverhalt};{kennzeichen_so_bil_buchung};{festschreibung};{leistungsdatum};{datum_zuord_steuerperiode};{fälligkeit};{generalumkehr};{steuersatz};{land};{abrechnungsreferenz};{bvv_position};{eu_ustid_ursprung};{eu_steuersatz_ursprung}\n"#,
        umsatz = self.umsatz,
        soll_haben_kennzeichen = self.soll_haben_kennzeichen,
        wkz_umsatz = self.wkz_umsatz,
        kurs = self.kurs,
        basis_umsatz = self.basis_umsatz,
        wkz_basis_umsatz = self.wkz_basis_umsatz,
        konto = self.konto,
        gegenkonto = self.gegenkonto,
        bu_schlüssel = self.bu_schlüssel,
        beleg_datum = self.beleg_datum,
        belegfeld1 = self.belegfeld1,
        belegfeld2 = self.belegfeld2,
        skonto = self.skonto,
        buchungstext = self.buchungstext,
        posten_sperre = self.posten_sperre,
        diverse_adressnummer = self.diverse_adressnummer,
        geschäftspartner_bank = self.geschäftspartner_bank,
        sachverhalt = self.sachverhalt,
        zinssperre = self.zinssperre,
        beleg_link = self.beleg_link,
        beleg_info_art1 = self.beleg_info_art1,
        beleg_info_inhalt1 = self.beleg_info_inhalt1,
        beleg_info_art2 = self.beleg_info_art2,
        beleg_info_inhalt2 = self.beleg_info_inhalt2,
        beleg_info_art3 = self.beleg_info_art3,
        beleg_info_inhalt3 = self.beleg_info_inhalt3,
        beleg_info_art4 = self.beleg_info_art4,
        beleg_info_inhalt4 = self.beleg_info_inhalt4,
        beleg_info_art5 = self.beleg_info_art5,
        beleg_info_inhalt5 = self.beleg_info_inhalt5,
        beleg_info_art6 = self.beleg_info_art6,
        beleg_info_inhalt6 = self.beleg_info_inhalt6,
        beleg_info_art7 = self.beleg_info_art7,
        beleg_info_inhalt7 = self.beleg_info_inhalt7,
        beleg_info_art8 = self.beleg_info_art8,
        beleg_info_inhalt8 = self.beleg_info_inhalt8,
        kost1_kostenstelle = self.kost1_kostenstelle,
        kost2_kostenstelle = self.kost2_kostenstelle,
        kost_menge = self.kost_menge,
        eu_ustid = self.eu_ustid,
        eu_steuersatz = self.eu_steuersatz,
        abweichende_versteuerungsart = self.abweichende_versteuerungsart,
        sachverhalt_l_l = self.sachverhalt_l_l,
        funktionsergänzung_l_l = self.funktionsergänzung_l_l,
        bu_49_hauptfunktiontyp = self.bu_49_hauptfunktiontyp,
        bu_49_hauptfunktionsnummer = self.bu_49_hauptfunktionsnummer,
        bu_49_funktionsergänzung = self.bu_49_funktionsergänzung,
        zusatzinformation_art1 = self.zusatzinformation_art1,
        zusatzinformation_inhalt1 = self.zusatzinformation_inhalt1,
        zusatzinformation_art2 = self.zusatzinformation_art2,
        zusatzinformation_inhalt2 = self.zusatzinformation_inhalt2,
        zusatzinformation_art3 = self.zusatzinformation_art3,
        zusatzinformation_inhalt3 = self.zusatzinformation_inhalt3,
        zusatzinformation_art4 = self.zusatzinformation_art4,
        zusatzinformation_inhalt4 = self.zusatzinformation_inhalt4,
        zusatzinformation_art5 = self.zusatzinformation_art5,
        zusatzinformation_inhalt5 = self.zusatzinformation_inhalt5,
        zusatzinformation_art6 = self.zusatzinformation_art6,
        zusatzinformation_inhalt6 = self.zusatzinformation_inhalt6,
        zusatzinformation_art7 = self.zusatzinformation_art7,
        zusatzinformation_inhalt7 = self.zusatzinformation_inhalt7,
        zusatzinformation_art8 = self.zusatzinformation_art8,
        zusatzinformation_inhalt8 = self.zusatzinformation_inhalt8,
        zusatzinformation_art9 = self.zusatzinformation_art9,
        zusatzinformation_inhalt9 = self.zusatzinformation_inhalt9,
        zusatzinformation_art10 = self.zusatzinformation_art10,
        zusatzinformation_inhalt10 = self.zusatzinformation_inhalt10,
        zusatzinformation_art11 = self.zusatzinformation_art11,
        zusatzinformation_inhalt11 = self.zusatzinformation_inhalt11,
        zusatzinformation_art12 = self.zusatzinformation_art12,
        zusatzinformation_inhalt12 = self.zusatzinformation_inhalt12,
        zusatzinformation_art13 = self.zusatzinformation_art13,
        zusatzinformation_inhalt13 = self.zusatzinformation_inhalt13,
        zusatzinformation_art14 = self.zusatzinformation_art14,
        zusatzinformation_inhalt14 = self.zusatzinformation_inhalt14,
        zusatzinformation_art15 = self.zusatzinformation_art15,
        zusatzinformation_inhalt15 = self.zusatzinformation_inhalt15,
        zusatzinformation_art16 = self.zusatzinformation_art16,
        zusatzinformation_inhalt16 = self.zusatzinformation_inhalt16,
        zusatzinformation_art17 = self.zusatzinformation_art17,
        zusatzinformation_inhalt17 = self.zusatzinformation_inhalt17,
        zusatzinformation_art18 = self.zusatzinformation_art18,
        zusatzinformation_inhalt18 = self.zusatzinformation_inhalt18,
        zusatzinformation_art19 = self.zusatzinformation_art19,
        zusatzinformation_inhalt19 = self.zusatzinformation_inhalt19,
        zusatzinformation_art20 = self.zusatzinformation_art20,
        zusatzinformation_inhalt20 = self.zusatzinformation_inhalt20,
        stück = self.stück,
        gewicht = self.gewicht,
        zahlweise = self.zahlweise,
        forderungsart = self.forderungsart,
        forderungsjahr = self.forderungsjahr,
        veranlagungsjahr = self.veranlagungsjahr,
        zugeordnete_fälligkeit = self.zugeordnete_fälligkeit,
        skonto_typ = self.skonto_typ,
        auftragsnummer = self.auftragsnummer,
        buchungstyp = self.buchungstyp,
        ust_schlüssel_anzahlung = self.ust_schlüssel_anzahlung,
        eu_mitgliedstaat_anzahlung = self.eu_mitgliedstaat_anzahlung,
        sachverhalt_l_l_anzahlung = self.sachverhalt_l_l_anzahlung,
        eu_steuersatz_anzahlung = self.eu_steuersatz_anzahlung,
        erlöskonto_anzahlung = self.erlöskonto_anzahlung,
        herkunft_kz = self.herkunft_kz,
        leerfeld = self.leerfeld,
        kost_datum = self.kost_datum,
        sepa_mandatsreferenz = self.sepa_mandatsreferenz,
        skontosperre = self.skontosperre,
        gesellschaftername = self.gesellschaftername,
        beteiligtennummer = self.beteiligtennummer,
        identifikationsnummer = self.identifikationsnummer,
        zeichennummer = self.zeichennummer,
        postensperre_bis = self.postensperre_bis,
        bezeichnung_so_bil_sachverhalt = self.bezeichnung_so_bil_sachverhalt,
        kennzeichen_so_bil_buchung = self.kennzeichen_so_bil_buchung,
        festschreibung = self.festschreibung,
        leistungsdatum = self.leistungsdatum,
        datum_zuord_steuerperiode = self.datum_zuord_steuerperiode,
        fälligkeit = self.fälligkeit,
        generalumkehr = self.generalumkehr,
        steuersatz = self.steuersatz,
        land = self.land,
        abrechnungsreferenz = self.abrechnungsreferenz,
        bvv_position = self.bvv_position,
        eu_ustid_ursprung = self.eu_ustid_ursprung,
        eu_steuersatz_ursprung = self.eu_steuersatz_ursprung,
        )      
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebKred_Stamm{
    header: Header,
    debitoren_kreditoren: Vec<DebKred>,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebKred {
    konto: u32,
}

#[test]
fn valid_header() {
    let str = r#""EXTF";510;21;"Buchungsstapel";7;20211106165314647;;"";"";"";1000;1;20190101;4;20190101;20191231;"";"";;;;"";;"";;;"";;;"";"""#;
    
    let result = Header::try_from(str);
    assert!(result.is_ok());
    let header = result.unwrap();
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

#[test]
fn full_cycle_header() {
    use csv::WriterBuilder;

    let header = Header{
        format_name: "Buchungsstapel".to_string(),
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
    let str = r#""INVALIDFORMAT";510;21;"Buchungsstapel";7;20211106165314647;;"";"";"";1000;1;20190101;4;20190101;20191231;"";"";;;;"";;"";;;"";;;"";"""#;
    let result = Header::try_from(str);
    assert!(result.is_ok());
    let header = result.unwrap();
    header.validate().unwrap();
}

#[test]
fn einzelbuchung() {
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
    let buchung = Buchung{
        soll_haben_kennzeichen: "S".to_string(),
        umsatz: 100.0,
        beleg_datum: 2802,
        konto: 1800,
        gegenkonto: 1420,
        buchungstext: "zahlung 123".to_string(),
        ..Default::default()
    };
    let stapel = Buchungsstapel{
        header: header,
        buchungen: vec![buchung],
    };
    let _str = format!("{}", stapel);
}

#[test]
fn test_extf_buchungstapel() {
    use std::io::Read;
    let mut f = std::fs::File::open("./test-data/EXTF_Buchungsstapel.csv").unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    //length in windows encoding
    assert_eq!(buffer.len(), 21426);
    let (cow, encoding_used, had_errors) = encoding_rs::WINDOWS_1252.decode(&buffer);
    assert_eq!(had_errors, false);
    assert_eq!(encoding_used, encoding_rs::WINDOWS_1252);
    //length in utf-8 encoding
    assert_eq!(cow.len(), 21447);
    let str: String = cow.to_string();
    println!("{}", str);
    println!("done.");
    let _stapel: Buchungsstapel = Buchungsstapel::try_from(str.as_str()).unwrap();
}