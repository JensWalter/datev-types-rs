use serde::{Serialize, Deserialize};
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DebKred {
    pub konto: u32,
    pub name_adressattyp_unternehmen: String,
    pub unternehmensgegenstand: String,
    pub name_adressattyp_natürl_person: String,
    pub vorname_adressattyp_natürl_person: String,
    pub name_adressattyp_keine_angabe: String,
    pub adressattyp: String,
    pub kurzbezeichnung: String,
    pub eu_land: String,
    pub eu_ustid: String,
    pub anrede: String,
    pub titel: String,
    pub adelstitel: String,
    pub namensvorsatz: String,
    pub adressart: String,
    pub straße: String,
    pub postfach: String,
    pub postleitzahl: String,
    pub ort: String,
    pub land: String,
    pub versandzusatz: String,
    pub adresszusatz: String,
    pub abweichende_anrede: String,
    pub abw_zustellbezeichnung_1: String,
    pub abw_zustellbezeichnung_2: String,
    pub kennz_korrespondenzadresse: String,
    pub adresse_gültig_von: String,
    pub adresse_gültig_bis: String,
    pub telefon: String,
    pub bemerkung_telefon: String,
    pub telefon_gl: String,
    pub bemerkung_telefon_gl: String,
    pub e_mail: String,
    pub bemerkung_e_mail: String,
    pub internet: String,
    pub bemerkung_internet: String,
    pub fax: String,
    pub bemerkung_fax: String,
    pub sonstige: String,
    pub bemerkung_sonstige: String,
    pub bankleitzahl_1: String,
    pub bankbezeichnung_1: String,
    pub bank_kontonummer_1: String,
    pub länderkennzeichen_1: String,
    pub iban_nr_1: String,
    pub leerfeld1: String,
    pub swift_code_1: String,
    pub abw_kontoinhaber_1: String,
    pub kennz_hauptbankverb_1: String,
    pub bankverb_1_gültig_von: String,
    pub bankverb_1_gültig_bis: String,
    pub bankleitzahl_2: String,
    pub bankbezeichnung_2: String,
    pub bank_kontonummer_2: String,
    pub länderkennzeichen_2: String,
    pub iban_nr_2: String,
    pub leerfeld2: String,
    pub swift_code_2: String,
    pub abw_kontoinhaber_2: String,
    pub kennz_hauptbankverb_2: String,
    pub bankverb_2_gültig_von: String,
    pub bankverb_2_gültig_bis: String,
    pub bankleitzahl_3: String,
    pub bankbezeichnung_3: String,
    pub bank_kontonummer_3: String,
    pub länderkennzeichen_3: String,
    pub iban_nr_3: String,
    pub leerfeld3: String,
    pub swift_code_3: String,
    pub abw_kontoinhaber_3: String,
    pub kennz_hauptbankverb_3: String,
    pub bankverb_3_gültig_von: String,
    pub bankverb_3_gültig_bis: String,
    pub bankleitzahl_4: String,
    pub bankbezeichnung_4: String,
    pub bank_kontonummer_4: String,
    pub länderkennzeichen_4: String,
    pub iban_nr_4: String,
    pub leerfeld4: String,
    pub swift_code_4: String,
    pub abw_kontoinhaber_4: String,
    pub kennz_hauptbankverb_4: String,
    pub bankverb_4_gültig_von: String,
    pub bankverb_4_gültig_bis: String,
    pub bankleitzahl_5: String,
    pub bankbezeichnung_5: String,
    pub bank_kontonummer_5: String,
    pub länderkennzeichen_5: String,
    pub iban_nr_5: String,
    pub leerfeld5: String,
    pub swift_code_5: String,
    pub abw_kontoinhaber_5: String,
    pub kennz_hauptbankverb_5: String,
    pub bankverb_5_gültig_von: String,
    pub bankverb_5_gültig_bis: String,
    pub leerfeld6: String,
    pub briefanrede: String,
    pub grußformel: String,
    pub kunden_lief_nr: String,
    pub steuernummer: String,
    pub sprache: String,
    pub ansprechpartner: String,
    pub vertreter: String,
    pub sachbearbeiter: String,
    pub diverse_konto: String,
    pub ausgabeziel: String,
    pub währungssteuerung: String,
    pub kreditlimit_debitor: String,
    pub zahlungsbedingung: String,
    pub fälligkeit_in_tagen_debitor: String,
    pub skonto_in_prozent_debitor: String,
    pub kreditoren_ziel_1_tg: String,
    pub kreditoren_skonto_1_prozent: String,
    pub kreditoren_ziel_2_tg: String,
    pub kreditoren_skonto_2_prozent: String,
    pub kreditoren_ziel_3_brutto_tg: String,
    pub kreditoren_ziel_4_tg: String,
    pub kreditoren_skonto_4_prozent: String,
    pub kreditoren_ziel_5_tg: String,
    pub kreditoren_skonto_5_prozent: String,
    pub mahnung: String,
    pub kontoauszug: String,
    pub mahntext_1: String,
    pub mahntext_2: String,
    pub mahntext_3: String,
    pub kontoauszugstext: String,
    pub mahnlimit_betrag: String,
    pub mahnlimit_prozent: String,
    pub zinsberechnung: String,
    pub mahnzinssatz_1: String,
    pub mahnzinssatz_2: String,
    pub mahnzinssatz_3: String,
    pub lastschrift: String,
    pub leerfeld7: String,
    pub mandantenbank: String,
    pub zahlungsträger: String,
    pub indiv_feld_1: String,
    pub indiv_feld_2: String,
    pub indiv_feld_3: String,
    pub indiv_feld_4: String,
    pub indiv_feld_5: String,
    pub indiv_feld_6: String,
    pub indiv_feld_7: String,
    pub indiv_feld_8: String,
    pub indiv_feld_9: String,
    pub indiv_feld_10: String,
    pub indiv_feld_11: String,
    pub indiv_feld_12: String,
    pub indiv_feld_13: String,
    pub indiv_feld_14: String,
    pub indiv_feld_15: String,
    pub abweichende_anrede_rechnungsadresse: String,
    pub adressart_rechnungsadresse: String,
    pub straße_rechnungsadresse: String,
    pub postfach_rechnungsadresse: String,
    pub postleitzahl_rechnungsadresse: String,
    pub ort_rechnungsadresse: String,
    pub land_rechnungsadresse: String,
    pub versandzusatz_rechnungsadresse: String,
    pub adresszusatz_rechnungsadresse: String,
    pub abw_zustellbezeichnung_1_rechnungsadresse: String,
    pub abw_zustellbezeichnung_2_rechnungsadresse: String,
    pub adresse_gültig_von_rechnungsadresse: String,
    pub adresse_gültig_bis_rechnungsadresse: String,
    pub bankleitzahl_6: String,
    pub bankbezeichnung_6: String,
    pub bank_kontonummer_6: String,
    pub länderkennzeichen_6: String,
    pub iban_nr_6: String,
    pub leerfeld8: String,
    pub swift_code_6: String,
    pub abw_kontoinhaber_6: String,
    pub kennz_hauptbankverb_6: String,
    pub bankverb_6_gültig_von: String,
    pub bankverb_6_gültig_bis: String,
    pub bankleitzahl_7: String,
    pub bankbezeichnung_7: String,
    pub bank_kontonummer_7: String,
    pub länderkennzeichen_7: String,
    pub iban_nr_7: String,
    pub leerfeld9: String,
    pub swift_code_7: String,
    pub abw_kontoinhaber_7: String,
    pub kennz_hauptbankverb_7: String,
    pub bankverb_7_gültig_von: String,
    pub bankverb_7_gültig_bis: String,
    pub bankleitzahl_8: String,
    pub bankbezeichnung_8: String,
    pub bank_kontonummer_8: String,
    pub länderkennzeichen_8: String,
    pub iban_nr_8: String,
    pub leerfeld10: String,
    pub swift_code_8: String,
    pub abw_kontoinhaber_8: String,
    pub kennz_hauptbankverb_8: String,
    pub bankverb_8_gültig_von: String,
    pub bankverb_8_gültig_bis: String,
    pub bankleitzahl_9: String,
    pub bankbezeichnung_9: String,
    pub bank_kontonummer_9: String,
    pub länderkennzeichen_9: String,
    pub iban_nr_9: String,
    pub leerfeld11: String,
    pub swift_code_9: String,
    pub abw_kontoinhaber_9: String,
    pub kennz_hauptbankverb_9: String,
    pub bankverb_9_gültig_von: String,
    pub bankverb_9_gültig_bis: String,
    pub bankleitzahl_10: String,
    pub bankbezeichnung_10: String,
    pub bank_kontonummer_10: String,
    pub länderkennzeichen_10: String,
    pub iban_nr_10: String,
    pub leerfeld12: String,
    pub swift_code_10: String,
    pub abw_kontoinhaber_10: String,
    pub kennz_hauptbankverb_10: String,
    pub bankverb_10_gültig_von: String,
    pub bankverb_10_gültig_bis: String,
    pub nummer_fremdsystem: String,
    pub insolvent: String,
    pub sepa_mandatsreferenz_1: String,
    pub sepa_mandatsreferenz_2: String,
    pub sepa_mandatsreferenz_3: String,
    pub sepa_mandatsreferenz_4: String,
    pub sepa_mandatsreferenz_5: String,
    pub sepa_mandatsreferenz_6: String,
    pub sepa_mandatsreferenz_7: String,
    pub sepa_mandatsreferenz_8: String,
    pub sepa_mandatsreferenz_9: String,
    pub sepa_mandatsreferenz_10: String,
    pub verknüpftes_opos_konto: String,
    pub mahnsperre_bis: String,
    pub lastschriftsperre_bis: String,
    pub zahlungssperre_bis: String,
    pub gebührenberechnung: String,
    pub mahngebühr_1: String,
    pub mahngebühr_2: String,
    pub mahngebühr_3: String,
    pub pauschalenberechnung: String,
    pub verzugspauschale_1: String,
    pub verzugspauschale_2: String,
    pub verzugspauschale_3: String,
    pub alternativer_suchname: String,
    pub status: String,
    pub anschrift_manuell_geändert_korrespondenzadresse: String,
    pub anschrift_individuell_korrespondenzadresse: String,
    pub anschrift_manuell_geändert_rechnungsadresse: String,
    pub anschrift_individuell_rechnungsadresse: String,
    pub fristberechnung_bei_debitor: String,
    pub mahnfrist_1: String,
    pub mahnfrist_2: String,
    pub mahnfrist_3: String,
    pub letzte_frist: String,
}

