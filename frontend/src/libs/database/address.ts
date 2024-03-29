import {Result} from "surrealdb.js";
import {query} from "./index";
import Address, {AddressResponse} from "../../model/address";
import {makeRecordForTable} from "../../model/surreal";
import store from "../../stores";

export async function ensureAddress(address: {
    extra: string,
    street: string,
    streetNumber: string,
    postalCode: string,
    city: string,
    country: string
}) {
    const potentialDuplicate = store.addressStore.addresses.find((item) => {
        return (item.extra ?? "") == address.extra.trim() &&
            item.street == address.street.trim() &&
            item.streetNumber == address.streetNumber.trim() &&
            item.postalCode == address.postalCode.trim() &&
            item.city == address.city.trim() &&
            item.country == address.country.trim()
    })

    if (potentialDuplicate !== undefined) {
        return potentialDuplicate.record
    } else {
        const response = await createAddress(
            address.street.trim(),
            address.streetNumber.trim(),
            address.extra.trim(),
            address.postalCode.trim(),
            address.city.trim(),
            address.country.trim()
        )

        if (response.error) {
            throw response.error
        }

        return makeRecordForTable(response.result[0].id, Address.TABLE)
    }
}

export async function deleteAddress(address: Address): Promise<Result<any>> {
    const checkResult = await query("SELECT id FROM client, event WHERE address = type::thing($addressTable, $addressID)", {
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

export async function createAddress(
    street: AddressResponse["data"]["street"],
    streetNumber: AddressResponse["data"]["street_number"],
    extra: AddressResponse["data"]["extra"],
    postalCode: AddressResponse["data"]["postal_code"],
    city: AddressResponse["data"]["city"],
    country: AddressResponse["data"]["country"]
): Promise<Result<any>> {
    const actualStreet = street.trim()
    const actualStreetNumber = streetNumber.trim()
    const actualExtra = !!extra?.trim().length ? extra?.trim() : null
    const actualPostalCode = postalCode.trim()
    const actualCity = city.trim()
    const actualCountry = country.trim()

    const response = await query(`
    CREATE type::table($addressTable) SET country = $country, postal_code = $postalCode, city = $city, street_number = $streetNumber, street = $street, extra = $extra;
`, {
        addressTable: Address.TABLE,
        country: actualCountry,
        postalCode: actualPostalCode,
        city: actualCity,
        streetNumber: actualStreetNumber,
        street: actualStreet,
        extra: actualExtra
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
        UPDATE type::table($addressTable)
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
