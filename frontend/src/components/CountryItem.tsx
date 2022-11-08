import {observer} from "mobx-react";
import Country from "../model/country";
import {Result} from "surrealdb.js";
import {LiveRefresher, query} from "../libs/database";
import {useState} from "react";
import {makeRecord} from "../model/surreal";

const CountryListItem = observer(({
                                      country,
                                      countries,
                                      refresh
                                  }: { country: Country, countries: Country[], refresh: LiveRefresher }) => {
    const [fullName, setFullName] = useState(country.name)
    const [shorthand, setShorthand] = useState(country.short)
    const [editing, setEditing] = useState(false)
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)
    const [changeSubmitted, setChangeSubmitted] = useState(false)
    const [validationError, setValidationError] = useState<string | null>(null)

    const deleteSubmit = async () => {
        setDeleteSubmitted(true)
        if ((await deleteCountry(country))?.result) {
            refresh()
        }
    }

    const changeSubmit = async () => {
        const shorthandUpper = shorthand.trim().toUpperCase()
        const fullNameTrimmed = fullName.trim()
        const valError = await validate(country.record.join(), fullNameTrimmed, shorthandUpper, countries)
        setValidationError(valError)
        if (valError == null) {
            setChangeSubmitted(true)
            if ((await updateCountry(new Country(country.record.join(), fullNameTrimmed, shorthandUpper)))?.result) {
                setChangeSubmitted(false)
                setEditing(false)
                refresh()
                setFullName(fullNameTrimmed)
                setShorthand(shorthandUpper)
            }
        }
    }

    return <div className="flex flex-col">
        <div className="flex flex-row space-x-6 items-center">
            <div className="flex flex-row space-x-4">
                <div className="flex flex-col">
                    <label className="text-gray-700 text-sm sm:w-48 w-fit">Full Name</label>
                    {editing ?
                        <input onChange={e => setFullName(e.target.value)} value={fullName} type="text"
                               placeholder={country.name}
                               className="w-48 p-1
                           text-md font-normal
                           form-control block
                           rounded-lg border-2 border-transparent outline-none
                           transition
                           focus:border-blue-600"/> :
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">{country.name}</p>}
                </div>
                <div className="flex flex-col">
                    <label className="text-gray-700 text-sm">Shorthand</label>
                    {editing ?
                        <input onChange={e => setShorthand(e.target.value)} value={shorthand} type="text"
                               placeholder={country.short}
                               className="w-24 p-1
                           text-md font-normal
                           form-control block
                           rounded-lg border-2 border-transparent outline-none
                           transition
                           focus:border-blue-600"/> :
                        <p className="text-lg">{country.short}</p>}
                </div>
            </div>


            {editing ?
                <form onSubmit={e => {
                    e.preventDefault()
                    changeSubmit()
                }} className="flex flex-row h-full self-end pb-1 space-x-1">
                    <button type="submit" disabled={changeSubmitted}
                            className="align-text-bottom text-3xl hover:text-4xl hover:text-green-700 text-green-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-check"/>
                    </button>
                    <button type="button" onClick={e => {
                        setFullName(country.name)
                        setShorthand(country.short)
                        setValidationError(null)
                        setEditing(false)
                        setChangeSubmitted(false)
                    }}
                            className="align-text-bottom text-2xl hover:text-3xl hover:text-orange-700 text-orange-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-cancel"/>
                    </button>
                </form> :
                <div className="flex flex-row h-full items-center space-x-1">
                    <button type="button" onClick={e => {
                        setEditing(true)
                    }} disabled={deleteSubmitted}
                            className="align-text-bottom text-2xl hover:text-3xl hover:text-gray-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-edit"/>
                    </button>
                    <button type="button" onClick={e => {
                        deleteSubmit()
                    }} disabled={deleteSubmitted}
                            className="align-text-bottom text-3xl hover:text-4xl hover:text-red-700 text-red-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-remove"/>
                    </button>
                </div>}
        </div>
        {validationError &&
            <div className="flex flex-col space-y-0">
                {validationError.split("\n").map((error) => error.trim().length > 0 &&
                    <p className="text-red-600">* {error}</p>)}
            </div>}
    </div>
})

