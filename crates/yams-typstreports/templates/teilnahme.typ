#import sys: inputs
#import "format.typ": datum

#set page(paper: "a4", margin: 2.2cm)
#set text(size: 12pt)

#let praxis = inputs.praxis
#let klient = inputs.klient

#align(center)[
  #text(size: 18pt, weight: "bold")[Teilnahmebestätigung]
]

#v(1.2cm)

#align(right)[
  #praxis.name\
  #praxis.straße_und_hausnummer\
  #praxis.postleitzahl #praxis.stadt\
  #praxis.ländercode
]

#v(1.5cm)

Hiermit wird bestätigt, dass

#align(center)[
  #text(size: 14pt, weight: "bold")[
    #klient.vorname #klient.nachname
  ]
]

am Seminar

#align(center)[
  #text(size: 14pt, weight: "bold")[#inputs.seminar_titel]
]

teilgenommen hat.

#v(0.8cm)

*Zeitraum:* #datum(inputs.zeitraum_beginn) – #datum(inputs.zeitraum_ende)\
#if inputs.ort_name != none [
  *Ort:* #inputs.ort_name\
]
#if inputs.ort_adresse != none [
  #let oa = inputs.ort_adresse
  *Adresse:* #oa.straße_und_hausnummer, #oa.postleitzahl #oa.stadt
]

#v(2cm)
#praxis.stadt, #datum(inputs.zeitraum_ende)

#v(1.5cm)
#praxis.name
