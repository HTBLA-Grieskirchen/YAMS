import React, {useState} from "react"
import {query} from "../libs/database";
import Address from "../model/Address";
import Select from "react-select";
import {observer} from "mobx-react";
import {Result} from "surrealdb.js";

const AddClientForm = observer(({onFinish}: { onFinish: (result: Result<any> | null) => void }) => {
    const [firstname, setFirstname] = useState('')
    const [lastname, setLastname] = useState('')
    const [address, setAddress] = useState<Address | null>(null)
    const [email, setEmail] = useState('')
    const [number, setNumber] = useState('')
    const [street, setStreet] = useState('')
    const [street_number, setStreetNumber] = useState('')
    const [postal_code, setPostalCode] = useState('')
    const [city, setCity] = useState('')
    const [date, setDate] = useState('2000-01-01')

    const [checked, setChecked] = useState(true)
    const [consent, setConsent] = useState(false)

    const handleButtonAddClient = async () => {
        let result: Result<any> | Result<any>[] | null = null
        if (address != null && !(firstname === '') && !(lastname === '') && !(email === '') && !(number === '')) {
            result = await query('CREATE client SET first_name = $firstname, address = type::thing($addressTable, $addressID), ' +
                'consent= $consent, last_name = $lastname, email = $email, mobile_number = $number, birthdate = $birthdate', {
                firstname: firstname,
                lastname: lastname,
                consent: consent,
                addressTable: "address",
                email: email,
                number: number,
                birthdate: new Date(date),
                addressID: address.id
            })

            setFirstname('')
            setLastname('')
            setConsent(false)
            setEmail('')
            setNumber('')
            setDate('2000-01-01')
            setCity('')
            setStreet('')
            setPostalCode('')
            setStreetNumber('')
        } else {
            alert('Felder dürfen nicht leer sein')
        }

        if (result) {
            onFinish(result[0])
        } else {
            onFinish(result)
        }
    }

    const options: any[] = []

    //TODO: improve validation | onSubmit deletes all inputs
    return (
        <div>
            <div className="flex flex-col space-y-2">
                <form>
                    <div className="flex flex-row space-x-4 items-end">
                        <div>
                            <label>
                                <span className="text-gray-700">Vorname</span>
                            </label>
                            <input
                                type="text"
                                className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                placeholder="Max"
                                value={firstname}
                                onChange={e => setFirstname(e.target.value)}
                                required={true}
                            />
                        </div>
                        <div>
                            <label>
                                <span className="text-gray-700">Nachname</span>
                            </label>
                            <input
                                type="text"
                                className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                placeholder="Mustermann"
                                value={lastname}
                                onChange={e => setLastname(e.target.value)}
                                required={true}
                            />
                        </div>
                    </div>
                    <div className="flex flex-row space-x-4 items-end">
                        <div>
                            <label>
                                <span className="text-gray-700">Geburtsdatum</span>
                            </label>
                            <input
                                type="date"
                                className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                value={date}
                                onChange={e => setDate(e.target.value)}
                                required={true}
                            />
                        </div>
                        <div className="text-gray-700">
                            <label>
                                <span className="">Telefonnummer</span>
                            </label>
                            <input
                                type="text"
                                className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                placeholder="+43 123 4567890"
                                value={number}
                                onChange={e => setNumber(e.target.value)}
                                required={true}
                            />
                        </div>
                    </div>
                    <div className="flex flex-row space-x-4 items-end">
                        <div>
                            <label>
                                <span className="text-gray-700">E-Mail</span>
                            </label>
                            <input
                                type="email"
                                className="w-96 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                placeholder="max.mustermann@email.com"
                                value={email}
                                onChange={e => setEmail(e.target.value)}
                                required={true}
                            />
                        </div>
                    </div>
                    <div className="flex flex-row space-x-4 items-end">
                        <div>
                            <label>
                                <span className="text-gray-700">Adresse</span>
                            </label>
                            <div
                                className="text-lg p-1 rounded-lg border-2 border-transparent outline-none focus:border-blue-600 bg-white">
                                <label htmlFor="cboAddress" className="text-gray-700">
                                    Aus vordefinierten Adressen suchen
                                </label>
                                <input
                                    id="cboAddress"
                                    name="cboAddress"
                                    type="checkbox"
                                    className="w-fit ml-2 text-lg p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                    checked={checked}
                                    onChange={e => setChecked(!checked)}
                                />
                            </div>
                        </div>
                    </div>
                    <div className="flex flex-row space-x-4 items-end">
                        {
                            checked ?
                                <Select
                                    className="w-96 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                    classNamePrefix="select"
                                    isClearable={true}
                                    isSearchable={true}
                                    name="addresses"
                                    options={options}
                                    value={address}
                                    onChange={e => setAddress(e)}
                                    required={true}
                                />
                                :
                                <div className="flex flex-col space-y-2">
                                    <div className="flex flex-row space-x-4 items-end">
                                        <div>
                                            <label>
                                                <span className="text-gray-700">Straße</span>
                                            </label>
                                            <input
                                                type="text"
                                                className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                                placeholder="Musterstraße"
                                                value={street}
                                                onChange={e => setStreet(e.target.value)}
                                                required={true}
                                            />
                                        </div>
                                        <div>
                                            <label>
                                                <span className="text-gray-700">Hausnummer</span>
                                            </label>
                                            <input
                                                type="text"
                                                className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                                placeholder="1"
                                                value={street_number}
                                                onChange={e => setStreetNumber(e.target.value)}
                                                required={true}
                                            />
                                        </div>
                                    </div>
                                    <div className="flex flex-row space-x-4 items-end">
                                        <div>
                                            <label>
                                            <span
                                                className="text-gray-700">Postleitzahl</span>
                                            </label>
                                            <input
                                                type="text"
                                                className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                                placeholder="1234"
                                                value={postal_code}
                                                onChange={e => setPostalCode(e.target.value)}
                                                required={true}
                                            />
                                        </div>
                                        <div>
                                            <label>
                                                <span className="text-gray-700">Stadt</span>
                                            </label>
                                            <input
                                                type="text"
                                                className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                                placeholder="Musterstadt"
                                                value={city}
                                                onChange={e => setCity(e.target.value)}
                                                required={true}
                                            />
                                        </div>
                                    </div>
                                </div>
                        }

                    </div>
                    <div className="flex flex-row space-x-4 items-end">
                        <div className="">
                            <label>
                                <span className="text-gray-700">Einverständniserklärung</span>
                            </label>
                            <div
                                className="text-lg p-1 rounded-lg border-2 border-transparent outline-none focus:border-blue-600 bg-white">
                                <label htmlFor="cboConsent" className="text-gray-700">
                                    Einverständniserklärung vorhanden
                                </label>
                                <input
                                    id="cboConsent"
                                    name="cboConsent"
                                    type="checkbox"
                                    className="w-fit ml-2 text-lg p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"
                                    checked={consent}
                                    onChange={e => setConsent(!consent)}
                                />
                            </div>
                        </div>
                    </div>
                    <div className="flex flex-row space-x-4 items-end">
                        <button
                            type="submit"
                            onClick={event => {
                                handleButtonAddClient()
                            }}
                            className="align-text-bottom text-2xl hover:text-3xl hover:text-green-700 text-green-600 w-8 h-8 transition-all">
                            <i className="fa-solid fa-check"/>
                        </button>
                        <button
                            type="button"
                            onClick={event => {
                                handleCancel()
                            }}
                            className="align-text-bottom text-2xl hover:text-3xl hover:text-orange-700 text-orange-600 w-8 h-8 transition-all">
                            <i className="fa-solid fa-cancel"/>
                        </button>
                    </div>
                </form>
            </div>
        </div>
    )
})

export default AddClientForm