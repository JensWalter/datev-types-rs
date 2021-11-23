# datev-types-rs
provides the DATEV format types including some serialization.

## still work in progress

This crate is still a work in progress and breaking changes will come at any time.

## Sample

render datev csv

```rust
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
    let str = format!("{}", stapel);
```