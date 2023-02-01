import {Result} from "surrealdb.js";
import {query} from "./index";
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

export async function patchAddress(address: Address, newStreet: string, newExtra: string): Promise<Result<any>> {
    const response = await query(`
IF ( SELECT true FROM type::thing($addressTable, $addressID) ) THEN
    ( UPDATE type::thing($addressTable, $addressID) SET street = $street, extra = $extra, city = type::thing($cityTable, $cityID) )
ELSE 
    ( CREATE type::table($addressTable) SET street = $street, extra = $extra, city = type::thing($cityTable, $cityID) )
END
`, {
        addressTable: address.record.table,
        addressID: address.record.id,
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

export async function patchAddressesDynamic(
    prev: { [field: string]: string }, next: { [field: string]: string }
): Promise<Result<any>> {
    const prefixedPrev = Object.entries(prev).map(([field, value]) => [`prev_${field}`, value])
    const prefixedNext = Object.entries(next).map(([field, value]) => [`next_${field}`, value])

    const prevRequirements = Object.keys(prev).map((field, index) => `${field} = $${prefixedPrev[index][0]}`).join(" AND ")
    const nextValues = Object.keys(next).map((field, index) => `${field} = $${prefixedNext[index][0]}`).join(", ")

    const response = await query(`
        UPDATE type:: table ($addressTable)
        SET ${nextValues}
        WHERE ${prevRequirements}
    `, {...Object.fromEntries(prefixedPrev), ...Object.fromEntries(prefixedNext), addressTable: Address.TABLE})

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}