impl Default for DebKred {
  fn default() -> Self {
    DebKred{
        konto: 0,
        name_adressattyp_unternehmen: "".to_string(),
        unternehmensgegenstand: "".to_string(),
        name_adressattyp_natürl_person: "".to_string(),
        vorname_adressattyp_natürl_person: "".to_string(),
        name_adressattyp_keine_angabe: "".to_string(),
        adressattyp: "".to_string(),
        kurzbezeichnung: "".to_string(),
        eu_land: "".to_string(),
        eu_ustid: "".to_string(),
        anrede: "".to_string(),
        titel: "".to_string(),
        adelstitel: "".to_string(),
        namensvorsatz: "".to_string(),
        adressart: "".to_string(),
        straße: "".to_string(),
        postfach: "".to_string(),
        postleitzahl: "".to_string(),
        ort: "".to_string(),
        land: "".to_string(),
        versandzusatz: "".to_string(),
        adresszusatz: "".to_string(),
        abweichende_anrede: "".to_string(),
        abw_zustellbezeichnung_1: "".to_string(),
        abw_zustellbezeichnung_2: "".to_string(),
        kennz_korrespondenzadresse: "".to_string(),
        adresse_gültig_von: "".to_string(),
        adresse_gültig_bis: "".to_string(),
        telefon: "".to_string(),
        bemerkung_telefon: "".to_string(),
        telefon_gl: "".to_string(),
        bemerkung_telefon_gl: "".to_string(),
        e_mail: "".to_string(),
        bemerkung_e_mail: "".to_string(),
        internet: "".to_string(),
        bemerkung_internet: "".to_string(),
        fax: "".to_string(),
        bemerkung_fax: "".to_string(),
        sonstige: "".to_string(),
        bemerkung_sonstige: "".to_string(),
        bankleitzahl_1: "".to_string(),
        bankbezeichnung_1: "".to_string(),
        bank_kontonummer_1: "".to_string(),
        länderkennzeichen_1: "".to_string(),
        iban_nr_1: "".to_string(),
        leerfeld1: "".to_string(),
        swift_code_1: "".to_string(),
        abw_kontoinhaber_1: "".to_string(),
        kennz_hauptbankverb_1: "".to_string(),
        bankverb_1_gültig_von: "".to_string(),
        bankverb_1_gültig_bis: "".to_string(),
        bankleitzahl_2: "".to_string(),
        bankbezeichnung_2: "".to_string(),
        bank_kontonummer_2: "".to_string(),
        länderkennzeichen_2: "".to_string(),
        iban_nr_2: "".to_string(),
        leerfeld2: "".to_string(),
        swift_code_2: "".to_string(),
        abw_kontoinhaber_2: "".to_string(),
        kennz_hauptbankverb_2: "".to_string(),
        bankverb_2_gültig_von: "".to_string(),
        bankverb_2_gültig_bis: "".to_string(),
        bankleitzahl_3: "".to_string(),
        bankbezeichnung_3: "".to_string(),
        bank_kontonummer_3: "".to_string(),
        länderkennzeichen_3: "".to_string(),
        iban_nr_3: "".to_string(),
        leerfeld3: "".to_string(),
        swift_code_3: "".to_string(),
        abw_kontoinhaber_3: "".to_string(),
        kennz_hauptbankverb_3: "".to_string(),
        bankverb_3_gültig_von: "".to_string(),
        bankverb_3_gültig_bis: "".to_string(),
        bankleitzahl_4: "".to_string(),
        bankbezeichnung_4: "".to_string(),
        bank_kontonummer_4: "".to_string(),
        länderkennzeichen_4: "".to_string(),
        iban_nr_4: "".to_string(),
        leerfeld4: "".to_string(),
        swift_code_4: "".to_string(),
        abw_kontoinhaber_4: "".to_string(),
        kennz_hauptbankverb_4: "".to_string(),
        bankverb_4_gültig_von: "".to_string(),
        bankverb_4_gültig_bis: "".to_string(),
        bankleitzahl_5: "".to_string(),
        bankbezeichnung_5: "".to_string(),
        bank_kontonummer_5: "".to_string(),
        länderkennzeichen_5: "".to_string(),
        iban_nr_5: "".to_string(),
        leerfeld5: "".to_string(),
        swift_code_5: "".to_string(),
        abw_kontoinhaber_5: "".to_string(),
        kennz_hauptbankverb_5: "".to_string(),
        bankverb_5_gültig_von: "".to_string(),
        bankverb_5_gültig_bis: "".to_string(),
        leerfeld6: "".to_string(),
        briefanrede: "".to_string(),
        grußformel: "".to_string(),
        kunden_lief_nr: "".to_string(),
        steuernummer: "".to_string(),
        sprache: "".to_string(),
        ansprechpartner: "".to_string(),
        vertreter: "".to_string(),
        sachbearbeiter: "".to_string(),
        diverse_konto: "".to_string(),
        ausgabeziel: "".to_string(),
        währungssteuerung: "".to_string(),
        kreditlimit_debitor: "".to_string(),
        zahlungsbedingung: "".to_string(),
        fälligkeit_in_tagen_debitor: "".to_string(),
        skonto_in_prozent_debitor: "".to_string(),
        kreditoren_ziel_1_tg: "".to_string(),
        kreditoren_skonto_1_prozent: "".to_string(),
        kreditoren_ziel_2_tg: "".to_string(),
        kreditoren_skonto_2_prozent: "".to_string(),
        kreditoren_ziel_3_brutto_tg: "".to_string(),
        kreditoren_ziel_4_tg: "".to_string(),
        kreditoren_skonto_4_prozent: "".to_string(),
        kreditoren_ziel_5_tg: "".to_string(),
        kreditoren_skonto_5_prozent: "".to_string(),
        mahnung: "".to_string(),
        kontoauszug: "".to_string(),
        mahntext_1: "".to_string(),
        mahntext_2: "".to_string(),
        mahntext_3: "".to_string(),
        kontoauszugstext: "".to_string(),
        mahnlimit_betrag: "".to_string(),
        mahnlimit_prozent: "".to_string(),
        zinsberechnung: "".to_string(),
        mahnzinssatz_1: "".to_string(),
        mahnzinssatz_2: "".to_string(),
        mahnzinssatz_3: "".to_string(),
        lastschrift: "".to_string(),
        leerfeld7: "".to_string(),
        mandantenbank: "".to_string(),
        zahlungsträger: "".to_string(),
        indiv_feld_1: "".to_string(),
        indiv_feld_2: "".to_string(),
        indiv_feld_3: "".to_string(),
        indiv_feld_4: "".to_string(),
        indiv_feld_5: "".to_string(),
        indiv_feld_6: "".to_string(),
        indiv_feld_7: "".to_string(),
        indiv_feld_8: "".to_string(),
        indiv_feld_9: "".to_string(),
        indiv_feld_10: "".to_string(),
        indiv_feld_11: "".to_string(),
        indiv_feld_12: "".to_string(),
        indiv_feld_13: "".to_string(),
        indiv_feld_14: "".to_string(),
        indiv_feld_15: "".to_string(),
        abweichende_anrede_rechnungsadresse: "".to_string(),
        adressart_rechnungsadresse: "".to_string(),
        straße_rechnungsadresse: "".to_string(),
        postfach_rechnungsadresse: "".to_string(),
        postleitzahl_rechnungsadresse: "".to_string(),
        ort_rechnungsadresse: "".to_string(),
        land_rechnungsadresse: "".to_string(),
        versandzusatz_rechnungsadresse: "".to_string(),
        adresszusatz_rechnungsadresse: "".to_string(),
        abw_zustellbezeichnung_1_rechnungsadresse: "".to_string(),
        abw_zustellbezeichnung_2_rechnungsadresse: "".to_string(),
        adresse_gültig_von_rechnungsadresse: "".to_string(),
        adresse_gültig_bis_rechnungsadresse: "".to_string(),
        bankleitzahl_6: "".to_string(),
        bankbezeichnung_6: "".to_string(),
        bank_kontonummer_6: "".to_string(),
        länderkennzeichen_6: "".to_string(),
        iban_nr_6: "".to_string(),
        leerfeld8: "".to_string(),
        swift_code_6: "".to_string(),
        abw_kontoinhaber_6: "".to_string(),
        kennz_hauptbankverb_6: "".to_string(),
        bankverb_6_gültig_von: "".to_string(),
        bankverb_6_gültig_bis: "".to_string(),
        bankleitzahl_7: "".to_string(),
        bankbezeichnung_7: "".to_string(),
        bank_kontonummer_7: "".to_string(),
        länderkennzeichen_7: "".to_string(),
        iban_nr_7: "".to_string(),
        leerfeld9: "".to_string(),
        swift_code_7: "".to_string(),
        abw_kontoinhaber_7: "".to_string(),
        kennz_hauptbankverb_7: "".to_string(),
        bankverb_7_gültig_von: "".to_string(),
        bankverb_7_gültig_bis: "".to_string(),
        bankleitzahl_8: "".to_string(),
        bankbezeichnung_8: "".to_string(),
        bank_kontonummer_8: "".to_string(),
        länderkennzeichen_8: "".to_string(),
        iban_nr_8: "".to_string(),
        leerfeld10: "".to_string(),
        swift_code_8: "".to_string(),
        abw_kontoinhaber_8: "".to_string(),
        kennz_hauptbankverb_8: "".to_string(),
        bankverb_8_gültig_von: "".to_string(),
        bankverb_8_gültig_bis: "".to_string(),
        bankleitzahl_9: "".to_string(),
        bankbezeichnung_9: "".to_string(),
        bank_kontonummer_9: "".to_string(),
        länderkennzeichen_9: "".to_string(),
        iban_nr_9: "".to_string(),
        leerfeld11: "".to_string(),
        swift_code_9: "".to_string(),
        abw_kontoinhaber_9: "".to_string(),
        kennz_hauptbankverb_9: "".to_string(),
        bankverb_9_gültig_von: "".to_string(),
        bankverb_9_gültig_bis: "".to_string(),
        bankleitzahl_10: "".to_string(),
        bankbezeichnung_10: "".to_string(),
        bank_kontonummer_10: "".to_string(),
        länderkennzeichen_10: "".to_string(),
        iban_nr_10: "".to_string(),
        leerfeld12: "".to_string(),
        swift_code_10: "".to_string(),
        abw_kontoinhaber_10: "".to_string(),
        kennz_hauptbankverb_10: "".to_string(),
        bankverb_10_gültig_von: "".to_string(),
        bankverb_10_gültig_bis: "".to_string(),
        nummer_fremdsystem: "".to_string(),
        insolvent: "".to_string(),
        sepa_mandatsreferenz_1: "".to_string(),
        sepa_mandatsreferenz_2: "".to_string(),
        sepa_mandatsreferenz_3: "".to_string(),
        sepa_mandatsreferenz_4: "".to_string(),
        sepa_mandatsreferenz_5: "".to_string(),
        sepa_mandatsreferenz_6: "".to_string(),
        sepa_mandatsreferenz_7: "".to_string(),
        sepa_mandatsreferenz_8: "".to_string(),
        sepa_mandatsreferenz_9: "".to_string(),
        sepa_mandatsreferenz_10: "".to_string(),
        verknüpftes_opos_konto: "".to_string(),
        mahnsperre_bis: "".to_string(),
        lastschriftsperre_bis: "".to_string(),
        zahlungssperre_bis: "".to_string(),
        gebührenberechnung: "".to_string(),
        mahngebühr_1: "".to_string(),
        mahngebühr_2: "".to_string(),
        mahngebühr_3: "".to_string(),
        pauschalenberechnung: "".to_string(),
        verzugspauschale_1: "".to_string(),
        verzugspauschale_2: "".to_string(),
        verzugspauschale_3: "".to_string(),
        alternativer_suchname: "".to_string(),
        status: "".to_string(),
        anschrift_manuell_geändert_korrespondenzadresse: "".to_string(),
        anschrift_individuell_korrespondenzadresse: "".to_string(),
        anschrift_manuell_geändert_rechnungsadresse: "".to_string(),
        anschrift_individuell_rechnungsadresse: "".to_string(),
        fristberechnung_bei_debitor: "".to_string(),
        mahnfrist_1: "".to_string(),
        mahnfrist_2: "".to_string(),
        mahnfrist_3: "".to_string(),
        letzte_frist: "".to_string(),
    }
  }
}

