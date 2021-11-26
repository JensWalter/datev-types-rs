use serde::{Serialize, Deserialize};
use validator::Validate;
use std::fmt::Display;
use std::fmt::Formatter;
use header::Header;
use buchung::Buchung;
#[macro_use]
extern crate lazy_static;

pub mod header;
pub mod buchung;

#[derive(Clone, Debug, PartialEq, Validate, Serialize, Deserialize)]
pub struct Buchungsstapel {
    header: Header,
    buchungen: Vec<Buchung>,
}

impl Default for Buchungsstapel{
    fn default() -> Self {
        Buchungsstapel {
            header: Header::default(),
            buchungen: Vec::new(),
        }
    }
}

impl Display for Buchungsstapel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}",self.header))?;
        for buchung in &self.buchungen {
            buchung.fmt(f)?;
        }
        Ok(())
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
        
        let header_str: &str = vec.get(0).unwrap();
        let header = Header::try_from(header_str).unwrap();

        let mut buchungen: Vec<Buchung> = Vec::new();
        for input in vec.iter().skip(2) {
            let input2: String = input.to_string();
            let input3: String = input2.replace(',',".");
            process_line(input3.as_bytes(), &mut buchungen);
        }

        let stapel = Buchungsstapel{
            header,
            buchungen,
        };
        Ok(stapel)
    }
}

