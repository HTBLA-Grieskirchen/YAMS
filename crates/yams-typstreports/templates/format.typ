/// German locale helpers used by the report templates.
#let datum(dt) = dt.display("[day].[month].[year]")

#let euro(n) = {
  let rounded = calc.round(n, digits: 2)
  str(rounded).replace(".", ",") + " €"
}

#let prozent(ratio) = {
  str(calc.round(ratio * 100, digits: 0)) + " %"
}
