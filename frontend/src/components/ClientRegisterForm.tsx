import React, {useState} from "react"
import Select from 'react-select'
import {query} from "../libs/dbConnection";
import Address from "../model/address";

const AddClientForm: any = () => {
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

    //const [file, setFile] = useState(null)
    //const [fileName, setFileName] = useState(null)

    /*const fileToBase64 = (file: any, cb: any) => {
        const reader = new FileReader()
        reader.readAsDataURL(file)
        reader.onload = function () {
            cb(null, reader.result)
        }
        reader.onerror = function (error) {
            cb(error, null)
        }
    }*/

    /*const onUploadFileChange = ({ target }: { target: any }) => {
        if (target.files < 1 || !target.validity.valid) {
            return
        }
        fileToBase64(target.files[0], (err: any, result: any) => {
            if (result) {
                setFile(result)
                setFileName(target.files[0])
            }
        })
    }*/

    const handleButtonAddClient = async () => {
        if (address != null && !(firstname === '') && !(lastname === '') && !(email === '') && !(number === '')) {
            let result = await query('CREATE client SET first_name = $firstname, address = type::thing($addressTable, $addressID), ' +
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
    }

    const options = [
        new Address('address:6thd5c3wb2dnckpz65yk', 'test', 'Test'),
        new Address('address:6thd5c3wb2dnckpz65yk', 'test2', 'Test2')
    ]

    return (
        <div className="relative flex flex-col justify-center min-h-screen overflow-hidden">
            <div
                className="w-full p-6 m-auto bg-white rounded-md shadow-xl shadow-rose-600/40 ring-2 ring-indigo-600 lg:max-w-xl">
                <h1 className="text-3xl font-semibold text-center text-indigo-700 underline uppercase">
                    Clientendaten anlegen
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
                        value={date}
                        onChange={e => setDate(e.target.value)}
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
                        <span className="block text-sm font-semibold text-gray-800">Adresse</span>
                    </label>
                    <input
                        id="cboAddress"
                        name="cboAddress"
                        type="checkbox"
                        className=""
                        checked={checked}
                        onChange={e => setChecked(!checked)}
                    />
                    <label htmlFor="cboAddress" className="p-2 pb-1 text-sm text-gray-800">
                        Aus vordefinierten Adressen suchen
                    </label>
                    {
                        checked ?
                            <Select
                                className="p-2 pb-1 text-sm text-indigo-700"
                                classNamePrefix="select"
                                isClearable={true}
                                isSearchable={true}
                                name="addresses"
                                options={options}
                                value={address}
                                onChange={e => setAddress(e)}
                            />
                            :
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
                                            <span
                                                className="block text-sm font-semibold text-gray-800">Hausnummer</span>
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
                                            <span
                                                className="block text-sm font-semibold text-gray-800">Postleitzahl</span>
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
                    }
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
                            handleButtonAddClient()
                        }}
                        className="w-full px-4 py-2 tracking-wide text-white transition-colors duration-200 transform bg-indigo-700 rounded-md hover:bg-indigo-600 focus:outline-none focus:bg-indigo-600">
                        Anlegen
                    </button>
                </div>
            </div>
        </div>
    )
}

export default AddClientForm