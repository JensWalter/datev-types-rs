use serde::{Serialize, Deserialize,de::Deserializer};
use serde::ser::Serializer;
use validator::Validate;
use regex::Regex;
use std::fmt::Display;
use std::fmt::Formatter;
use crate::header::Festschreibung;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Buchung {
    /// Umsatz/Betrag für den Datensatz
    /// z.B.: 1234567890,12
    /// Betrag muss immer positiv sein.
    pub umsatz: f64,
    /// Soll-/Haben-Kennzeichnung
    /// bezieht sich auf das Feld #7
    /// Konto
    /// S = SOLL (default)
    /// H = HABEN
    pub soll_haben_kennzeichen: String,
    /// ISO-Code der Währung
    /// #22 aus Header = default
    pub wkz_umsatz: String,
    /// Wenn Umsatz in Fremdwährung bei #1 angegeben wird
    /// #004, 005 und 006 sind zu übergeben
    /// z.B.: 1234,123456
    pub kurs: Option<f64>,
    pub basis_umsatz: Option<f64>,
    pub wkz_basis_umsatz: String,
    pub konto: u32,
    pub gegenkonto: u32,
    pub bu_schlüssel: Option<u32>,
    /// Format: TTMM, z.B. 0105
    /// Das Jahr wird immer aus
    /// dem Feld 13 des Headers ermittelt
    pub beleg_datum: u16,
    pub belegfeld1: String,
    pub belegfeld2: String,
    pub skonto: Option<f64>,
    #[validate(length(min = 0, max = 60))]
    pub buchungstext: Option<String>,
    #[validate(range(min = 0, max = 1))]
    pub postensperre: Option<u8>,
    pub diverse_adressnummer: String,
    pub geschäftspartner_bank: String,
    pub sachverhalt: String,
    pub zinssperre: String,
    pub beleg_link: String,
    pub beleg_info_art1: String,
    pub beleg_info_inhalt1: String,
    pub beleg_info_art2: String,
    pub beleg_info_inhalt2: String,
    pub beleg_info_art3: String,
    pub beleg_info_inhalt3: String,
    pub beleg_info_art4: String,
    pub beleg_info_inhalt4: String,
    pub beleg_info_art5: String,
    pub beleg_info_inhalt5: String,
    pub beleg_info_art6: String,
    pub beleg_info_inhalt6: String,
    pub beleg_info_art7: String,
    pub beleg_info_inhalt7: String,
    pub beleg_info_art8: String,
    pub beleg_info_inhalt8: String,
    #[validate(length(min = 0, max = 36))]
    pub kost1_kostenstelle: Option<String>,
    #[validate(length(min = 0, max = 36))]
    pub kost2_kostenstelle: Option<String>,
    pub kost_menge: Option<f64>,
    pub eu_ustid: Option<String>,
    pub eu_steuersatz: Option<f64>,
    pub abweichende_versteuerungsart: String,
    pub sachverhalt_l_l: String,
    pub funktionsergänzung_l_l: String,
    pub bu_49_hauptfunktiontyp: String,
    pub bu_49_hauptfunktionsnummer: String,
    pub bu_49_funktionsergänzung: String,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art1: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt1: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art2: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt2: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art3: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt3: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art4: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt4: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art5: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt5: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art6: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt6: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art7: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt7: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art8: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt8: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art9: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt9: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art10: Option<String>,    
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt10: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art11: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt11: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art12: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt12: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art13: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt13: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art14: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt14: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art15: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt15: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art16: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt16: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art17: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt17: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art18: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt18: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art19: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt19: Option<String>,
    #[validate(length(min = 0, max = 20))]
    pub zusatzinformation_art20: Option<String>,
    #[validate(length(min = 0, max = 210))]
    pub zusatzinformation_inhalt20: Option<String>,
    pub stück: Option<f64>,
    pub gewicht: Option<f64>,
    pub zahlweise: Option<String>,
    pub forderungsart: Option<String>,
    #[validate(range(min = 0, max = 2099))]
    pub forderungsjahr: Option<u16>,
    pub veranlagungsjahr: Option<u16>,
    pub zugeordnete_fälligkeit: Option<String>,
    // #[validate(range(min = 1, max = 2))]
    pub skontotyp: Option<String>,
    pub auftragsnummer: Option<String>,
    pub buchungstyp: Option<String>,
    pub ust_schlüssel_anzahlung: Option<u8>,
    pub eu_mitgliedstaat_anzahlung: Option<String>,
    pub sachverhalt_l_l_anzahlung: Option<String>,
    pub eu_steuersatz_anzahlung: Option<f64>,
    pub erlöskonto_anzahlung: Option<u32>,
    pub herkunft_kz: Option<String>,
    pub leerfeld: Option<String>,
    /// Format TTMMJJJJ
    pub kost_datum: Option<String>,
    pub sepa_mandatsreferenz: Option<String>,
    pub skontosperre: Option<u8>,
    pub gesellschaftername: Option<String>,
    pub beteiligtennummer: Option<String>,
    pub identifikationsnummer: Option<String>,
    pub zeichennummer: Option<String>,
    pub postensperre_bis: Option<String>,
    pub bezeichnung_so_bil_sachverhalt: Option<String>,
    pub kennzeichen_so_bil_buchung: Option<String>,
    /// leer=nichtdefiniert;wird  automatisch festgeschrieben
    /// 0 = keine Festschreibung
    /// 1 = Festschreibung
    /// Hat ein Buchungssatz in diesem Feld den Inhalt 1, so wird der
    /// gesamte Stapel nach dem Import festgeschrieben.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub festschreibung: Option<Festschreibung>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leistungsdatum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datum_zuord_steuerperiode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fälligkeit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generalumkehr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steuersatz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub land: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abrechnungsreferenz: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bvv_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_ustid_ursprung: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_steuersatz_ursprung: Option<f64>,
}

