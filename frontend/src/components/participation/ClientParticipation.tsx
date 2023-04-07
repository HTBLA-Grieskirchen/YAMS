import { observer } from "mobx-react";
import EventParticipation from "../../model/participation";
import Link from "next/link";
import paths from "../../util/paths";
import { useSubmissionState } from "../../libs/form/submit";
import { askEditEventParticipation, submitDeleteEventParticipation } from "./index";

const ClientParticipation = observer((
    {participation}:
        { participation: EventParticipation }
) => {
    const client = participation.from
    const price = participation.cost

    const deleteState = useSubmissionState()
    const updateState = useSubmissionState()

    return <div key={client.record.join()}
                className="group w-full flex place-content-between my-1 last:mb-0 first:mt-0">
        <div className="flex flex-row space-x-4">
            <div className="flex flex-col my-1">
                <div className="flex flex-row">
                    <p className="text-lg font-medium mr-2">{client.firstName} {client.lastName}
                        <span className="font-normal tracking-light opacity-50">
                            , {client.birthdate.getFullYear()}
                            </span>
                    </p>
                </div>
                <div className="flex flex-row py-1">
                    <div className="tooltip tooltip-accent tooltip-right w-fit cursor-help"
                         data-tip={`Message ${client.firstName} ${client.lastName} per E-Mail`}>
                        <a href={`mailto:${client.email}`} className="mr-2 link">
                            <i className="fa-solid fa-paper-plane text-primary/75"/>
                        </a>
                    </div>
                    <p className="text-md grow-0">{client.email}</p>
                </div>
                <div className="flex flex-row">
                    <div className="tooltip tooltip-accent tooltip-right w-fit cursor-help"
                         data-tip={`Call ${client.firstName} ${client.lastName}`}>
                        <a href={`tel:${client.mobileNumber}`} className="mr-2 link">
                            <i className="fa-solid fa-phone text-primary/75"/>
                        </a>
                    </div>
                    <p className="text-md grow-0">{client.mobileNumber}</p>
                </div>
            </div>

            <div className="my-auto text-lg font-bold text-green-600">
                <i className="fa-solid fa-dollar mr-2"/>
                {price}
            </div>
        </div>

        <div className="self-center dropdown dropdown-left group-last:dropdown-end group-last:dropdown-left">
            <label tabIndex={0} className="btn btn-sm btn-ghost m-1">
                <i className="text-lg fa-solid fa-ellipsis"/>
            </label>
            <ul tabIndex={0} className="dropdown-content menu menu-compact shadow-lg bg-base-100 rounded-box w-52">
                <li>
                    <Link href={paths.client(client.record.join())}>
                        <a>
                            <i className="fa-solid fa-magnifying-glass"/>
                            View client
                        </a>
                    </Link>
                </li>
                <li>
                    <a onClick={e => askEditEventParticipation(participation, updateState)}
                       className={updateState.submitted ? "pointer-events-none bg-base-200/50 opacity-50" : ""}>
                        {updateState.submitted ?
                            <i className="fa-solid fa-circle-notch animate-spin"/> :
                            <i className="fa-solid fa-pen text-primary"/>}
                        Edit participation
                    </a>
                </li>
                <li><a onClick={e => submitDeleteEventParticipation(participation, deleteState)}
                       className={deleteState.submitted ? "pointer-events-none bg-base-200/50 opacity-50" : ""}>
                    {deleteState.submitted ?
                        <i className="fa-solid fa-circle-notch animate-spin"/> :
                        <i className="fa-solid fa-trash-can text-error"/>}
                    Remove participant
                </a></li>
            </ul>
        </div>
    </div>
})
export default ClientParticipation