impl Display for DebKred{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, r#"{konto};{name_adressattyp_unternehmen};{unternehmensgegenstand};{name_adressattyp_natürl_person};{vorname_adressattyp_natürl_person};{name_adressattyp_keine_angabe};{adressattyp};{kurzbezeichnung};{eu_land};{eu_ustid};{anrede};{titel};{adelstitel};{namensvorsatz};{adressart};{straße};{postfach};{postleitzahl};{ort};{land};{versandzusatz};{adresszusatz};{abweichende_anrede};{abw_zustellbezeichnung_1};{abw_zustellbezeichnung_2};{kennz_korrespondenzadresse};{adresse_gültig_von};{adresse_gültig_bis};{telefon};{bemerkung_telefon};{telefon_gl};{bemerkung_telefon_gl};{e_mail};{bemerkung_e_mail};{internet};{bemerkung_internet};{fax};{bemerkung_fax};{sonstige};{bemerkung_sonstige};{bankleitzahl_1};{bankbezeichnung_1};{bank_kontonummer_1};{länderkennzeichen_1};{iban_nr_1};{leerfeld1};{swift_code_1};{abw_kontoinhaber_1};{kennz_hauptbankverb_1};{bankverb_1_gültig_von};{bankverb_1_gültig_bis};{bankleitzahl_2};{bankbezeichnung_2};{bank_kontonummer_2};{länderkennzeichen_2};{iban_nr_2};{leerfeld2};{swift_code_2};{abw_kontoinhaber_2};{kennz_hauptbankverb_2};{bankverb_2_gültig_von};{bankverb_2_gültig_bis};{bankleitzahl_3};{bankbezeichnung_3};{bank_kontonummer_3};{länderkennzeichen_3};{iban_nr_3};{leerfeld3};{swift_code_3};{abw_kontoinhaber_3};{kennz_hauptbankverb_3};{bankverb_3_gültig_von};{bankverb_3_gültig_bis};{bankleitzahl_4};{bankbezeichnung_4};{bank_kontonummer_4};{länderkennzeichen_4};{iban_nr_4};{leerfeld4};{swift_code_4};{abw_kontoinhaber_4};{kennz_hauptbankverb_4};{bankverb_4_gültig_von};{bankverb_4_gültig_bis};{bankleitzahl_5};{bankbezeichnung_5};{bank_kontonummer_5};{länderkennzeichen_5};{iban_nr_5};{leerfeld5};{swift_code_5};{abw_kontoinhaber_5};{kennz_hauptbankverb_5};{bankverb_5_gültig_von};{bankverb_5_gültig_bis};{leerfeld6};{briefanrede};{grußformel};{kunden_lief_nr};{steuernummer};{sprache};{ansprechpartner};{vertreter};{sachbearbeiter};{diverse_konto};{ausgabeziel};{währungssteuerung};{kreditlimit_debitor};{zahlungsbedingung};{fälligkeit_in_tagen_debitor};{skonto_in_prozent_debitor};{kreditoren_ziel_1_tg};{kreditoren_skonto_1_prozent};{kreditoren_ziel_2_tg};{kreditoren_skonto_2_prozent};{kreditoren_ziel_3_brutto_tg};{kreditoren_ziel_4_tg};{kreditoren_skonto_4_prozent};{kreditoren_ziel_5_tg};{kreditoren_skonto_5_prozent};{mahnung};{kontoauszug};{mahntext_1};{mahntext_2};{mahntext_3};{kontoauszugstext};{mahnlimit_betrag};{mahnlimit_prozent};{zinsberechnung};{mahnzinssatz_1};{mahnzinssatz_2};{mahnzinssatz_3};{lastschrift};{leerfeld7};{mandantenbank};{zahlungsträger};{indiv_feld_1};{indiv_feld_2};{indiv_feld_3};{indiv_feld_4};{indiv_feld_5};{indiv_feld_6};{indiv_feld_7};{indiv_feld_8};{indiv_feld_9};{indiv_feld_10};{indiv_feld_11};{indiv_feld_12};{indiv_feld_13};{indiv_feld_14};{indiv_feld_15};{abweichende_anrede_rechnungsadresse};{adressart_rechnungsadresse};{straße_rechnungsadresse};{postfach_rechnungsadresse};{postleitzahl_rechnungsadresse};{ort_rechnungsadresse};{land_rechnungsadresse};{versandzusatz_rechnungsadresse};{adresszusatz_rechnungsadresse};{abw_zustellbezeichnung_1_rechnungsadresse};{abw_zustellbezeichnung_2_rechnungsadresse};{adresse_gültig_von_rechnungsadresse};{adresse_gültig_bis_rechnungsadresse};{bankleitzahl_6};{bankbezeichnung_6};{bank_kontonummer_6};{länderkennzeichen_6};{iban_nr_6};{leerfeld8};{swift_code_6};{abw_kontoinhaber_6};{kennz_hauptbankverb_6};{bankverb_6_gültig_von};{bankverb_6_gültig_bis};{bankleitzahl_7};{bankbezeichnung_7};{bank_kontonummer_7};{länderkennzeichen_7};{iban_nr_7};{leerfeld9};{swift_code_7};{abw_kontoinhaber_7};{kennz_hauptbankverb_7};{bankverb_7_gültig_von};{bankverb_7_gültig_bis};{bankleitzahl_8};{bankbezeichnung_8};{bank_kontonummer_8};{länderkennzeichen_8};{iban_nr_8};{leerfeld10};{swift_code_8};{abw_kontoinhaber_8};{kennz_hauptbankverb_8};{bankverb_8_gültig_von};{bankverb_8_gültig_bis};{bankleitzahl_9};{bankbezeichnung_9};{bank_kontonummer_9};{länderkennzeichen_9};{iban_nr_9};{leerfeld11};{swift_code_9};{abw_kontoinhaber_9};{kennz_hauptbankverb_9};{bankverb_9_gültig_von};{bankverb_9_gültig_bis};{bankleitzahl_10};{bankbezeichnung_10};{bank_kontonummer_10};{länderkennzeichen_10};{iban_nr_10};{leerfeld12};{swift_code_10};{abw_kontoinhaber_10};{kennz_hauptbankverb_10};{bankverb_10_gültig_von};{bankverb_10_gültig_bis};{nummer_fremdsystem};{insolvent};{sepa_mandatsreferenz_1};{sepa_mandatsreferenz_2};{sepa_mandatsreferenz_3};{sepa_mandatsreferenz_4};{sepa_mandatsreferenz_5};{sepa_mandatsreferenz_6};{sepa_mandatsreferenz_7};{sepa_mandatsreferenz_8};{sepa_mandatsreferenz_9};{sepa_mandatsreferenz_10};{verknüpftes_opos_konto};{mahnsperre_bis};{lastschriftsperre_bis};{zahlungssperre_bis};{gebührenberechnung};{mahngebühr_1};{mahngebühr_2};{mahngebühr_3};{pauschalenberechnung};{verzugspauschale_1};{verzugspauschale_2};{verzugspauschale_3};{alternativer_suchname};{status};{anschrift_manuell_geändert_korrespondenzadresse};{anschrift_individuell_korrespondenzadresse};{anschrift_manuell_geändert_rechnungsadresse};{anschrift_individuell_rechnungsadresse};{fristberechnung_bei_debitor};{mahnfrist_1};{mahnfrist_2};{mahnfrist_3};{letzte_frist}"#,
          konto = self.konto,
          name_adressattyp_unternehmen = self.name_adressattyp_unternehmen,
          unternehmensgegenstand = self.unternehmensgegenstand,
          name_adressattyp_natürl_person = self.name_adressattyp_natürl_person,
          vorname_adressattyp_natürl_person = self.vorname_adressattyp_natürl_person,
          name_adressattyp_keine_angabe = self.name_adressattyp_keine_angabe,
          adressattyp = self.adressattyp,
          kurzbezeichnung = self.kurzbezeichnung,
          eu_land = self.eu_land,
          eu_ustid = self.eu_ustid,
          anrede = self.anrede,
          titel = self.titel,
          adelstitel = self.adelstitel,
          namensvorsatz = self.namensvorsatz,
          adressart = self.adressart,
          straße = self.straße,
          postfach = self.postfach,
          postleitzahl = self.postleitzahl,
          ort = self.ort,
          land = self.land,
          versandzusatz = self.versandzusatz,
          adresszusatz = self.adresszusatz,
          abweichende_anrede = self.abweichende_anrede,
          abw_zustellbezeichnung_1 = self.abw_zustellbezeichnung_1,
          abw_zustellbezeichnung_2 = self.abw_zustellbezeichnung_2,
          kennz_korrespondenzadresse = self.kennz_korrespondenzadresse,
          adresse_gültig_von = self.adresse_gültig_von,
          adresse_gültig_bis = self.adresse_gültig_bis,
          telefon = self.telefon,
          bemerkung_telefon = self.bemerkung_telefon,
          telefon_gl = self.telefon_gl,
          bemerkung_telefon_gl = self.bemerkung_telefon_gl,
          e_mail = self.e_mail,
          bemerkung_e_mail = self.bemerkung_e_mail,
          internet = self.internet,
          bemerkung_internet = self.bemerkung_internet,
          fax = self.fax,
          bemerkung_fax = self.bemerkung_fax,
          sonstige = self.sonstige,
          bemerkung_sonstige = self.bemerkung_sonstige,
          bankleitzahl_1 = self.bankleitzahl_1,
          bankbezeichnung_1 = self.bankbezeichnung_1,
          bank_kontonummer_1 = self.bank_kontonummer_1,
          länderkennzeichen_1 = self.länderkennzeichen_1,
          iban_nr_1 = self.iban_nr_1,
          leerfeld1 = self.leerfeld1,
          swift_code_1 = self.swift_code_1,
          abw_kontoinhaber_1 = self.abw_kontoinhaber_1,
          kennz_hauptbankverb_1 = self.kennz_hauptbankverb_1,
          bankverb_1_gültig_von = self.bankverb_1_gültig_von,
          bankverb_1_gültig_bis = self.bankverb_1_gültig_bis,
          bankleitzahl_2 = self.bankleitzahl_2,
          bankbezeichnung_2 = self.bankbezeichnung_2,
          bank_kontonummer_2 = self.bank_kontonummer_2,
          länderkennzeichen_2 = self.länderkennzeichen_2,
          iban_nr_2 = self.iban_nr_2,
          leerfeld2 = self.leerfeld2,
          swift_code_2 = self.swift_code_2,
          abw_kontoinhaber_2 = self.abw_kontoinhaber_2,
          kennz_hauptbankverb_2 = self.kennz_hauptbankverb_2,
          bankverb_2_gültig_von = self.bankverb_2_gültig_von,
          bankverb_2_gültig_bis = self.bankverb_2_gültig_bis,
          bankleitzahl_3 = self.bankleitzahl_3,
          bankbezeichnung_3 = self.bankbezeichnung_3,
          bank_kontonummer_3 = self.bank_kontonummer_3,
          länderkennzeichen_3 = self.länderkennzeichen_3,
          iban_nr_3 = self.iban_nr_3,
          leerfeld3 = self.leerfeld3,
          swift_code_3 = self.swift_code_3,
          abw_kontoinhaber_3 = self.abw_kontoinhaber_3,
          kennz_hauptbankverb_3 = self.kennz_hauptbankverb_3,
          bankverb_3_gültig_von = self.bankverb_3_gültig_von,
          bankverb_3_gültig_bis = self.bankverb_3_gültig_bis,
          bankleitzahl_4 = self.bankleitzahl_4,
          bankbezeichnung_4 = self.bankbezeichnung_4,
          bank_kontonummer_4 = self.bank_kontonummer_4,
          länderkennzeichen_4 = self.länderkennzeichen_4,
          iban_nr_4 = self.iban_nr_4,
          leerfeld4 = self.leerfeld4,
          swift_code_4 = self.swift_code_4,
          abw_kontoinhaber_4 = self.abw_kontoinhaber_4,
          kennz_hauptbankverb_4 = self.kennz_hauptbankverb_4,
          bankverb_4_gültig_von = self.bankverb_4_gültig_von,
          bankverb_4_gültig_bis = self.bankverb_4_gültig_bis,
          bankleitzahl_5 = self.bankleitzahl_5,
          bankbezeichnung_5 = self.bankbezeichnung_5,
          bank_kontonummer_5 = self.bank_kontonummer_5,
          länderkennzeichen_5 = self.länderkennzeichen_5,
          iban_nr_5 = self.iban_nr_5,
          leerfeld5 = self.leerfeld5,
          swift_code_5 = self.swift_code_5,
          abw_kontoinhaber_5 = self.abw_kontoinhaber_5,
          kennz_hauptbankverb_5 = self.kennz_hauptbankverb_5,
          bankverb_5_gültig_von = self.bankverb_5_gültig_von,
          bankverb_5_gültig_bis = self.bankverb_5_gültig_bis,
          leerfeld6 = self.leerfeld6,
          briefanrede = self.briefanrede,
          grußformel = self.grußformel,
          kunden_lief_nr = self.kunden_lief_nr,
          steuernummer = self.steuernummer,
          sprache = self.sprache,
          ansprechpartner = self.ansprechpartner,
          vertreter = self.vertreter,
          sachbearbeiter = self.sachbearbeiter,
          diverse_konto = self.diverse_konto,
          ausgabeziel = self.ausgabeziel,
          währungssteuerung = self.währungssteuerung,
          kreditlimit_debitor = self.kreditlimit_debitor,
          zahlungsbedingung = self.zahlungsbedingung,
          fälligkeit_in_tagen_debitor = self.fälligkeit_in_tagen_debitor,
          skonto_in_prozent_debitor = self.skonto_in_prozent_debitor,
          kreditoren_ziel_1_tg = self.kreditoren_ziel_1_tg,
          kreditoren_skonto_1_prozent = self.kreditoren_skonto_1_prozent,
          kreditoren_ziel_2_tg = self.kreditoren_ziel_2_tg,
          kreditoren_skonto_2_prozent = self.kreditoren_skonto_2_prozent,
          kreditoren_ziel_3_brutto_tg = self.kreditoren_ziel_3_brutto_tg,
          kreditoren_ziel_4_tg = self.kreditoren_ziel_4_tg,
          kreditoren_skonto_4_prozent = self.kreditoren_skonto_4_prozent,
          kreditoren_ziel_5_tg = self.kreditoren_ziel_5_tg,
          kreditoren_skonto_5_prozent = self.kreditoren_skonto_5_prozent,
          mahnung = self.mahnung,
          kontoauszug = self.kontoauszug,
          mahntext_1 = self.mahntext_1,
          mahntext_2 = self.mahntext_2,
          mahntext_3 = self.mahntext_3,
          kontoauszugstext = self.kontoauszugstext,
          mahnlimit_betrag = self.mahnlimit_betrag,
          mahnlimit_prozent = self.mahnlimit_prozent,
          zinsberechnung = self.zinsberechnung,
          mahnzinssatz_1 = self.mahnzinssatz_1,
          mahnzinssatz_2 = self.mahnzinssatz_2,
          mahnzinssatz_3 = self.mahnzinssatz_3,
          lastschrift = self.lastschrift,
          leerfeld7 = self.leerfeld7,
          mandantenbank = self.mandantenbank,
          zahlungsträger = self.zahlungsträger,
          indiv_feld_1 = self.indiv_feld_1,
          indiv_feld_2 = self.indiv_feld_2,
          indiv_feld_3 = self.indiv_feld_3,
          indiv_feld_4 = self.indiv_feld_4,
          indiv_feld_5 = self.indiv_feld_5,
          indiv_feld_6 = self.indiv_feld_6,
          indiv_feld_7 = self.indiv_feld_7,
          indiv_feld_8 = self.indiv_feld_8,
          indiv_feld_9 = self.indiv_feld_9,
          indiv_feld_10 = self.indiv_feld_10,
          indiv_feld_11 = self.indiv_feld_11,
          indiv_feld_12 = self.indiv_feld_12,
          indiv_feld_13 = self.indiv_feld_13,
          indiv_feld_14 = self.indiv_feld_14,
          indiv_feld_15 = self.indiv_feld_15,
          abweichende_anrede_rechnungsadresse = self.abweichende_anrede_rechnungsadresse,
          adressart_rechnungsadresse = self.adressart_rechnungsadresse,
          straße_rechnungsadresse = self.straße_rechnungsadresse,
          postfach_rechnungsadresse = self.postfach_rechnungsadresse,
          postleitzahl_rechnungsadresse = self.postleitzahl_rechnungsadresse,
          ort_rechnungsadresse = self.ort_rechnungsadresse,
          land_rechnungsadresse = self.land_rechnungsadresse,
          versandzusatz_rechnungsadresse = self.versandzusatz_rechnungsadresse,
          adresszusatz_rechnungsadresse = self.adresszusatz_rechnungsadresse,
          abw_zustellbezeichnung_1_rechnungsadresse = self.abw_zustellbezeichnung_1_rechnungsadresse,
          abw_zustellbezeichnung_2_rechnungsadresse = self.abw_zustellbezeichnung_2_rechnungsadresse,
          adresse_gültig_von_rechnungsadresse = self.adresse_gültig_von_rechnungsadresse,
          adresse_gültig_bis_rechnungsadresse = self.adresse_gültig_bis_rechnungsadresse,
          bankleitzahl_6 = self.bankleitzahl_6,
          bankbezeichnung_6 = self.bankbezeichnung_6,
          bank_kontonummer_6 = self.bank_kontonummer_6,
          länderkennzeichen_6 = self.länderkennzeichen_6,
          iban_nr_6 = self.iban_nr_6,
          leerfeld8 = self.leerfeld8,
          swift_code_6 = self.swift_code_6,
          abw_kontoinhaber_6 = self.abw_kontoinhaber_6,
          kennz_hauptbankverb_6 = self.kennz_hauptbankverb_6,
          bankverb_6_gültig_von = self.bankverb_6_gültig_von,
          bankverb_6_gültig_bis = self.bankverb_6_gültig_bis,
          bankleitzahl_7 = self.bankleitzahl_7,
          bankbezeichnung_7 = self.bankbezeichnung_7,
          bank_kontonummer_7 = self.bank_kontonummer_7,
          länderkennzeichen_7 = self.länderkennzeichen_7,
          iban_nr_7 = self.iban_nr_7,
          leerfeld9 = self.leerfeld9,
          swift_code_7 = self.swift_code_7,
          abw_kontoinhaber_7 = self.abw_kontoinhaber_7,
          kennz_hauptbankverb_7 = self.kennz_hauptbankverb_7,
          bankverb_7_gültig_von = self.bankverb_7_gültig_von,
          bankverb_7_gültig_bis = self.bankverb_7_gültig_bis,
          bankleitzahl_8 = self.bankleitzahl_8,
          bankbezeichnung_8 = self.bankbezeichnung_8,
          bank_kontonummer_8 = self.bank_kontonummer_8,
          länderkennzeichen_8 = self.länderkennzeichen_8,
          iban_nr_8 = self.iban_nr_8,
          leerfeld10 = self.leerfeld10,
          swift_code_8 = self.swift_code_8,
          abw_kontoinhaber_8 = self.abw_kontoinhaber_8,
          kennz_hauptbankverb_8 = self.kennz_hauptbankverb_8,
          bankverb_8_gültig_von = self.bankverb_8_gültig_von,
          bankverb_8_gültig_bis = self.bankverb_8_gültig_bis,
          bankleitzahl_9 = self.bankleitzahl_9,
          bankbezeichnung_9 = self.bankbezeichnung_9,
          bank_kontonummer_9 = self.bank_kontonummer_9,
          länderkennzeichen_9 = self.länderkennzeichen_9,
          iban_nr_9 = self.iban_nr_9,
          leerfeld11  = self.leerfeld11,
          swift_code_9 = self.swift_code_9,
          abw_kontoinhaber_9 = self.abw_kontoinhaber_9,
          kennz_hauptbankverb_9 = self.kennz_hauptbankverb_9,
          bankverb_9_gültig_von = self.bankverb_9_gültig_von,
          bankverb_9_gültig_bis = self.bankverb_9_gültig_bis,
          bankleitzahl_10 = self.bankleitzahl_10,
          bankbezeichnung_10 = self.bankbezeichnung_10,
          bank_kontonummer_10 = self.bank_kontonummer_10,
          länderkennzeichen_10 = self.länderkennzeichen_10,
          iban_nr_10 = self.iban_nr_10,
          leerfeld12 = self.leerfeld12,
          swift_code_10 = self.swift_code_10,
          abw_kontoinhaber_10 = self.abw_kontoinhaber_10,
          kennz_hauptbankverb_10 = self.kennz_hauptbankverb_10,
          bankverb_10_gültig_von = self.bankverb_10_gültig_von,
          bankverb_10_gültig_bis = self.bankverb_10_gültig_bis,
          nummer_fremdsystem = self.nummer_fremdsystem,
          insolvent = self.insolvent,
          sepa_mandatsreferenz_1 = self.sepa_mandatsreferenz_1,
          sepa_mandatsreferenz_2 = self.sepa_mandatsreferenz_2,
          sepa_mandatsreferenz_3 = self.sepa_mandatsreferenz_3,
          sepa_mandatsreferenz_4 = self.sepa_mandatsreferenz_4,
          sepa_mandatsreferenz_5 = self.sepa_mandatsreferenz_5,
          sepa_mandatsreferenz_6 = self.sepa_mandatsreferenz_6,
          sepa_mandatsreferenz_7  = self.sepa_mandatsreferenz_7,
          sepa_mandatsreferenz_8 = self.sepa_mandatsreferenz_8,
          sepa_mandatsreferenz_9 = self.sepa_mandatsreferenz_9,
          sepa_mandatsreferenz_10 = self.sepa_mandatsreferenz_10,
          verknüpftes_opos_konto = self.verknüpftes_opos_konto,
          mahnsperre_bis = self.mahnsperre_bis,
          lastschriftsperre_bis = self.lastschriftsperre_bis,
          zahlungssperre_bis = self.zahlungssperre_bis,
          gebührenberechnung = self.gebührenberechnung,
          mahngebühr_1 = self.mahngebühr_1,
          mahngebühr_2 = self.mahngebühr_2,
          mahngebühr_3 = self.mahngebühr_3,
          pauschalenberechnung  = self.pauschalenberechnung,
          verzugspauschale_1 = self.verzugspauschale_1,
          verzugspauschale_2 = self.verzugspauschale_2,
          verzugspauschale_3 = self.verzugspauschale_3,
          alternativer_suchname = self.alternativer_suchname,
          status = self.status,
          anschrift_manuell_geändert_korrespondenzadresse = self.anschrift_manuell_geändert_korrespondenzadresse,
          anschrift_individuell_korrespondenzadresse = self.anschrift_individuell_korrespondenzadresse,
          anschrift_manuell_geändert_rechnungsadresse = self.anschrift_manuell_geändert_rechnungsadresse,
          anschrift_individuell_rechnungsadresse = self.anschrift_individuell_rechnungsadresse,
          fristberechnung_bei_debitor = self.fristberechnung_bei_debitor,
          mahnfrist_1 = self.mahnfrist_1,
          mahnfrist_2 = self.mahnfrist_2,
          mahnfrist_3 = self.mahnfrist_3,
          letzte_frist = self.letzte_frist
      )
  }
}

