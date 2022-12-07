import Country from "../../model/country";
import {Result} from "surrealdb.js";
import {query} from "./index";

export async function deleteCountry(country: Country): Promise<Result<any>> {
    const checkResult = await query("SELECT * FROM city WHERE country = type::thing($landTable, $landID)", {
        landTable: country.record.table,
        landID: country.record.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("Country is still used in some places")
        }
    }

    const response = await query("DELETE type::thing($landTable, $landID)", {
        landTable: country.record.table,
        landID: country.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export async function patchCountry(country: Country, newName: string, newShort: string): Promise<Result<any>> {
    const shortTakenError = 2

    const response = await query(`
IF ( count(( SELECT id FROM country WHERE short = $short AND id != type::thing($countryTable, $countryID) )) > 0 ) THEN
    $shortError
ELSE IF ( SELECT true FROM type::thing($countryTable, $countryID) ) THEN
    ( UPDATE type::thing($countryTable, $countryID) SET name = $name, short = $short )
ELSE 
    ( CREATE type::table($countryTable) SET name = $name, short = $short )
END
`, {
        countryTable: country.record.table,
        countryID: country.record.id,
        name: newName,
        short: newShort.toUpperCase(),
        shortError: shortTakenError
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    switch (response[0].result) {
        case shortTakenError:
            return {
                error: new Error("That code is already used")
            }
    }

    return response[0]
}
