# datev-types-rs
provides the DATEV format types including some serialization.

## still work in progress

This crate is still a work in progress and breaking changes will come at any time.

## Sample

render datev csv

```rust
use datev_types::header::Header;
use datev_types::buchung::Buchung;
use datev_types::buchung::SollHabenKennzeichen;
use datev_types::Buchungsstapel;
use chrono::NaiveDate;

fn main() {
    let header = Header{
        format_name: "Buchungsstapel".to_string(),
        erzeugt_am: chrono::Local::now().naive_local(),
        beraternummer: 1000,
        mandantennummer: 1,
        wj_beginn: NaiveDate::from_ymd(2019,01,01),
        sachkontenlänge: 4,
        datum_von: NaiveDate::from_ymd(2019,01,01),
        datum_bis: NaiveDate::from_ymd(2019,12,31),
        ..Default::default()
    };
    let buchung = Buchung{
        soll_haben_kennzeichen: SollHabenKennzeichen::Soll,
        umsatz: 100.0,
        beleg_datum: NaiveDate::from_ymd(2021, 2, 28),
        konto: 1800,
        gegenkonto: 1420,
        buchungstext: Some("zahlung 123".to_string()),
        ..Default::default()
    };
    let stapel = Buchungsstapel{
        header: header,
        buchungen: vec![buchung],
    };
    let str = format!("{}", stapel);
    println!("{}", str);
    let json_str = serde_json::to_string_pretty(&stapel).unwrap();
    println!("{}", json_str);
}
```

produces the following output
```csv
"EXTF";700;21;"Buchungsstapel";12;20211127230309532;;;;;1000;1;20190101;4;20190101;20191231;;;;;;;;;;;;;;;
100;S;EUR;;;;1800;1420;;2802;;;;zahlung 123;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
```

```json
{
  "header": {
    "Kennzeichen": "EXTF",
    "Versionsnummer": 700,
    "FormatKategorie": 21,
    "FormatName": "Buchungsstapel",
    "FormatVersion": 12,
    "ErzeugtAm": "2021-11-27T23:03:09.532227",
    "Beraternummer": 1000,
    "Mandantennummer": 1,
    "WjBeginn": "2019-01-01",
    "Sachkontenlänge": 4,
    "DatumVon": "2019-01-01",
    "DatumBis": "2019-12-31"
  },
  "buchungen": [
    {
      "umsatz": 100.0,
      "sollHabenKennzeichen": "Soll",
      "wkzUmsatz": "EUR",
      "konto": 1800,
      "gegenkonto": 1420,
      "belegDatum": "2021-02-28",
      "buchungstext": "zahlung 123"
    }
  ]
}
```