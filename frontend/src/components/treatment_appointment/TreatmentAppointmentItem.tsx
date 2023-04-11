import {observer} from "mobx-react";
import Animal from "../../model/animal";
import {useState} from "react";

const TreatmentAppointmentItem = observer(({animal}: { animal: Animal }) => {
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)

    const deleteSubmit = async () => {
        setDeleteSubmitted(true)
        //TODO: finish implementing
    }

    return <div className="table-row">
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-900 whitespace-nowrap border-t-2 border-t-gray-200">
            <p className="text-lg xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                {animal.name}
            </p>
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200">
            <p className="text-lg xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                {animal.race}
            </p>
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200">
            {animal.birthdate != null ?
                <p className="text-lg xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                    {animal.birthdate.toLocaleDateString()}
                </p>
                :
                <p className="text-lg xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                    Not Present
                </p>
            }
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200">

        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200">
            <button className="text-red-600 hover:underline disabled:text-red-600/50 disabled:hover:no-underline"
                    disabled={deleteSubmitted}>
                Delete
            </button>
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200">
            <button className="text-blue-600 hover:underline">
                Edit
            </button>
        </div>
    </div>
})

export default TreatmentAppointmentItem