import {observer} from "mobx-react";
import Land from "../model/land";
import {Result} from "surrealdb.js";
import {query} from "../libs/dbConnection";
import {useState} from "react";

const LandListItem = observer(({land, lands, refresh}: { land: Land, lands: Land[], refresh: () => void }) => {
    const [fullName, setFullName] = useState(land.name)
    const [shorthand, setShorthand] = useState(land.short)
    const [editing, setEditing] = useState(false)
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)
    const [changeSubmitted, setChangeSubmitted] = useState(false)
    const [validationError, setValidationError] = useState<string | null>(null)

    const deleteSubmit = async () => {
        setDeleteSubmitted(true)
        if ((await deleteLand(land))?.result) {
            refresh()
        }
    }

    const changeSubmit = async () => {
        const shorthandUpper = shorthand.trim().toUpperCase()
        const fullNameTrimmed = fullName.trim()
        const valError = validate(land.id, fullNameTrimmed, shorthandUpper, lands)
        setValidationError(valError)
        if (valError == null) {
            setChangeSubmitted(true)
            if ((await updateLand(new Land(land.id, fullNameTrimmed, shorthandUpper)))?.result) {
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
                    <label className="text-gray-700 text-sm">Full Name</label>
                    {editing ?
                        <input onChange={e => setFullName(e.target.value)} value={fullName} type="text"
                               placeholder={land.name}
                               className="w-48 p-1
                           text-md font-normal
                           form-control block
                           rounded-lg border-2 border-transparent outline-none
                           transition
                           focus:border-blue-600"/> :
                        <p className="text-lg">{land.name}</p>}
                </div>
                <div className="flex flex-col">
                    <label className="text-gray-700 text-sm">Shorthand</label>
                    {editing ?
                        <input onChange={e => setShorthand(e.target.value)} value={shorthand} type="text"
                               placeholder={land.short}
                               className="w-24 p-1
                           text-md font-normal
                           form-control block
                           rounded-lg border-2 border-transparent outline-none
                           transition
                           focus:border-blue-600"/> :
                        <p className="text-lg">{land.short}</p>}
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
                        setFullName(land.name)
                        setShorthand(land.short)
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

const LandCreation = observer(({lands, onFinish}: { lands: Land[], onFinish: (result: Result<any> | null) => {} }) => {
    const [fullName, setFullName] = useState("")
    const [shorthand, setShorthand] = useState("")
    const [submitted, setSubmitted] = useState("")
    const [validationError, setValidationError] = useState<string | null>(null)

    const onSubmit = async () => {
        const shorthandUpper = shorthand.trim().toUpperCase()
        const fullNameTrimmed = fullName.trim()
        const valError = validate(null, fullNameTrimmed, shorthandUpper, lands)
        setValidationError(valError)
        if (valError == null) {
            setSubmitted(true)
            onFinish(await sendLand(fullNameTrimmed, shorthandUpper))
        }
    }

    return <div className="flex flex-col space-y-2">
        <form onSubmit={e => {
            e.preventDefault()
            onSubmit()
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
                {validationError.split("\n").map((error) => error.trim().length > 0 &&
                    <p className="text-red-600">* {error}</p>)}
            </div>}
    </div>
})

async function sendLand(name: string, short: string): Promise<Result<any>> {
    const response = await query("CREATE land SET name = $name, short = $short", {
        name: name,
        short: short
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

async function updateLand(land: Land): Promise<Result<any>> {
    const response = await query("UPDATE type::thing($landTable, $landID) SET name = $name, short = $short", {
        landTable: Land.TABLE_NAME,
        landID: land.id,
        name: land.name,
        short: land.short
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

async function deleteLand(land: Land): Promise<Result<any>> {
    const checkResult = await query("SELECT * FROM city WHERE land = type::thing($landTable, $landID)", {
        landTable: Land.TABLE_NAME,
        landID: land.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("Land is still used in some cities")
        }
    }

    const response = await query("DELETE type::thing($landTable, $landID)", {
        landTable: Land.TABLE_NAME,
        landID: land.id
    })

    return response[0] ?? {
        error: new Error("No Response at all")
    }
}

function validate(selfID: string, fullName: string, short: string, lands: Land[]): string | null {
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

    if (lands.find((land) => land.short == short && land.id != selfID)) {
        return "That Shorthand already exists"
    }
    return null
}

export {LandListItem, LandCreation}