fn process_line(bytes: &[u8], buchungen: &mut Vec<Buchung>) {
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').flexible(true)
    .has_headers(false).from_reader(bytes);

            //manual way
            let mut iter = rdr.records();
            if let Some(result) = iter.next() {
                let record = result.unwrap();
                let mut buchung = Buchung::default();
                //add values
                if let Some(val) = record.get(0) {
                    let fixed_decimal = val.replace(',', ".");
                    buchung.umsatz = fixed_decimal.parse().unwrap();
                }
                if let Some(val) = record.get(1) {
                    buchung.soll_haben_kennzeichen = val.to_string();
                }
                if let Some(val) = record.get(2) {
                    buchung.wkz_umsatz = val.to_string();
                }
                if let Some(val) = record.get(3) {
                    if val.len() > 0 {
                        let fixed_decimal = val.replace(',', ".");
                        buchung.kurs = Some(fixed_decimal.parse().unwrap());
                    }
                }
                if let Some(val) = record.get(4) {
                    if val.len() > 0 {
                        let fixed_decimal = val.replace(',', ".");
                        buchung.basis_umsatz = Some(fixed_decimal.parse().unwrap());
                    }
                }
                if let Some(val) = record.get(5) {
                    buchung.wkz_basis_umsatz = val.to_string();
                }
                if let Some(val) = record.get(6) {
                    buchung.konto = val.parse().unwrap();
                }
                if let Some(val) = record.get(7) {
                    buchung.gegenkonto = val.parse().unwrap();
                }
                if let Some(val) = record.get(8) {
                    if val.len() > 0 {
                        buchung.bu_schlüssel = Some(val.parse().unwrap());
                    }
                }
                if let Some(val) = record.get(9) {
                    buchung.beleg_datum = val.parse().unwrap();
                }
                if let Some(val) = record.get(10) {
                    buchung.belegfeld1 = val.to_string();
                }
                if let Some(val) = record.get(11) {
                    buchung.belegfeld2 = val.to_string();
                }
                if let Some(val) = record.get(12) {
                    if val.len() > 0 {
                        let fixed_decimal = val.replace(',', ".");
                        buchung.skonto = Some(fixed_decimal.parse().unwrap());
                    }
                }
        // pub buchungstext: Option<String>,
        // #[validate(range(min = 0, max = 1))]
        // pub postensperre: Option<u8>,
        // pub diverse_adressnummer: String,
        // pub geschäftspartner_bank: String,
        // pub sachverhalt: String,
        // pub zinssperre: String,
        // pub beleg_link: String,
        // pub beleg_info_art1: String,
        // pub beleg_info_inhalt1: String,
        // pub beleg_info_art2: String,
        // pub beleg_info_inhalt2: String,
        // pub beleg_info_art3: String,
        // pub beleg_info_inhalt3: String,
        // pub beleg_info_art4: String,
        // pub beleg_info_inhalt4: String,
        // pub beleg_info_art5: String,
        // pub beleg_info_inhalt5: String,
        // pub beleg_info_art6: String,
        // pub beleg_info_inhalt6: String,
        // pub beleg_info_art7: String,
        // pub beleg_info_inhalt7: String,
        // pub beleg_info_art8: String,
        // pub beleg_info_inhalt8: String,
        // kost1_kostenstelle: Option<String>,
        // kost2_kostenstelle: Option<String>,
        // kost_menge: Option<f64>,
        // eu_ustid: Option<String>,
        // eu_steuersatz: Option<f64>,
        // abweichende_versteuerungsart: String,
        // sachverhalt_l_l: String,
        // funktionsergänzung_l_l: String,
        // bu_49_hauptfunktiontyp: String,
        // bu_49_hauptfunktionsnummer: String,
        // bu_49_funktionsergänzung: String,
        // pub zusatzinformation_art1: Option<String>,
        // pub zusatzinformation_inhalt1: Option<String>,
        // pub zusatzinformation_art2: Option<String>,
        // pub zusatzinformation_inhalt2: Option<String>,
        // pub zusatzinformation_art3: Option<String>,
        // pub zusatzinformation_inhalt3: Option<String>,
        // pub zusatzinformation_art4: Option<String>,
        // pub zusatzinformation_inhalt4: Option<String>,
        // pub zusatzinformation_art5: Option<String>,
        // pub zusatzinformation_inhalt5: Option<String>,
        // pub zusatzinformation_art6: Option<String>,
        // pub zusatzinformation_inhalt6: Option<String>,
        // pub zusatzinformation_art7: Option<String>,
        // pub zusatzinformation_inhalt7: Option<String>,
        // pub zusatzinformation_art8: Option<String>,
        // pub zusatzinformation_inhalt8: Option<String>,
        // pub zusatzinformation_art9: Option<String>,
        // pub zusatzinformation_inhalt9: Option<String>,
        // pub zusatzinformation_art10: Option<String>,    
        // pub zusatzinformation_inhalt10: Option<String>,
        // pub zusatzinformation_art11: Option<String>,
        // pub zusatzinformation_inhalt11: Option<String>,
        // pub zusatzinformation_art12: Option<String>,
        // pub zusatzinformation_inhalt12: Option<String>,
        // pub zusatzinformation_art13: Option<String>,
        // pub zusatzinformation_inhalt13: Option<String>,
        // pub zusatzinformation_art14: Option<String>,
        // pub zusatzinformation_inhalt14: Option<String>,
        // pub zusatzinformation_art15: Option<String>,
        // pub zusatzinformation_inhalt15: Option<String>,
        // pub zusatzinformation_art16: Option<String>,
        // pub zusatzinformation_inhalt16: Option<String>,
        // pub zusatzinformation_art17: Option<String>,
        // pub zusatzinformation_inhalt17: Option<String>,
        // pub zusatzinformation_art18: Option<String>,
        // pub zusatzinformation_inhalt18: Option<String>,
        // pub zusatzinformation_art19: Option<String>,
        // pub zusatzinformation_inhalt19: Option<String>,
        // pub zusatzinformation_art20: Option<String>,
        // pub zusatzinformation_inhalt20: Option<String>,
        // pub stück: Option<f64>,
        // pub gewicht: Option<f64>,
        // pub zahlweise: Option<String>,
        // pub forderungsart: Option<String>,
        // pub forderungsjahr: Option<u16>,
        // pub veranlagungsjahr: Option<u16>,
        // pub zugeordnete_fälligkeit: Option<String>,
        // pub skontotyp: Option<String>,
        // pub auftragsnummer: Option<String>,
        // pub buchungstyp: Option<String>,
        // pub ust_schlüssel_anzahlung: Option<u8>,
        // pub eu_mitgliedstaat_anzahlung: Option<String>,
        // pub sachverhalt_l_l_anzahlung: Option<String>,
        // pub eu_steuersatz_anzahlung: Option<f64>,
        // pub erlöskonto_anzahlung: Option<u32>,
        // pub herkunft_kz: Option<String>,
        // pub leerfeld: Option<String>,
        // pub kost_datum: Option<String>,
        // pub sepa_mandatsreferenz: Option<String>,
        // pub skontosperre: Option<u8>,
        // pub gesellschaftername: Option<String>,
        // pub beteiligtennummer: Option<String>,
        // pub identifikationsnummer: Option<String>,
        // pub zeichennummer: Option<String>,
        // pub postensperre_bis: Option<String>,
        // pub bezeichnung_so_bil_sachverhalt: Option<String>,
        // pub kennzeichen_so_bil_buchung: Option<String>,
        // pub festschreibung: Option<u8>,
        // pub leistungsdatum: Option<String>,
        // pub datum_zuord_steuerperiode: Option<String>,
        // pub fälligkeit: Option<String>,
        // pub generalumkehr: Option<String>,
        // pub steuersatz: Option<f64>,
        // pub land: Option<String>,
        // pub abrechnungsreferenz: Option<String>,
        // pub bvv_position: Option<String>,
        // pub eu_ustid_ursprung: Option<String>,
        // pub eu_steuersatz_ursprung: Option<f64>,
                buchungen.push(buchung);
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
    assert_eq!(header.festschreibung, None);
}

#[test]
fn full_cycle_header() {
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
    let data = format!("{}",header);
    println!("{}",data);
    let header2 = Header::try_from(data.as_str()).unwrap();
    println!("{}",header2);
    assert_eq!(header, header2);
    assert_eq!(format!("{}",header),format!("{}",header2));
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
        buchungstext: Some("zahlung 123".to_string()),
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

#[test]
fn test_sage_export_buchungstapel() {
    // use std::io::Read;
    let input = r#""EXTF";510;21;"Buchungsstapel";7;20211106165344208;;"";"";"";1000;1;20170101;4;20170101;20171231;"";"";;;;"";;"";;;"";;;"";""
Umsatz (ohne Soll/Haben-Kz);Soll/Haben-Kennzeichen;WKZ Umsatz;Kurs;Basis-Umsatz;WKZ Basis-Umsatz;Konto;Gegenkonto (ohne BU-Schlüssel);BU-Schlüssel;Belegdatum;Belegfeld 1;Belegfeld 2;Skonto;Buchungstext;Postensperre;Diverse Adressnummer;Geschäftspartnerbank;Sachverhalt;Zinssperre;Beleglink;Beleginfo - Art 1;Beleginfo - Inhalt 1;Beleginfo - Art 2;Beleginfo - Inhalt 2;Beleginfo - Art 3;Beleginfo - Inhalt 3;Beleginfo - Art 4;Beleginfo - Inhalt 4;Beleginfo - Art 5;Beleginfo - Inhalt 5;Beleginfo - Art 6;Beleginfo - Inhalt 6;Beleginfo - Art 7;Beleginfo - Inhalt 7;Beleginfo - Art 8;Beleginfo - Inhalt 8;KOST1 - Kostenstelle;KOST2 - Kostenstelle;Kost-Menge;EU-Land u. UStID;EU-Steuersatz;Abw. Versteuerungsart;Sachverhalt L+L;Funktionsergänzung L+L;BU 49 Hauptfunktionstyp;BU 49 Hauptfunktionsnummer;BU 49 Funktionsergänzung;Zusatzinformation - Art 1;Zusatzinformation- Inhalt 1;Zusatzinformation - Art 2;Zusatzinformation- Inhalt 2;Zusatzinformation - Art 3;Zusatzinformation- Inhalt 3;Zusatzinformation - Art 4;Zusatzinformation- Inhalt 4;Zusatzinformation - Art 5;Zusatzinformation- Inhalt 5;Zusatzinformation - Art 6;Zusatzinformation- Inhalt 6;Zusatzinformation - Art 7;Zusatzinformation- Inhalt 7;Zusatzinformation - Art 8;Zusatzinformation- Inhalt 8;Zusatzinformation - Art 9;Zusatzinformation- Inhalt 9;Zusatzinformation - Art 10;Zusatzinformation- Inhalt 10;Zusatzinformation - Art 11;Zusatzinformation- Inhalt 11;Zusatzinformation - Art 12;Zusatzinformation- Inhalt 12;Zusatzinformation - Art 13;Zusatzinformation- Inhalt 13;Zusatzinformation - Art 14;Zusatzinformation- Inhalt 14;Zusatzinformation - Art 15;Zusatzinformation- Inhalt 15;Zusatzinformation - Art 16;Zusatzinformation- Inhalt 16;Zusatzinformation - Art 17;Zusatzinformation- Inhalt 17;Zusatzinformation - Art 18;Zusatzinformation- Inhalt 18;Zusatzinformation - Art 19;Zusatzinformation- Inhalt 19;Zusatzinformation - Art 20;Zusatzinformation- Inhalt 20;Stück;Gewicht;Zahlweise;Forderungsart;Veranlagungsjahr;Zugeordnete Fälligkeit;Skontotyp;Auftragsnummer;Buchungstyp;USt-Schlüssel (Anzahlungen);EU-Land (Anzahlungen);Sachverhalt L+L (Anzahlungen);EU-Steuersatz (Anzahlungen);Erlöskonto (Anzahlungen);Herkunft-Kz;Buchungs GUID;KOST-Datum;SEPA-Mandatsreferenz;Skontosperre;Gesellschaftername;Beteiligtennummer;Identifikationsnummer;Zeichnernummer;Postensperre bis;Bezeichnung SoBil-Sachverhalt;Kennzeichen SoBil-Buchung;Festschreibung;Leistungsdatum;Datum Zuord. Steuerperiode
25000,00;"S";"EUR";;;"";1800;1460;"";0610;"1";"";;"1";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;
25000,00;"S";"EUR";;;"";1460;2900;"";1712;"2";"";;"2";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;
25000,00;"S";"EUR";;;"";2900;1460;"";1712;"3";"";;"Storno von Journal";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;
25000,00;"S";"EUR";;;"";1460;2900;"";0610;"4";"";;"2";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;
25000,00;"S";"EUR";;;"";1810;1800;"";2410;"5";"";;"3";;"";;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;"";;"";;;;;;"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";"";;;;"";;;;"";"";;"";;;;"";"";;"";;"";;"";"";;"";;;;"#;
    // let mut f = std::fs::File::open("./test-data/EXTF_Buchungsstapel_01-01-2017_bis_31-12-2017.csv").unwrap();
    // let mut buffer = String::new();
    // let _ingore = f.read_to_string(&mut buffer);
    let stapel: Buchungsstapel = Buchungsstapel::try_from(input).unwrap();
    println!("{:?}",stapel);
    println!("{}", serde_json::to_string_pretty(&stapel).unwrap());
    // panic!("x");
}