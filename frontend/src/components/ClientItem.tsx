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
        <div>

        </div>
    )
})