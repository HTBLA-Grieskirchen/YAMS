#import sys: inputs
#import "format.typ": datum, euro, prozent

#set page(paper: "a4", margin: (x: 2cm, y: 2cm))
#set text(size: 11pt)

#let praxis = inputs.praxis
#let klient = inputs.klient
#let adresse = klient.adresse

#grid(
  columns: (1fr, 1fr),
  [
    *#praxis.name*\
    #praxis.straße_und_hausnummer\
    #praxis.postleitzahl #praxis.stadt\
    #praxis.ländercode\
    #praxis.email\
    #praxis.telefon\
    UID: #praxis.uid
  ],
  align(right)[
    Rechnung Nr. *#inputs.rechnungsnummer*\
    Datum: #datum(inputs.rechnungsdatum)
  ],
)

#v(1.5cm)

*Rechnungsempfänger*\
#klient.vorname #klient.nachname\
Kundennummer: #klient.kundennummer\
#adresse.straße_und_hausnummer\
#adresse.postleitzahl #adresse.stadt\
#adresse.ländercode

#v(1cm)

#table(
  columns: (1fr, auto, auto, auto, auto),
  inset: 6pt,
  stroke: 0.4pt,
  [*Beschreibung*], [*Menge*], [*Einzelpreis*], [*MwSt*], [*Betrag*],
  ..for pos in inputs.positionen {
    (
      pos.beschreibung,
      str(pos.stückzahl),
      euro(pos.einzelpreis),
      prozent(pos.mwst),
      euro(pos.einzelpreis * pos.stückzahl),
    )
  },
)

#v(0.8cm)
#align(right)[
  Netto: #euro(inputs.gesamt_netto)\
  MwSt: #euro(inputs.gesamt_mwst)\
  *Brutto: #euro(inputs.gesamt_brutto)*
]
