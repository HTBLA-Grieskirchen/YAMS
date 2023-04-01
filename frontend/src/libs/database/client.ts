import { Result } from "surrealdb.js";
import { query } from "./index";
import Client, { ClientResponse } from "../../model/client";
import { Record } from "../../model/surreal";

export async function createClient(
    firstname: ClientResponse["data"]["first_name"],
    lastname: ClientResponse["data"]["last_name"],
    email: ClientResponse["data"]["email"],
    mobileNumber: ClientResponse["data"]["mobile_number"],
    birthdate: Client["birthdate"],
    consent: ClientResponse["data"]["consent"],
    address: Record
): Promise<Result<any>> {
    const actualFirstname = firstname.trim()
    const actualLastname = lastname.trim()
    const actualEmail = email.trim()
    const actualMobilenumber = mobileNumber.trim()

    const response = await query(`
CREATE type::table($clientTable) SET first_name = $firstname, last_name = $lastname, birthdate = $birthdate, mobile_number = $mobileNumber, email = $email, consent = $consent, address = type::thing($addressTable, $addressID);
`, {
        clientTable: Client.TABLE,
        firstname: actualFirstname,
        lastname: actualLastname,
        birthdate: birthdate,
        mobileNumber: actualMobilenumber,
        email: actualEmail,
        consent: consent,
        addressTable: address.table,
        addressID: address.id
    })



    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}

export async function updateClient(
    target: Record,
    firstname: ClientResponse["data"]["first_name"],
    lastname: ClientResponse["data"]["last_name"],
    email: ClientResponse["data"]["email"],
    mobileNumber: ClientResponse["data"]["mobile_number"],
    birthdate: Client["birthdate"],
    consent: ClientResponse["data"]["consent"],
    address: Record
): Promise<Result<any>> {
    const actualFirstname = firstname.trim()
    const actualLastname = lastname.trim()
    const actualEmail = email.trim()
    const actualMobilenumber = mobileNumber.trim()

    const response = await query(`
UPDATE type::thing($clientTable, $clientID) SET first_name = $firstname, last_name = $lastname, birthdate = $birthdate, mobile_number = $mobileNumber, email = $email, consent = $consent, address = type::thing($addressTable, $addressID);
`, {
        clientTable: target.table,
        clientID: target.id,
        firstname: actualFirstname,
        lastname: actualLastname,
        birthdate: birthdate,
        mobileNumber: actualMobilenumber,
        email: actualEmail,
        consent: consent,
        addressTable: address.table,
        addressID: address.id
    })



    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}