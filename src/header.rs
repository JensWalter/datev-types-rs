use serde::{Serialize, Deserialize};
use validator::Validate;
use regex::Regex;
use std::fmt::Display;
use std::fmt::Formatter;

lazy_static! {
  static ref KENNZEICHEN: Regex = Regex::new(r#"^(EXTF|DTVF)$"#).unwrap();
  static ref FORMATNAME: Regex = Regex::new(r#"^(Buchungsstapel|Wiederkehrende Buchungen|Debitoren/Kreditoren|Sachkontenbeschriftungen|Zahlungsbedingungen|Diverse Adressen)$"#).unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq, Validate, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Header {
  #[validate(regex = "KENNZEICHEN")]
  pub kennzeichen: String,
  pub versionsnummer: u32,
  /// 16 = Debitoren-/Kreditoren
  /// 20 = Sachkontenbeschriftungen
  /// 21 = Buchungsstapel
  /// 46 = Zahlungsbedingungen
  /// 48 = Diverse Adressen
  /// 65 = Wiederkehrende Buchungen
  pub format_kategorie: u16,
  #[validate(regex = "FORMATNAME")]
  pub format_name: String,
  /// Debitoren-/Kreditoren = 5
  /// Sachkontenbeschriftungen = 3
  /// Buchungsstapel = 12
  /// Zahlungsbedingungen = 2
  /// Wiederkehrende Buchungen = 4
  /// Diverse Adressen = 2
  pub format_version: u16,
  /// Zeitstempel:
  /// YYYYMMDDHHMMSSFFF
  pub erzeugt_am: u64,
  pub leerfeld1: String,
  pub leerfeld2: String,
  pub leerfeld3: String,
  pub leerfeld4: String,
  /// Bereich 1001-9999999
  pub beraternummer: u32,
  /// Bereich 1-99999
  pub mandantennummer: u32,
  /// Wirtschaftsjahresbeginn
  /// Format: YYYYMMDD
  pub wj_beginn: u64,
  /// Nummernlänge der Sachkonten.
  /// Wert muss beim Import mit Konfiguration des Mandats in der DATEV App übereinstimmen.
  pub sachkontenlänge: u32,
  /// Beginn der Periode des Stapels
  /// Format: YYYYMMDD
  pub datum_von: u32,
  /// Ende der Periode des Stapels
  /// Format: YYYYMMDD
  pub datum_bis: u32,
  /// Bezeichnung des Stapels
  /// z.B. „Rechnungsausgang 09/2019“
  #[validate(length(min = 0, max = 30))]
  pub bezeichnung: String,
  /// Kürzel in Großbuchstaben des Bearbeiters
  /// z.B. "MM" für Max Mustermann
  #[validate(length(min = 0, max = 2))]
  pub diktatkürzel: String,
  /// 1 = Finanzbuchführung (default)
  /// 2 = Jahresabschluss
  #[serde(skip_serializing_if = "Option::is_none")]
  pub buchungstyp: Option<BuchungsTyp>,
  /// 0 = unabhängig (default)
  /// 30 = Steuerrecht
  /// 40 = Kalkulatorik
  /// 50 = Handelsrecht
  /// 64 = IFRS
  pub rechnungslegungszweck: Option<u8>,
  /// 0 = keine Festschreibung
  /// 1 = Festschreibung (default)
  // #[serde(default = "default_festschreibung")]
  pub festschreibung: Option<Festschreibung>,
  /// ISO-Code der Währung "EUR" = default
  #[validate(length(min = 0, max = 3))]
  pub wkz: Option<String>,
  pub leerfeld5: String,
  pub derivatskennzeichen: String,
  pub leerfeld6: String,
  pub leerfeld7: String,
  /// Sachkontenrahmen der für die Bewegungsdaten verwendet wurde
  pub sachkontenrahmen: String,
  /// Falls eine spezielle DATEV Branchenlösung genutzt wird.
  #[validate(length(min = 0, max = 4))]
  pub id_der_branchenlösung: String, 	
  pub leerfeld8: String,
  pub leerfeld9: String,
  /// Verarbeitungskennzeichen der abgebenden Anwendung
  // z.B. „09/2019“
  #[validate(length(min = 0, max = 16))]
  pub anwendungsinformation: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum BuchungsTyp {
  Finanzbuchführung,
  Jahresabschluss,
}

impl Default for Header {
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
          festschreibung: None,
          wkz: None,
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

impl Display for Header {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
      write!(f, r#""{kennzeichen}";{versionsnummer};{format_kategorie};"{format_name}";{format_version};{erzeugt_am};{leerfeld1};{leerfeld2};{leerfeld3};{leerfeld4};{beraternummer};{mandantennummer};{wj_beginn};{sachkontenlänge};{datum_von};{datum_bis};{bezeichnung};{diktatkürzel};{buchungstyp};{rechnungslegungszweck};{festschreibung};{wkz};{leerfeld5};{derivatskennzeichen};{leerfeld6};{leerfeld7};{sachkontenrahmen};{id_der_branchenlösung};{leerfeld8};{leerfeld9};{anwendungsinformation}{newline}"#,
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
          buchungstyp = match self.buchungstyp {
              Some(BuchungsTyp::Finanzbuchführung) => "1",
              Some(BuchungsTyp::Jahresabschluss) => "2",
              _ => "",
          },
          rechnungslegungszweck = match self.rechnungslegungszweck {
              Some(0) => "0",
              Some(30) => "30",
              Some(40) => "40",
              Some(50) => "50",
              Some(64) => "64",
              None => "",
              _ => "",
          },
          festschreibung= match self.festschreibung {
              Some(Festschreibung::KeineFestschreibung) => "0",
              Some(Festschreibung::Festschreibung) => "1",
              None => "",
          },
          wkz = match &self.wkz {
              Some(val) => val.to_string(),
              None => "".to_string(),
          },
          leerfeld5=self.leerfeld5,
          derivatskennzeichen=self.derivatskennzeichen,
          leerfeld6=self.leerfeld6,
          leerfeld7=self.leerfeld7,
          sachkontenrahmen=self.sachkontenrahmen,
          id_der_branchenlösung=self.id_der_branchenlösung,
          leerfeld8=self.leerfeld8,
          leerfeld9=self.leerfeld9,
          anwendungsinformation=self.anwendungsinformation,
          newline = "\n",
      )
  }
}

impl TryFrom<&str> for Header {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
      let mut rdr = csv::ReaderBuilder::new().delimiter(b';')
          .has_headers(false).from_reader(value.as_bytes());
      
      //manual way
      let mut iter = rdr.records();
      if let Some(result) = iter.next() {
          let record = result.unwrap();
          let mut header = Header::default();
          //add values
          if let Some(val) = record.get(0) {
              header.kennzeichen = val.to_string();
          }
          if let Some(val) = record.get(1) {
              header.versionsnummer = val.parse::<u32>().unwrap();
          }
          if let Some(val) = record.get(2) {
              header.format_kategorie = val.parse::<u16>().unwrap();
          }
          if let Some(val) = record.get(3) {
              header.format_name = val.to_string();
          }
          if let Some(val) = record.get(4) {
              header.format_version = val.parse::<u16>().unwrap();
          }
          if let Some(val) = record.get(5) {
              header.erzeugt_am = val.parse::<u64>().unwrap();
          }
          if let Some(val) = record.get(6) {
              header.leerfeld1 = val.to_string();
          }
          if let Some(val) = record.get(7) {
              header.leerfeld2 = val.to_string();
          }
          if let Some(val) = record.get(8) {
              header.leerfeld3 = val.to_string();
          }
          if let Some(val) = record.get(9) {
              header.leerfeld4 = val.to_string();
          }
          if let Some(val) = record.get(10) {
              header.beraternummer = val.parse::<u32>().unwrap();
          }
          if let Some(val) = record.get(11) {
              header.mandantennummer = val.parse::<u32>().unwrap();
          }
          if let Some(val) = record.get(12) {
              header.wj_beginn = val.parse::<u64>().unwrap();
          }
          if let Some(val) = record.get(13) {
              header.sachkontenlänge = val.parse::<u32>().unwrap();
          }
          if let Some(val) = record.get(14) {
              header.datum_von = val.parse::<u32>().unwrap();
          }
          if let Some(val) = record.get(15) {
              header.datum_bis = val.parse::<u32>().unwrap();
          }
          if let Some(val) = record.get(16) {
              header.bezeichnung = val.to_string();
          }
          if let Some(val) = record.get(17) {
              header.diktatkürzel = val.to_string();
          }
          if let Some(val) = record.get(18) {
              header.buchungstyp = match val {
                  "1" => Some(BuchungsTyp::Finanzbuchführung),
                  "2" => Some(BuchungsTyp::Jahresabschluss),
                  _ => None,
              };
          }
          if let Some(val) = record.get(19) {
              header.rechnungslegungszweck = match val {
                  "0" => Some(0),
                  "30" => Some(30),
                  "40" => Some(40),
                  "50" => Some(50),
                  "64" => Some(64),
                  _ => None,
              };
          }
          if let Some(val) = record.get(20) {
              header.festschreibung = match val {
                  "0" => Some(Festschreibung::KeineFestschreibung),
                  "1" => Some(Festschreibung::Festschreibung),
                  _ => None,
              };
          }
          if let Some(val) = record.get(21) {
              if !val.is_empty() {
                  header.wkz = Some(val.to_string());
              } else {
                  header.wkz = None;
              }
          }
          if let Some(val) = record.get(22) {
              header.leerfeld5 = val.to_string();
          }
          if let Some(val) = record.get(23) {
              header.derivatskennzeichen = val.to_string();
          }
          if let Some(val) = record.get(24) {
              header.leerfeld6 = val.to_string();
          }
          if let Some(val) = record.get(25) {
              header.leerfeld7 = val.to_string();
          }
          if let Some(val) = record.get(26) {
              header.sachkontenrahmen = val.to_string();
          }
          if let Some(val) = record.get(27) {
              header.id_der_branchenlösung = val.to_string();
          }
          if let Some(val) = record.get(28) {
              header.leerfeld8 = val.to_string();
          }
          if let Some(val) = record.get(29) {
              header.leerfeld9 = val.to_string();
          }
          if let Some(val) = record.get(30) {
              header.anwendungsinformation = val.to_string();
          }
          Ok(header)
      } else {
          Err("No header found")
      }
  }
}

#[derive(Clone, PartialEq, Debug, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Festschreibung{
  KeineFestschreibung = 0,
  Festschreibung = 1,
}
impl Default for Festschreibung {
  fn default() -> Self {
      Festschreibung::Festschreibung
  }
}
impl Display for Festschreibung {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
      write!(f, "{}", match self {
          Festschreibung::KeineFestschreibung => "0",
          Festschreibung::Festschreibung => "1",
      })
  }
}