const CountryCreation = observer(({
                                      countries,
                                      onFinish
                                  }: { countries: Country[], onFinish: (result: Result<any> | null) => void }) => {
    const [fullName, setFullName] = useState("")
    const [shorthand, setShorthand] = useState("")
    const [submitted, setSubmitted] = useState(false)
    const [validationError, setValidationError] = useState<string | null>(null)

    const onSubmit = async () => {
        const shorthandUpper = shorthand.trim().toUpperCase()
        const fullNameTrimmed = fullName.trim()
        const valError = await validate("", fullNameTrimmed, shorthandUpper, countries)
        setValidationError(valError)
        if (valError == null) {
            setSubmitted(true)
            onFinish(await sendCountry(fullNameTrimmed, shorthandUpper))
        }
    }

    return <div className="flex flex-col space-y-2">
        <form onSubmit={e => {
            e.preventDefault()
            onSubmit()
        }} onKeyDown={e => {
            if (e.key == "Escape") {
                onFinish(null)
            }
        }}>
            <div className="flex flex-row space-x-4 items-end">
                <div className="flex flex-col space-y-1">
                    <label className="text-gray-700">Full Name</label>
                    <input onChange={e => setFullName(e.target.value)} value={fullName} type="text"
                           placeholder="Austria"
                           className="w-64 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"/>
                </div>
                <div className="flex flex-col space-y-1">
                    <label className="text-gray-700">Shorthand</label>
                    <input onChange={e => setShorthand(e.target.value)} value={shorthand} type="text" placeholder="AT"
                           className="w-32 text-lg form-control block p-1 font-normal rounded-lg border-2 border-transparent outline-none transition focus:border-blue-600"/>
                </div>
                <button type="submit" disabled={submitted}
                        className="align-text-bottom text-3xl hover:text-4xl hover:text-green-700 text-green-600 w-8 h-8 transition-all">
                    <i className="fa-solid fa-check"/>
                </button>
                <button onClick={e => {
                    onFinish(null)
                }}
                        className="align-text-bottom text-2xl hover:text-3xl hover:text-orange-700 text-orange-600 w-8 h-8 transition-all">
                    <i className="fa-solid fa-cancel"/>
                </button>
            </div>
        </form>
        {validationError &&
            <div className="flex flex-col space-y-0">
                {validationError.split("\n").map((error, index) => error.trim().length > 0 &&
                    <p key={index} className="text-red-600">* {error}</p>)}
            </div>}
    </div>
})

async function sendCountry(name: string, short: string): Promise<Result<any>> {
    const response = await query("CREATE country SET name = $name, short = $short", {
        name: name,
        short: short
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

async function updateCountry(country: Country): Promise<Result<any>> {
    const response = await query("UPDATE type::thing($landTable, $landID) SET name = $name, short = $short", {
        landTable: country.record.table,
        landID: country.record.id,
        name: country.name,
        short: country.short
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

async function deleteCountry(country: Country): Promise<Result<any>> {
    const checkResult = await query("SELECT * FROM city WHERE country = type::thing($landTable, $landID)", {
        landTable: country.record.table,
        landID: country.record.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("Country is still used in some cities")
        }
    }

    const response = await query("DELETE type::thing($landTable, $landID)", {
        landTable: country.record.table,
        landID: country.record.id
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

async function validate(selfID: string, fullName: string, short: string, countries: Country[]): Promise<string | null> {
    let error = ""
    if (fullName.trim().length == 0) {
        error += "Full Name may not be empty\n"
    }
    if (short.trim().length == 0) {
        error += "Shorthand may not be empty\n"
    }

    if (error.length != 0) {
        return error
    }

    if (countries.find((land) => land.short == short && land.record != makeRecord(selfID))) {
        return "That Shorthand already exists"
    } else if ((await query("SELECT * FROM country WHERE short = $short", {
        short: short
    }))[0]?.result.length > 0) {
        return "That Shorthand already exists"
    }
    return null
}

export {CountryListItem, CountryCreation}