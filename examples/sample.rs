use datev_types::header::Header;
use datev_types::buchung::Buchung;
use datev_types::Buchungsstapel;
use chrono::NaiveDate;

fn main() {
    let header = Header{
        format_name: "Buchungsstapel".to_string(),
        erzeugt_am: 20211106165314647,
        beraternummer: 1000,
        mandantennummer: 1,
        wj_beginn: NaiveDate::from_ymd(2019,01,01),
        sachkontenlänge: 4,
        datum_von: NaiveDate::from_ymd(2019,01,01),
        datum_bis: NaiveDate::from_ymd(2019,12,31),
        ..Default::default()
    };
    let buchung = Buchung{
        soll_haben_kennzeichen: "S".to_string(),
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