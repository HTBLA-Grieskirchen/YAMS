import {observer} from "mobx-react";
import {useState} from "react";
import notification from "../../libs/notification";
import {useStore} from "../../stores";
import Treatment from "../../model/treatment";
import Link from "next/link";
import paths from "../../util/paths";
import {Record} from "../../model/surreal";
import TreatmentAppointment from "../../model/treatmentAppointment";
import {patchTreatmentAppointment} from "../../libs/database/treatmentAppointment";

const TreatmentAppointmentEditItem = observer((
    {treatmentAppointment, onConfirm, onCancel, noBottomPadding}:
        { treatmentAppointment: TreatmentAppointment, onConfirm?: (success: boolean) => void, onCancel?: () => void, noBottomPadding?: boolean }
) => {
    const store = useStore()
    const [cost, setCost] = useState(treatmentAppointment.cost)
    const [date, setDate] = useState(treatmentAppointment.date)
    const [extra, setExtra] = useState(treatmentAppointment.extra)
    const [treatment, setTreatment] = useState(treatmentAppointment.treatment)
    const [submitted, setSubmitted] = useState(false)

    const [costErrors, setCostErrors] = useState<string[]>([])
    const [dateErrors, setDateErrors] = useState<string[]>([])
    const [extraErrors, setExtraErrors] = useState<string[]>([])
    const [treatmentErrors, setTreatmentErrors] = useState<string[]>([])

    const emptyTreatmentID = new Record(Treatment.TABLE, "").join()

    function check(cost: number, date: Date, extra: string, treatment: Treatment): [string[], string[], string[]] {


        const localCostErrors = []
        const localNameErrors: string[] = []
        const localTreatmentErrors = []

        if (cost == 0) {
            localCostErrors.push("Cost may not be 0")
        }
        if (treatment.record.join() == emptyTreatmentID) {
            localTreatmentErrors.push("Treatment has to be selected")
        }

        return [localCostErrors, localNameErrors, localTreatmentErrors]
    }

    function checkset(cost: number, date: Date, extra: string, treatment: Treatment): boolean {
        const [localCostErrors, localExtraErrors, localTreatmentErrors] = check(cost, date, extra, treatment)

        setCostErrors(localCostErrors)
        setExtraErrors(localExtraErrors)
        setTreatmentErrors(localTreatmentErrors)

        return !localCostErrors.length && !localExtraErrors.length && !localTreatmentErrors.length
    }

    function validate(): boolean {
        return checkset(cost, date, extra, treatment)
    }

    async function submit() {
        setSubmitted(true)

        if (validate()) {
            const result = await patchTreatmentAppointment(treatmentAppointment, cost, extra, treatment)
            if (result.error) {
                notification.error({
                    title: "Update TreatmentAppointment",
                    message: `${treatmentAppointment.cost} can not be updated. ${result.error.message}.`
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
                await store.treatmentAppointmentStore.refresh()
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
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-900 border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <input type="text" placeholder="Cost and number"
                   className="w-64 px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setCost(Number.parseFloat(e.target.value))
                       checkset(Number.parseFloat(e.target.value), date, extra, treatment)
                   }} value={cost}/>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-md font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <input type="text" placeholder="Extra information"
                   className="w-40 px-1 shadow-sm rounded border border-gray-200"
                   onChange={e => {
                       setExtra(e.target.value)
                       checkset(cost, date, e.target.value, treatment)
                   }} value={extra}/>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col">
                <p className={`max-w-prose truncate text-ellipsis ${treatment.record.id.trim() ? "text-gray-800" : "text-red-600/75"}`}>
                    {treatment.record.id.trim() ? treatment.description : "Nothing selected"}
                </p>
                {store.treatmentAppointmentStore.treatments.length > 0 ?
                    <select className="form-select form-select-sm
                        block
                        w-fit pr-4 m-0
                        text-sm font-normal text-gray-700
                        bg-white bg-clip-padding bg-no-repeat
                        border border-solid border-gray-300 rounded
                        transition ease-in-out
                        focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none"
                            placeholder="Select a treatment" onChange={e => {
                        let treatment = store.treatmentAppointmentStore.treatments.find((treatment) => treatment.record.join() == e.target.value)!!
                        setTreatment(treatment)
                        checkset(cost, date, extra, treatment)
                    }} value={treatment.record.join()}>
                        {emptyTreatmentID == treatment.record.join() &&
                            <option value={emptyTreatmentID} hidden>Select a treatment</option>}
                        {store.treatmentAppointmentStore.treatments.map((treatment) => <option key={treatment.record.join()}
                                                                         value={treatment.record.join()}>
                                {treatment.description}
                            </option>
                        )}
                    </select> :
                    <Link href={paths.treatments}><a className="
                        w-fit px-2
                        font-bold text-gray-800 hover:text-gray-900
                        rounded
                        shadow-sm hover:shadow-md
                        bg-gray-300 hover:bg-gray-200
                        transition">
                        Start to add treatments
                    </a></Link>
                }
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-600 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            {treatment.record.join() == emptyTreatmentID ?
                <p className={`max-w-prose truncate text-ellipsis text-red-600/75`}>
                    Select a treatment
                </p> :
                <div className="flex flex-col">
                    <p className="max-w-prose truncate text-ellipsis text-gray-800">{treatment.treatment_type.description}</p>
                </div>
            }
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="flex flex-col text-left">
                {costErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
                {extraErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
                {treatmentErrors.map((error, index) =>
                    <p key={index} className="text-red-600"><i className="fa-solid fa-circle-exclamation"/> {error}
                    </p>)}
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="w-16">
                <button className="text-green-600 hover:underline disabled:opatreatment-50 disabled:hover:no-underline"
                        type="submit"
                        disabled={submitted || !!check(cost, date, extra, treatment).reduce((prev, current) => prev + current.length, 0)}>
                    Confirm
                </button>
            </div>
        </div>
        <div
            className={`table-cell ${noBottomPadding ? "pt-2" : "py-2"} px-2 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200 group-first:border-t-0`}>
            <div className="w-16">
                <button className="text-red-600 hover:underline" onClick={e => onCancel && onCancel()}>
                    Cancel
                </button>
            </div>
        </div>
    </form>
})

export default TreatmentAppointmentEditItem