impl TryFrom<&str> for DebKred {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
      let mut rdr = csv::ReaderBuilder::new().delimiter(b';').flexible(true)
          .has_headers(false).from_reader(value.as_bytes());

      let mut iter = rdr.records();
      if let Some(result) = iter.next() {
          let record = result.unwrap();
          let mut debcred = DebKred::default();
          //add values
          if let Some(val) = record.get(0) {
              debcred.konto = val.parse().unwrap();
          }
          debcred.name_adressattyp_unternehmen = record.get(1).unwrap().to_string();
          debcred.unternehmensgegenstand = record.get(2).unwrap().to_string();
          debcred.name_adressattyp_natürl_person = record.get(3).unwrap().to_string();
          debcred.vorname_adressattyp_natürl_person = record.get(4).unwrap().to_string();
          debcred.name_adressattyp_keine_angabe = record.get(5).unwrap().to_string();
          debcred.adressattyp = record.get(6).unwrap().to_string();
          debcred.kurzbezeichnung = record.get(7).unwrap().to_string();
          debcred.eu_land = record.get(8).unwrap().to_string();
          debcred.eu_ustid = record.get(9).unwrap().to_string();
          debcred.anrede = record.get(10).unwrap().to_string();
          debcred.titel = record.get(11).unwrap().to_string();
          debcred.adelstitel = record.get(12).unwrap().to_string();
          debcred.namensvorsatz = record.get(13).unwrap().to_string();
          debcred.adressart = record.get(14).unwrap().to_string();
          debcred.straße = record.get(15).unwrap().to_string();
          debcred.postfach = record.get(16).unwrap().to_string();
          debcred.postleitzahl = record.get(17).unwrap().to_string();
          debcred.ort = record.get(18).unwrap().to_string();
          debcred.land = record.get(19).unwrap().to_string();
          debcred.versandzusatz = record.get(20).unwrap().to_string();
          debcred.adresszusatz = record.get(21).unwrap().to_string();
          debcred.abweichende_anrede = record.get(22).unwrap().to_string();
          debcred.abw_zustellbezeichnung_1 = record.get(23).unwrap().to_string();
          debcred.abw_zustellbezeichnung_2 = record.get(24).unwrap().to_string();
          debcred.kennz_korrespondenzadresse = record.get(25).unwrap().to_string();
          debcred.adresse_gültig_von = record.get(26).unwrap().to_string();
          debcred.adresse_gültig_bis = record.get(27).unwrap().to_string();
          debcred.telefon = record.get(28).unwrap().to_string();
          debcred.bemerkung_telefon = record.get(29).unwrap().to_string();
          debcred.telefon_gl = record.get(30).unwrap().to_string();
          debcred.bemerkung_telefon_gl = record.get(31).unwrap().to_string();
          debcred.e_mail = record.get(32).unwrap().to_string();
          debcred.bemerkung_e_mail = record.get(33).unwrap().to_string();
          debcred.internet = record.get(34).unwrap().to_string();
          debcred.bemerkung_internet = record.get(35).unwrap().to_string();
          debcred.fax = record.get(36).unwrap().to_string();
          debcred.bemerkung_fax = record.get(37).unwrap().to_string();
          debcred.sonstige = record.get(38).unwrap().to_string();
          debcred.bemerkung_sonstige = record.get(39).unwrap().to_string();
          debcred.bankleitzahl_1 = record.get(40).unwrap().to_string();
          debcred.bankbezeichnung_1 = record.get(41).unwrap().to_string();
          debcred.bank_kontonummer_1 = record.get(42).unwrap().to_string();
          debcred.länderkennzeichen_1 = record.get(43).unwrap().to_string();
          debcred.iban_nr_1 = record.get(44).unwrap().to_string();
          debcred.leerfeld1 = record.get(45).unwrap().to_string();
          debcred.swift_code_1 = record.get(46).unwrap().to_string();
          debcred.abw_kontoinhaber_1 = record.get(47).unwrap().to_string();
          debcred.kennz_hauptbankverb_1 = record.get(48).unwrap().to_string();
          debcred.bankverb_1_gültig_von = record.get(49).unwrap().to_string();
          debcred.bankverb_1_gültig_bis = record.get(50).unwrap().to_string();
          debcred.bankleitzahl_2 = record.get(51).unwrap().to_string();
          debcred.bankbezeichnung_2 = record.get(52).unwrap().to_string();
          debcred.bank_kontonummer_2 = record.get(53).unwrap().to_string();
          debcred.länderkennzeichen_2 = record.get(54).unwrap().to_string();
          debcred.iban_nr_2 = record.get(55).unwrap().to_string();
          debcred.leerfeld2 = record.get(56).unwrap().to_string();
          debcred.swift_code_2 = record.get(57).unwrap().to_string();
          debcred.abw_kontoinhaber_2 = record.get(58).unwrap().to_string();
          debcred.kennz_hauptbankverb_2 = record.get(59).unwrap().to_string();
          debcred.bankverb_2_gültig_von = record.get(60).unwrap().to_string();
          debcred.bankverb_2_gültig_bis = record.get(61).unwrap().to_string();
          debcred.bankleitzahl_3 = record.get(62).unwrap().to_string();
          debcred.bankbezeichnung_3 = record.get(63).unwrap().to_string();
          debcred.bank_kontonummer_3 = record.get(64).unwrap().to_string();
          debcred.länderkennzeichen_3 = record.get(65).unwrap().to_string();
          debcred.iban_nr_3 = record.get(66).unwrap().to_string();
          debcred.leerfeld3 = record.get(67).unwrap().to_string();
          debcred.swift_code_3 = record.get(68).unwrap().to_string();
          debcred.abw_kontoinhaber_3 = record.get(69).unwrap().to_string();
          debcred.kennz_hauptbankverb_3 = record.get(70).unwrap().to_string();
          debcred.bankverb_3_gültig_von = record.get(71).unwrap().to_string();
          debcred.bankverb_3_gültig_bis = record.get(72).unwrap().to_string();
          debcred.bankleitzahl_4 = record.get(73).unwrap().to_string();
          debcred.bankbezeichnung_4 = record.get(74).unwrap().to_string();
          debcred.bank_kontonummer_4 = record.get(75).unwrap().to_string();
          debcred.länderkennzeichen_4 = record.get(76).unwrap().to_string();
          debcred.iban_nr_4 = record.get(77).unwrap().to_string();
          debcred.leerfeld4 = record.get(78).unwrap().to_string();
          debcred.swift_code_4 = record.get(79).unwrap().to_string();
          debcred.abw_kontoinhaber_4 = record.get(80).unwrap().to_string();
          debcred.kennz_hauptbankverb_4 = record.get(81).unwrap().to_string();
          debcred.bankverb_4_gültig_von = record.get(82).unwrap().to_string();
          debcred.bankverb_4_gültig_bis = record.get(83).unwrap().to_string();
          debcred.bankleitzahl_5 = record.get(84).unwrap().to_string();
          debcred.bankbezeichnung_5 = record.get(85).unwrap().to_string();
          debcred.bank_kontonummer_5 = record.get(86).unwrap().to_string();
          debcred.länderkennzeichen_5 = record.get(87).unwrap().to_string();
          debcred.iban_nr_5 = record.get(88).unwrap().to_string();
          debcred.leerfeld5 = record.get(89).unwrap().to_string();
          debcred.swift_code_5 = record.get(90).unwrap().to_string();
          debcred.abw_kontoinhaber_5 = record.get(91).unwrap().to_string();
          debcred.kennz_hauptbankverb_5 = record.get(92).unwrap().to_string();
          debcred.bankverb_5_gültig_von = record.get(93).unwrap().to_string();
          debcred.bankverb_5_gültig_bis = record.get(94).unwrap().to_string();
          debcred.leerfeld6 = record.get(95).unwrap().to_string();
          debcred.briefanrede = record.get(96).unwrap().to_string();
          debcred.grußformel = record.get(97).unwrap().to_string();
          debcred.kunden_lief_nr = record.get(98).unwrap().to_string();
          debcred.steuernummer = record.get(99).unwrap().to_string();
          debcred.sprache = record.get(100).unwrap().to_string();
          debcred.ansprechpartner = record.get(101).unwrap().to_string();
          debcred.vertreter = record.get(102).unwrap().to_string();
          debcred.sachbearbeiter = record.get(103).unwrap().to_string();
          debcred.diverse_konto = record.get(104).unwrap().to_string();
          debcred.ausgabeziel = record.get(105).unwrap().to_string();
          debcred.währungssteuerung = record.get(106).unwrap().to_string();
          debcred.kreditlimit_debitor = record.get(107).unwrap().to_string();
          debcred.zahlungsbedingung = record.get(108).unwrap().to_string();
          debcred.fälligkeit_in_tagen_debitor = record.get(109).unwrap().to_string();
          debcred.skonto_in_prozent_debitor = record.get(110).unwrap().to_string();
          debcred.kreditoren_ziel_1_tg = record.get(111).unwrap().to_string();
          debcred.kreditoren_skonto_1_prozent = record.get(112).unwrap().to_string();
          debcred.kreditoren_ziel_2_tg = record.get(113).unwrap().to_string();
          debcred.kreditoren_skonto_2_prozent = record.get(114).unwrap().to_string();
          debcred.kreditoren_ziel_3_brutto_tg = record.get(115).unwrap().to_string();
          debcred.kreditoren_ziel_4_tg = record.get(116).unwrap().to_string();
          debcred.kreditoren_skonto_4_prozent = record.get(117).unwrap().to_string();
          debcred.kreditoren_ziel_5_tg = record.get(118).unwrap().to_string();
          debcred.kreditoren_skonto_5_prozent = record.get(119).unwrap().to_string();
          debcred.mahnung = record.get(120).unwrap().to_string();
          debcred.kontoauszug = record.get(121).unwrap().to_string();
          debcred.mahntext_1 = record.get(122).unwrap().to_string();
          debcred.mahntext_2 = record.get(123).unwrap().to_string();
          debcred.mahntext_3 = record.get(124).unwrap().to_string();
          debcred.kontoauszugstext = record.get(125).unwrap().to_string();
          debcred.mahnlimit_betrag = record.get(126).unwrap().to_string();
          debcred.mahnlimit_prozent = record.get(127).unwrap().to_string();
          debcred.zinsberechnung = record.get(128).unwrap().to_string();
          debcred.mahnzinssatz_1 = record.get(129).unwrap().to_string();
          debcred.mahnzinssatz_2 = record.get(130).unwrap().to_string();
          debcred.mahnzinssatz_3 = record.get(131).unwrap().to_string();
          debcred.lastschrift = record.get(132).unwrap().to_string();
          debcred.leerfeld7 = record.get(133).unwrap().to_string();
          debcred.mandantenbank = record.get(134).unwrap().to_string();
          debcred.zahlungsträger = record.get(135).unwrap().to_string();
          debcred.indiv_feld_1 = record.get(136).unwrap().to_string();
          debcred.indiv_feld_2 = record.get(137).unwrap().to_string();
          debcred.indiv_feld_3 = record.get(138).unwrap().to_string();
          debcred.indiv_feld_4 = record.get(139).unwrap().to_string();
          debcred.indiv_feld_5 = record.get(140).unwrap().to_string();
          debcred.indiv_feld_6 = record.get(141).unwrap().to_string();
          debcred.indiv_feld_7 = record.get(142).unwrap().to_string();
          debcred.indiv_feld_8  = record.get(143).unwrap().to_string();
          debcred.indiv_feld_9 = record.get(144).unwrap().to_string();
          debcred.indiv_feld_10 = record.get(145).unwrap().to_string();
          debcred.indiv_feld_11 = record.get(146).unwrap().to_string();
          debcred.indiv_feld_12 = record.get(147).unwrap().to_string();
          debcred.indiv_feld_13 = record.get(148).unwrap().to_string();
          debcred.indiv_feld_14 = record.get(149).unwrap().to_string();
          debcred.indiv_feld_15 = record.get(150).unwrap().to_string();
          debcred.abweichende_anrede_rechnungsadresse = record.get(151).unwrap().to_string();
          debcred.adressart_rechnungsadresse = record.get(152).unwrap().to_string();
          debcred.straße_rechnungsadresse = record.get(153).unwrap().to_string();
          debcred.postfach_rechnungsadresse = record.get(154).unwrap().to_string();
          debcred.postleitzahl_rechnungsadresse = record.get(155).unwrap().to_string();
          debcred.ort_rechnungsadresse = record.get(156).unwrap().to_string();
          debcred.land_rechnungsadresse = record.get(157).unwrap().to_string();
          debcred.versandzusatz_rechnungsadresse = record.get(158).unwrap().to_string();
          debcred.adresszusatz_rechnungsadresse = record.get(159).unwrap().to_string();
          debcred.abw_zustellbezeichnung_1_rechnungsadresse = record.get(160).unwrap().to_string();
          debcred.abw_zustellbezeichnung_2_rechnungsadresse = record.get(161).unwrap().to_string();
          debcred.adresse_gültig_von_rechnungsadresse = record.get(162).unwrap().to_string();
          debcred.adresse_gültig_bis_rechnungsadresse = record.get(163).unwrap().to_string();
          debcred.bankleitzahl_6 = record.get(164).unwrap().to_string();
          debcred.bankbezeichnung_6 = record.get(165).unwrap().to_string();
          debcred.bank_kontonummer_6 = record.get(166).unwrap().to_string();
          debcred.länderkennzeichen_6 = record.get(167).unwrap().to_string();
          debcred.iban_nr_6 = record.get(168).unwrap().to_string();
          debcred.leerfeld8 = record.get(169).unwrap().to_string();
          debcred.swift_code_6 = record.get(170).unwrap().to_string();
          debcred.abw_kontoinhaber_6 = record.get(171).unwrap().to_string();
          debcred.kennz_hauptbankverb_6 = record.get(172).unwrap().to_string();
          debcred.bankverb_6_gültig_von = record.get(173).unwrap().to_string();
          debcred.bankverb_6_gültig_bis = record.get(174).unwrap().to_string();
          debcred.bankleitzahl_7 = record.get(175).unwrap().to_string();
          debcred.bankbezeichnung_7 = record.get(176).unwrap().to_string();
          debcred.bank_kontonummer_7 = record.get(177).unwrap().to_string();
          debcred.länderkennzeichen_7 = record.get(178).unwrap().to_string();
          debcred.iban_nr_7 = record.get(179).unwrap().to_string();
          debcred.leerfeld9 = record.get(180).unwrap().to_string();
          debcred.swift_code_7 = record.get(181).unwrap().to_string();
          debcred.abw_kontoinhaber_7 = record.get(182).unwrap().to_string();
          debcred.kennz_hauptbankverb_7 = record.get(183).unwrap().to_string();
          debcred.bankverb_7_gültig_von = record.get(184).unwrap().to_string();
          debcred.bankverb_7_gültig_bis = record.get(185).unwrap().to_string();
          debcred.bankleitzahl_8 = record.get(186).unwrap().to_string();
          debcred.bankbezeichnung_8 = record.get(187).unwrap().to_string();
          debcred.bank_kontonummer_8 = record.get(188).unwrap().to_string();
          debcred.länderkennzeichen_8 = record.get(189).unwrap().to_string();
          debcred.iban_nr_8 = record.get(190).unwrap().to_string();
          debcred.leerfeld10 = record.get(191).unwrap().to_string();
          debcred.swift_code_8 = record.get(192).unwrap().to_string();
          debcred.abw_kontoinhaber_8 = record.get(193).unwrap().to_string();
          debcred.kennz_hauptbankverb_8 = record.get(194).unwrap().to_string();
          debcred.bankverb_8_gültig_von = record.get(195).unwrap().to_string();
          debcred.bankverb_8_gültig_bis = record.get(196).unwrap().to_string();
          debcred.bankleitzahl_9 = record.get(197).unwrap().to_string();
          debcred.bankbezeichnung_9 = record.get(198).unwrap().to_string();
          debcred.bank_kontonummer_9 = record.get(199).unwrap().to_string();
          debcred.länderkennzeichen_9 = record.get(200).unwrap().to_string();
          debcred.iban_nr_9 = record.get(201).unwrap().to_string();
          debcred.leerfeld11 = record.get(202).unwrap().to_string();
          debcred.swift_code_9 = record.get(203).unwrap().to_string();
          debcred.abw_kontoinhaber_9 = record.get(204).unwrap().to_string();
          debcred.kennz_hauptbankverb_9 = record.get(205).unwrap().to_string();
          debcred.bankverb_9_gültig_von = record.get(206).unwrap().to_string();
          debcred.bankverb_9_gültig_bis = record.get(207).unwrap().to_string();
          debcred.bankleitzahl_10 = record.get(208).unwrap().to_string();
          debcred.bankbezeichnung_10 = record.get(209).unwrap().to_string();
          debcred.bank_kontonummer_10 = record.get(210).unwrap().to_string();
          debcred.länderkennzeichen_10 = record.get(211).unwrap().to_string();
          debcred.iban_nr_10 = record.get(212).unwrap().to_string();
          debcred.leerfeld12 = record.get(213).unwrap().to_string();
          debcred.swift_code_10 = record.get(214).unwrap().to_string();
          debcred.abw_kontoinhaber_10 = record.get(215).unwrap().to_string();
          debcred.kennz_hauptbankverb_10  = record.get(216).unwrap().to_string();
          debcred.bankverb_10_gültig_von = record.get(217).unwrap().to_string();
          debcred.bankverb_10_gültig_bis = record.get(218).unwrap().to_string();
          debcred.nummer_fremdsystem = record.get(219).unwrap().to_string();
          debcred.insolvent = record.get(220).unwrap().to_string();
          debcred.sepa_mandatsreferenz_1  = record.get(221).unwrap().to_string();
          debcred.sepa_mandatsreferenz_2 = record.get(222).unwrap().to_string();
          debcred.sepa_mandatsreferenz_3 = record.get(223).unwrap().to_string();
          debcred.sepa_mandatsreferenz_4 = record.get(224).unwrap().to_string();
          debcred.sepa_mandatsreferenz_5 = record.get(225).unwrap().to_string();
          debcred.sepa_mandatsreferenz_6  = record.get(226).unwrap().to_string();
          debcred.sepa_mandatsreferenz_7 = record.get(227).unwrap().to_string();
          debcred.sepa_mandatsreferenz_8 = record.get(228).unwrap().to_string();
          debcred.sepa_mandatsreferenz_9 = record.get(229).unwrap().to_string();
          debcred.sepa_mandatsreferenz_10 = record.get(230).unwrap().to_string();
          debcred.verknüpftes_opos_konto = record.get(231).unwrap().to_string();
          debcred.mahnsperre_bis = record.get(232).unwrap().to_string();
          debcred.lastschriftsperre_bis = record.get(233).unwrap().to_string();
          debcred.zahlungssperre_bis = record.get(234).unwrap().to_string();
          debcred.gebührenberechnung = record.get(235).unwrap().to_string();
          debcred.mahngebühr_1 = record.get(236).unwrap().to_string();
          debcred.mahngebühr_2 = record.get(237).unwrap().to_string();
          debcred.mahngebühr_3 = record.get(238).unwrap().to_string();
          debcred.pauschalenberechnung = record.get(239).unwrap().to_string();
          debcred.verzugspauschale_1 = record.get(240).unwrap().to_string();
          debcred.verzugspauschale_2 = record.get(241).unwrap().to_string();
          debcred.verzugspauschale_3 = record.get(242).unwrap().to_string();
          debcred.alternativer_suchname = record.get(243).unwrap().to_string();
          debcred.status = record.get(244).unwrap().to_string();
          debcred.anschrift_manuell_geändert_korrespondenzadresse = record.get(245).unwrap().to_string();
          debcred.anschrift_individuell_korrespondenzadresse = record.get(246).unwrap().to_string();
          debcred.anschrift_manuell_geändert_rechnungsadresse = record.get(247).unwrap().to_string();
          debcred.anschrift_individuell_rechnungsadresse = record.get(248).unwrap().to_string();
          debcred.fristberechnung_bei_debitor = record.get(249).unwrap().to_string();
          debcred.mahnfrist_1   = record.get(250).unwrap().to_string();
          debcred.mahnfrist_2 = record.get(251).unwrap().to_string();
          debcred.mahnfrist_3 = record.get(252).unwrap().to_string();
          debcred.letzte_frist = record.get(253).unwrap().to_string();
          Ok(debcred)
      }else{
          Err("content not recognised")
      }  
  }
}
