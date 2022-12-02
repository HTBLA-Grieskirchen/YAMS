import {observer} from "mobx-react";
import Client from "../../model/client"
import {LiveRefresher} from "../../libs/database";
import {useState} from "react";

const ClientItem = observer(({client, refresh}: { client: Client, refresh: LiveRefresher }) => {
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)
    const [showDetail, setShowDetail] = useState(false)

    const deleteSubmit = async () => {
        setDeleteSubmitted(true)
        //TODO: finish implementing
    }

    return (
        <div className="flex flex-col w-fit" onClick={() => setShowDetail(!showDetail)}>
            <div className="flex flex-row space-x-4 items-center">
                <div className="flex flex-row space-x-2">
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Last Name
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.last_name}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            First Name
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.first_name}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Birth Date
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.birthdate.toLocaleDateString()}
                        </p>
                    </div>
                </div>
                <div className="flex flex-row h-full items-center space-x-1">
                    <button type="button" onClick={e => {
                        deleteSubmit()
                    }} disabled={deleteSubmitted}
                            className="align-text-bottom text-3xl hover:text-4xl hover:text-red-700 text-red-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-remove"/>
                    </button>
                </div>
            </div>
            {showDetail ?
                <div className="bg-white rounded-xl p-2 w-fit mt-2">
                    <div className="border-dashed border-2 rounded-xl border-gray-300 p-2">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            E-Mail
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.email}
                        </p>
                    </div>
                    <div className="">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Consent
                        </label>
                        {client.consent ?
                            <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                                Present
                            </p>
                            :
                            <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                                Not Present
                            </p>
                        }
                    </div>
                    <div className="">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Mobile Number
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.mobile_number}
                        </p>
                    </div>
                </div>
                :
                <div></div>
            }
        </div>
    )
})

export default ClientItem