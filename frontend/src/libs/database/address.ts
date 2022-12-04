import {Result} from "surrealdb.js";
import {query} from "./index";
import City from "../../model/city";
import Address from "../../model/address";

export async function deleteAddress(address: Address): Promise<Result<any>> {
    // TODO: Add event check once implemented
    const checkResult = await query("SELECT id FROM client WHERE address = type::thing($addressTable, $addressID)", {
        addressTable: address.record.table,
        addressID: address.record.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("Address is still used in some places")
        }
    }

    const response = await query("DELETE type::thing($addressTable, $addressID)", {
        addressTable: address.record.table,
        addressID: address.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export async function patchAddress(address: Address, newStreet: string, newExtra: string, newCity: City): Promise<Result<any>> {
    const response = await query(`
IF ( SELECT true FROM type::thing($addressTable, $addressID) ) THEN
    ( UPDATE type::thing($addressTable, $addressID) SET street = $street, extra = $extra, city = type::thing($cityTable, $cityID) )
ELSE 
    ( CREATE type::table($addressTable) SET street = $street, extra = $extra, city = type::thing($cityTable, $cityID) )
END
`, {
        addressTable: address.record.table,
        addressID: address.record.id,
        cityTable: newCity.record.table,
        cityID: newCity.record.id,
        street: newStreet,
        extra: newExtra,
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}