impl Default for Buchung {
    fn default() -> Self {
        Buchung {
            umsatz: 0.0,
            soll_haben_kennzeichen: "H".to_string(),
            wkz_umsatz: "EUR".to_string(),
            kurs: None,
            basis_umsatz: None,
            wkz_basis_umsatz: "".to_string(),
            konto: 0,
            gegenkonto: 0,
            bu_schlüssel: None,
            beleg_datum: 0,
            belegfeld1: "".to_string(),
            belegfeld2: "".to_string(),
            skonto: None,
            buchungstext: None,
            postensperre: None,
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
            kost1_kostenstelle: None,
            kost2_kostenstelle: None,
            kost_menge: None,
            eu_ustid: None,
            eu_steuersatz: None,
            abweichende_versteuerungsart: "".to_string(),
            sachverhalt_l_l: "".to_string(),
            funktionsergänzung_l_l: "".to_string(),
            bu_49_hauptfunktiontyp: "".to_string(),
            bu_49_hauptfunktionsnummer: "".to_string(),
            bu_49_funktionsergänzung: "".to_string(),
            zusatzinformation_art1: None,
            zusatzinformation_inhalt1: None,
            zusatzinformation_art2: None,
            zusatzinformation_inhalt2: None,
            zusatzinformation_art3: None,
            zusatzinformation_inhalt3: None,
            zusatzinformation_art4: None,
            zusatzinformation_inhalt4: None,
            zusatzinformation_art5: None,
            zusatzinformation_inhalt5: None,
            zusatzinformation_art6: None,
            zusatzinformation_inhalt6: None,
            zusatzinformation_art7: None,
            zusatzinformation_inhalt7: None,
            zusatzinformation_art8: None,
            zusatzinformation_inhalt8: None,
            zusatzinformation_art9: None,
            zusatzinformation_inhalt9: None,
            zusatzinformation_art10: None,    
            zusatzinformation_inhalt10: None,
            zusatzinformation_art11: None,
            zusatzinformation_inhalt11: None,
            zusatzinformation_art12: None,
            zusatzinformation_inhalt12: None,
            zusatzinformation_art13: None,
            zusatzinformation_inhalt13: None,
            zusatzinformation_art14: None,
            zusatzinformation_inhalt14: None,
            zusatzinformation_art15: None,
            zusatzinformation_inhalt15: None,
            zusatzinformation_art16: None,
            zusatzinformation_inhalt16: None,
            zusatzinformation_art17: None,
            zusatzinformation_inhalt17: None,
            zusatzinformation_art18: None,
            zusatzinformation_inhalt18: None,
            zusatzinformation_art19: None,
            zusatzinformation_inhalt19: None,
            zusatzinformation_art20: None,
            zusatzinformation_inhalt20: None,
            stück: None,
            gewicht: None,
            zahlweise: None,
            forderungsart: None,
            forderungsjahr: None,
            veranlagungsjahr: None,
            zugeordnete_fälligkeit: None,
            skontotyp: None,
            auftragsnummer: None,
            buchungstyp: None,
            ust_schlüssel_anzahlung: None,
            eu_mitgliedstaat_anzahlung: None,
            sachverhalt_l_l_anzahlung: None,
            eu_steuersatz_anzahlung: None,
            erlöskonto_anzahlung: None,
            herkunft_kz: None,
            leerfeld: None,
            kost_datum: None,
            sepa_mandatsreferenz: None,
            skontosperre: None,
            gesellschaftername: None,
            beteiligtennummer: None,
            identifikationsnummer: None,
            zeichennummer: None,
            postensperre_bis: None,
            bezeichnung_so_bil_sachverhalt: None,
            kennzeichen_so_bil_buchung: None,
            festschreibung: None,
            leistungsdatum: None,
            datum_zuord_steuerperiode: None,
            fälligkeit: None,
            generalumkehr: None,
            steuersatz: None,
            land: None,
            abrechnungsreferenz: None,
            bvv_position: None,
            eu_ustid_ursprung: None,
            eu_steuersatz_ursprung: None,
        }
    }
}

