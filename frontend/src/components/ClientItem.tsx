import {observer} from "mobx-react";
import Client from "../model/client"
import {LiveRefresher} from "../libs/database";
import {useState} from "react";

const ClientItem = observer(({client, refresh}: { client: Client, refresh: LiveRefresher }) => {
    const [lastName, setLastName] = useState(client.lastName)
    const [firstName, setFirstName] = useState(client.firstName)
    const [birthDate, setBirthDate] = useState(client.birthdate)
    const [email, setEmail] = useState(client.email)
    const [consent, setConsent] = useState(client.consent)
    const [telNumber, setTelNumber] = useState(client.mobileNumber)
    //TODO: Add address information

    const [deleteSubmitted, setDeleteSubmitted] = useState(false)

    const deleteSubmit = async () => {
        setDeleteSubmitted(true)
        //TODO: finish implementing
    }

    return (
        <div className="flex flex-col">
            <div className="flex flex-row space-x-6 items-center">
                <div className="flex flex-row space-x-4">
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Last Name
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {client.lastName}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    )
})