import Country from "../../model/country";
import {Result} from "surrealdb.js";
import {query} from "./index";
import City from "../../model/city";

export async function deleteCity(city: City): Promise<Result<any>> {
    const checkResult = await query("SELECT id FROM address WHERE city = type::thing($cityTable, $cityID)", {
        cityTable: city.record.table,
        cityID: city.record.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("City is still used in some places")
        }
    }

    const response = await query("DELETE type::thing($cityTable, $cityID)", {
        cityTable: city.record.table,
        cityID: city.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export async function patchCity(city: City, newName: string, newPLZ: string, newCountry: Country): Promise<Result<any>> {
    const response = await query(`
IF ( SELECT true FROM type::thing($cityTable, $cityID) ) THEN
    ( UPDATE type::thing($cityTable, $cityID) SET name = $name, plz = $plz, country = type::thing($countryTable, $countryID) )
ELSE 
    ( CREATE type::table($cityTable) SET name = $name, plz = $plz, country = type::thing($countryTable, $countryID) )
END
`, {
        countryTable: newCountry.record.table,
        countryID: newCountry.record.id,
        cityTable: city.record.table,
        cityID: city.record.id,
        name: newName,
        plz: newPLZ,
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}
