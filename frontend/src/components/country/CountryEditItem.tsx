import {observer} from "mobx-react";
import Country from "../../model/country";
import {useState} from "react";
import {patchCountry} from "../../libs/database/country";
import notification from "../../libs/notification";
import {useStore} from "../../stores";

const CountryEditItem = observer((
    {country, onConfirm, onCancel}:
        { country: Country, onConfirm?: (success: boolean) => void, onCancel?: () => void }
) => {
    const store = useStore()
    const [code, setCode] = useState(country.short)
    const [name, setName] = useState(country.name)
    const [submitted, setSubmitted] = useState(false)

    const [codeErrors, setCodeErrors] = useState<string[]>([])
    const [nameErrors, setNameErrors] = useState<string[]>([])

    function check(code: string, name: string): [string[], string[]] {
        code = code.toUpperCase()

        const localCodeErrors = []
        const localNameErrors = []

        if (name.trim().length == 0) {
            localNameErrors.push("Name may not be empty")
        }
        if (code.trim().length == 0) {
            localCodeErrors.push("Country code may not be empty")
        }

        if (store.addressStore.countries.find((item) => item.short == code && item.record != country.record)) {
            localCodeErrors.push("That code already exists")
        }

        return [localCodeErrors, localNameErrors]
    }

    function validate(): boolean {
        const [tempCodeErrors, tempNameErrors] = check(code, name)
        setCodeErrors(tempCodeErrors)
        setNameErrors(tempNameErrors)

        return !tempCodeErrors.length && !tempNameErrors.length
    }

    async function submit() {
        setSubmitted(true)

        if (validate()) {

            const result = await patchCountry(country, name, code)
            if (result.error) {
                notification.error({
                    title: "Update Country",
                    message: `${country.name} can not be updated. ${result.error.message}.`
                }, 10, {
                    "Retry": {
                        action: async () => {
                            await submit()
                            return true
                        },
                        disabled: () => submitted
                    }
                })
            } else {
                await store.addressStore.refresh()
            }
            onConfirm && onConfirm(!result.error)

        }

        setSubmitted(false)
    }

    return <form className="table-row group" onSubmit={e => {
        e.preventDefault()
        submit()
    }} onKeyDown={e => {
        if (e.key == "Escape") {
            onCancel && onCancel()
        }
    }}>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-900 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0">
            <input type="text" placeholder="Country Code"
                   className="w-full px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setCode(e.target.value)
                       const [localCodeErrors, localNameErrors] = check(e.target.value, name)
                       setCodeErrors(localCodeErrors)
                       setNameErrors(localNameErrors)
                   }} value={code}/>
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-900 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0">
            <input type="text" placeholder="Country Name" className="px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setName(e.target.value)
                       const [localCodeErrors, localNameErrors] = check(code, e.target.value)
                       setCodeErrors(localCodeErrors)
                       setNameErrors(localNameErrors)
                   }} value={name}/>
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0">
            <div className="flex flex-col text-left">
                {codeErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
                {nameErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
            </div>
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0">
            <button className="text-green-600 hover:underline disabled:opacity-50 disabled:hover:no-underline"
                    type="submit"
                    disabled={submitted || !!check(code, name).reduce((prev, current) => prev + current.length, 0)}>
                Confirm
            </button>
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0">
            <button className="text-red-600 hover:underline" onClick={e => onCancel && onCancel()}>
                Cancel
            </button>
        </div>
    </form>
})

export default CountryEditItem
