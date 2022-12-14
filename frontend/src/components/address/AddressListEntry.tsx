import {observer} from "mobx-react";
import AddressDetailItem from "./AddressDetailItem";
import {useState} from "react";
import AddressEditItem from "./AddressEditItem";
import AddressUsageTabs from "./AddressUsageTabs";
import Address from "../../model/address";

const AddressListEntry = observer((
    {address}:
        { address: Address }
) => {
    const [editing, setEditing] = useState(false)

    return <>
        {editing ?
            <AddressEditItem address={address} onCancel={() => setEditing(false)} onConfirm={(s) => {
                if (s) setEditing(false)
            }} noBottomPadding={true}/> :
            <AddressDetailItem address={address} onEdit={() => setEditing(true)} noBottomPadding={true}/>
        }
        <div className="table-row">
            <td colSpan={10} className="px-4 pt-1 pb-2">
                <AddressUsageTabs address={address}/>
            </td>
        </div>
    </>
})

export default AddressListEntry