impl Display for Buchung{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{umsatz};{soll_haben_kennzeichen};{wkz_umsatz};{kurs};{basis_umsatz};{wkz_basis_umsatz};{konto};{gegenkonto};{bu_schlüssel};{beleg_datum};{belegfeld1};{belegfeld2};{skonto};{buchungstext};{posten_sperre};{diverse_adressnummer};{geschäftspartner_bank};{sachverhalt};{zinssperre};{beleg_link};{beleg_info_art1};{beleg_info_inhalt1};{beleg_info_art2};{beleg_info_inhalt2};{beleg_info_art3};{beleg_info_inhalt3};{beleg_info_art4};{beleg_info_inhalt4};{beleg_info_art5};{beleg_info_inhalt5};{beleg_info_art6};{beleg_info_inhalt6};{beleg_info_art7};{beleg_info_inhalt7};{beleg_info_art8};{beleg_info_inhalt8};{kost1_kostenstelle};{kost2_kostenstelle};{kost_menge};{eu_ustid};{eu_steuersatz};{abweichende_versteuerungsart};{sachverhalt_l_l};{funktionsergänzung_l_l};{bu_49_hauptfunktiontyp};{bu_49_hauptfunktionsnummer};{bu_49_funktionsergänzung};{zusatzinformation_art1};{zusatzinformation_inhalt1};{zusatzinformation_art2};{zusatzinformation_inhalt2};{zusatzinformation_art3};{zusatzinformation_inhalt3};{zusatzinformation_art4};{zusatzinformation_inhalt4};{zusatzinformation_art5};{zusatzinformation_inhalt5};{zusatzinformation_art6};{zusatzinformation_inhalt6};{zusatzinformation_art7};{zusatzinformation_inhalt7};{zusatzinformation_art8};{zusatzinformation_inhalt8};{zusatzinformation_art9};{zusatzinformation_inhalt9};{zusatzinformation_art10};{zusatzinformation_inhalt10};{zusatzinformation_art11};{zusatzinformation_inhalt11};{zusatzinformation_art12};{zusatzinformation_inhalt12};{zusatzinformation_art13};{zusatzinformation_inhalt13};{zusatzinformation_art14};{zusatzinformation_inhalt14};{zusatzinformation_art15};{zusatzinformation_inhalt15};{zusatzinformation_art16};{zusatzinformation_inhalt16};{zusatzinformation_art17};{zusatzinformation_inhalt17};{zusatzinformation_art18};{zusatzinformation_inhalt18};{zusatzinformation_art19};{zusatzinformation_inhalt19};{zusatzinformation_art20};{zusatzinformation_inhalt20};{stück};{gewicht};{zahlweise};{forderungsart};{forderungsjahr};{veranlagungsjahr};{zugeordnete_fälligkeit};{skontotyp};{auftragsnummer};{buchungstyp};{ust_schlüssel_anzahlung};{eu_mitgliedstaat_anzahlung};{sachverhalt_l_l_anzahlung};{eu_steuersatz_anzahlung};{erlöskonto_anzahlung};{herkunft_kz};{leerfeld};{kost_datum};{sepa_mandatsreferenz};{skontosperre};{gesellschaftername};{beteiligtennummer};{identifikationsnummer};{zeichennummer};{postensperre_bis};{bezeichnung_so_bil_sachverhalt};{kennzeichen_so_bil_buchung};{festschreibung};{leistungsdatum};{datum_zuord_steuerperiode};{fälligkeit};{generalumkehr};{steuersatz};{land};{abrechnungsreferenz};{bvv_position};{eu_ustid_ursprung};{eu_steuersatz_ursprung}\n"#,
        umsatz = self.umsatz,
        soll_haben_kennzeichen = self.soll_haben_kennzeichen,
        wkz_umsatz = self.wkz_umsatz,
        kurs = match self.kurs {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        basis_umsatz = match self.basis_umsatz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        wkz_basis_umsatz = self.wkz_basis_umsatz,
        konto = self.konto,
        gegenkonto = self.gegenkonto,
        bu_schlüssel = match self.bu_schlüssel {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beleg_datum = self.beleg_datum,
        belegfeld1 = self.belegfeld1,
        belegfeld2 = self.belegfeld2,
        skonto = match self.skonto {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        buchungstext = match &self.buchungstext {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        posten_sperre = match self.postensperre {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
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
        kost1_kostenstelle = match &self.kost1_kostenstelle {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        kost2_kostenstelle = match &self.kost2_kostenstelle {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        kost_menge = match self.kost_menge {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_ustid = match &self.eu_ustid {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_steuersatz = match self.eu_steuersatz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        abweichende_versteuerungsart = self.abweichende_versteuerungsart,
        sachverhalt_l_l = self.sachverhalt_l_l,
        funktionsergänzung_l_l = self.funktionsergänzung_l_l,
        bu_49_hauptfunktiontyp = self.bu_49_hauptfunktiontyp,
        bu_49_hauptfunktionsnummer = self.bu_49_hauptfunktionsnummer,
        bu_49_funktionsergänzung = self.bu_49_funktionsergänzung,
        zusatzinformation_art1 = match &self.zusatzinformation_art1 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt1 = match &self.zusatzinformation_inhalt1 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art2 = match &self.zusatzinformation_art2 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt2 = match &self.zusatzinformation_inhalt2 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art3 = match &self.zusatzinformation_art3 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt3 = match &self.zusatzinformation_inhalt3 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art4 = match &self.zusatzinformation_art4 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt4 = match &self.zusatzinformation_inhalt4 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art5 = match &self.zusatzinformation_art5 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt5 = match &self.zusatzinformation_inhalt5 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art6 = match &self.zusatzinformation_art6 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt6 = match &self.zusatzinformation_inhalt6 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art7 = match &self.zusatzinformation_art7 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt7 = match &self.zusatzinformation_inhalt7 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art8 = match &self.zusatzinformation_art8 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt8 = match &self.zusatzinformation_inhalt8 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art9 = match &self.zusatzinformation_art9 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt9 = match &self.zusatzinformation_inhalt9 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art10 = match &self.zusatzinformation_art10 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt10 = match &self.zusatzinformation_inhalt10 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art11 = match &self.zusatzinformation_art11 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt11 = match &self.zusatzinformation_inhalt11 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art12 = match &self.zusatzinformation_art12 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt12 = match &self.zusatzinformation_inhalt12 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art13 = match &self.zusatzinformation_art13 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt13 = match &self.zusatzinformation_inhalt13 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art14 = match &self.zusatzinformation_art14 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt14 = match &self.zusatzinformation_inhalt14 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art15 = match &self.zusatzinformation_art15 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt15 = match &self.zusatzinformation_inhalt15 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art16 = match &self.zusatzinformation_art16 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt16 = match &self.zusatzinformation_inhalt16 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art17 = match &self.zusatzinformation_art17 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt17 = match &self.zusatzinformation_inhalt17 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art18 = match &self.zusatzinformation_art18 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt18 = match &self.zusatzinformation_inhalt18 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art19 = match &self.zusatzinformation_art19 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt19 = match &self.zusatzinformation_inhalt19 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_art20 = match &self.zusatzinformation_art20 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zusatzinformation_inhalt20 = match &self.zusatzinformation_inhalt20 {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        stück = match self.stück {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        gewicht = match self.gewicht {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zahlweise = match &self.zahlweise {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        forderungsart = match &self.forderungsart {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        forderungsjahr = match self.forderungsjahr {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        veranlagungsjahr = match self.veranlagungsjahr {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zugeordnete_fälligkeit = match &self.zugeordnete_fälligkeit {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        skontotyp = match &self.skontotyp {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        auftragsnummer = match &self.auftragsnummer {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        buchungstyp = match &self.buchungstyp {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        ust_schlüssel_anzahlung = match self.ust_schlüssel_anzahlung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_mitgliedstaat_anzahlung = match &self.eu_mitgliedstaat_anzahlung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        sachverhalt_l_l_anzahlung = match &self.sachverhalt_l_l_anzahlung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_steuersatz_anzahlung = match self.eu_steuersatz_anzahlung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        erlöskonto_anzahlung = match self.erlöskonto_anzahlung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        herkunft_kz = match &self.herkunft_kz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        leerfeld = match &self.leerfeld {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        kost_datum = match &self.kost_datum {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        sepa_mandatsreferenz = match &self.sepa_mandatsreferenz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        skontosperre = match self.skontosperre {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        gesellschaftername = match &self.gesellschaftername {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        beteiligtennummer = match &self.beteiligtennummer {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        identifikationsnummer = match &self.identifikationsnummer {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        zeichennummer = match &self.zeichennummer {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        postensperre_bis = match &self.postensperre_bis {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        bezeichnung_so_bil_sachverhalt = match &self.bezeichnung_so_bil_sachverhalt {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        kennzeichen_so_bil_buchung = match &self.kennzeichen_so_bil_buchung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        festschreibung = match &self.festschreibung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        leistungsdatum = match &self.leistungsdatum {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        datum_zuord_steuerperiode = match &self.datum_zuord_steuerperiode {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        fälligkeit = match &self.fälligkeit {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        generalumkehr = match &self.generalumkehr {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        steuersatz = match self.steuersatz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        land = match &self.land {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        abrechnungsreferenz = match &self.abrechnungsreferenz {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        bvv_position = match &self.bvv_position {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_ustid_ursprung = match &self.eu_ustid_ursprung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        eu_steuersatz_ursprung = match self.eu_steuersatz_ursprung {
            Some(val) => val.to_string(),
            None => "".to_string(),
        },
        )      
    }
}