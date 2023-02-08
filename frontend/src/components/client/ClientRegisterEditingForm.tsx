import React, {useState} from "react";
import Address from "../../model/address";
import {query} from "../../libs/database";
import Select from "react-select";
import {observer} from "mobx-react";
import Client from "../../model/client";

const EditClientForm = observer(({client}: {client: Client}) => {
    const [firstname, setFirstname] = useState(client.firstName)
    const [lastname, setLastname] = useState(client.lastName)
    const [email, setEmail] = useState(client.email)
    const [number, setNumber] = useState(client.mobileNumber)
    const [street_number, setStreetNumber] = useState(client.address.streetNumber)
    const [postal_code, setPostalCode] = useState(client.address.postalCode)
    const [city, setCity] = useState(client.address.city)
    const [country, setCountry] = useState(client.address.country)
    const [extra, setExtra] = useState(client.address.extra)
    const [street, setStreet] = useState(client.address.street)
    const [date, setDate] = useState(client.birthdate)

    const [checked, setChecked] = useState(true)
    const [consent, setConsent] = useState(false)

    const handleButtonEditClient = async () => {

        let result = await query('UPDATE client SET first_name = $firstname, last_name = $lastname, email = $email, street_number=$streetNumber, postal_code=$postalCode, city = $city, country=$country, extra=$extra, street=$street, date=$birthdate')

    }

    return (
        <div className="relative flex flex-col justify-center min-h-screen overflow-hidden">
            <div
                className="w-full p-6 m-auto bg-white rounded-md ring-2 ring-indigo-600 lg:max-w-xl">
                <h1 className="text-3xl font-semibold text-center text-indigo-700 underline uppercase">
                    Klientendaten bearbeiten
                </h1>
                <div className="mb-2">
                    <label>
                        <span className="block text-sm font-semibold text-gray-800">Vorname</span>
                    </label>
                    <input
                        type="text"
                        className="block w-full px-4 py-2 mt-2 text-indigo-700 bg-white border rounded-md focus:border-indigo-400 focus:ring-indigo-300 focus:outline-none focus:ring focus:ring-opacity-40"
                        placeholder="Max"
                        value={firstname}
                        onChange={e => setFirstname(e.target.value)}
                    />
                </div>
                <div className="mb-2">
                    <label>
                        <span className="block text-sm font-semibold text-gray-800">Nachname</span>
                    </label>
                    <input
                        type="text"
                        className="block w-full px-4 py-2 mt-2 text-indigo-700 bg-white border rounded-md focus:border-indigo-400 focus:ring-indigo-300 focus:outline-none focus:ring focus:ring-opacity-40"
                        placeholder="Mustermann"
                        value={lastname}
                        onChange={e => setLastname(e.target.value)}
                    />
                </div>
                <div className="mb-2">
                    <label>
                        <span className="block text-sm font-semibold text-gray-800">Geburtsdatum</span>
                    </label>
                    <input
                        type="date"
                        className="block w-full px-4 py-2 mt-2 bg-white border rounded-md text-indigo-700  focus:ring-indigo-300 focus:border-indigo-400 focus:outline-none focus:ring focus:ring-opacity-40 placeholder-indigo-700"
                        value={date.toISOString().substring(0, 10)}
                        onChange={e => console.log(e.target.value) === undefined || setDate(new Date(e.target.value))}
                    />
                </div>
                <div className="mb-2">
                    <label>
                        <span className="block text-sm font-semibold text-gray-800">E-Mail</span>
                    </label>
                    <input
                        type="email"
                        className="block w-full px-4 py-2 mt-2 text-indigo-700 bg-white border rounded-md focus:border-indigo-400 focus:ring-indigo-300 focus:outline-none focus:ring focus:ring-opacity-40"
                        placeholder="max.mustermann@email.com"
                        value={email}
                        onChange={e => setEmail(e.target.value)}
                    />
                </div>
                <div className="mb-2">
                    <label>
                        <span className="block text-sm font-semibold text-gray-800">Telefonnummer</span>
                    </label>
                    <input
                        type="text"
                        className="block w-full px-4 py-2 mt-2 text-indigo-700 bg-white border rounded-md focus:border-indigo-400 focus:ring-indigo-300 focus:outline-none focus:ring focus:ring-opacity-40"
                        placeholder="+43 123 4567890"
                        value={number}
                        onChange={e => setNumber(e.target.value)}
                    />
                </div>
                <div className="mb-2">
                    <label>
                        <span className="block text-sm font-semibold text-gray-800 text-lg">Adresse</span>
                    </label>

                    <div className="mb-2">
                        <div className="w-full">
                            <div className="w-3/4 float-left pr-4">
                                <label>
                                    <span className="block text-sm font-semibold text-gray-800">Straße</span>
                                </label>
                                <input
                                    type="text"
                                    className="block w-full px-4 py-2 mt-2 text-indigo-700 bg-white border rounded-md focus:border-indigo-400 focus:ring-indigo-300 focus:outline-none focus:ring focus:ring-opacity-40"
                                    placeholder="Musterstraße"
                                    value={street}
                                    onChange={e => setStreet(e.target.value)}
                                />
                            </div>
                            <div className="w-1/4 float-left">
                                <label>
                                    <span className="block text-sm font-semibold text-gray-800">Hausnummer</span>
                                </label>
                                <input
                                    type="text"
                                    className="block w-full px-4 py-2 mt-2 text-indigo-700 bg-white border rounded-md focus:border-indigo-400 focus:ring-indigo-300 focus:outline-none focus:ring focus:ring-opacity-40"
                                    placeholder="1"
                                    value={street_number}
                                    onChange={e => setStreetNumber(e.target.value)}
                                />
                            </div>
                        </div>
                        <div className="w-full">
                            <div className="w-1/4 float-left pr-4">
                                <label>
                                    <span className="block text-sm font-semibold text-gray-800">Postleitzahl</span>
                                </label>
                                <input
                                    type="text"
                                    className="block w-full px-4 py-2 mt-2 text-indigo-700 bg-white border rounded-md focus:border-indigo-400 focus:ring-indigo-300 focus:outline-none focus:ring focus:ring-opacity-40"
                                    placeholder="1234"
                                    value={postal_code}
                                    onChange={e => setPostalCode(e.target.value)}
                                />
                            </div>
                            <div className="w-3/4 float-left">
                                <label>
                                    <span className="block text-sm font-semibold text-gray-800">Stadt</span>
                                </label>
                                <input
                                    type="text"
                                    className="block w-full px-4 py-2 mt-2 text-indigo-700 bg-white border rounded-md focus:border-indigo-400 focus:ring-indigo-300 focus:outline-none focus:ring focus:ring-opacity-40"
                                    placeholder="Musterstadt"
                                    value={city}
                                    onChange={e => setCity(e.target.value)}
                                />
                            </div>
                        </div>
                    </div>
                </div>
                <div className="mb-2">
                    <label>
                        <span className="block text-sm font-semibold text-gray-800">Einverständniserklärung</span>
                    </label>
                    <input
                        id="cboConsent"
                        name="cboConsent"
                        type="checkbox"
                        className=""
                        checked={consent}
                        onChange={e => setConsent(!consent)}
                    />
                    <label htmlFor="cboConsent" className="p-2 pb-1 text-sm text-gray-800">
                        Einverständniserklärung vorhanden
                    </label>
                </div>
                <div className="mt-6">
                    <button
                        onClick={event => {
                            handleButtonEditClient()
                        }}
                        className="w-full px-4 py-2 tracking-wide text-white transition-colors duration-200 transform bg-indigo-700 rounded-md hover:bg-indigo-600 focus:outline-none focus:bg-indigo-600">
                        Bestätigen
                    </button>
                </div>
            </div>
        </div>
    )
})

export default EditClientForm