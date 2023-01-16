import {observer} from "mobx-react";
import PurchaseDetailItem from "./PurchaseDetailItem";
import {useState} from "react";
import PurchaseEditItem from "./PurchaseEditItem";
import PurchaseUsageTabs from "./PurchaseUsageTabs";
import Purchase from "../../model/purchase";

const PurchaseListEntry = observer((
    {purchase}:
        { purchase: Purchase }
) => {
    const [editing, setEditing] = useState(false)

    return <>
        {editing ?
            <PurchaseEditItem purchase={purchase} onCancel={() => setEditing(false)} onConfirm={(s) => {
                if (s) setEditing(false)
            }} noBottomPadding={true}/> :
            <PurchaseDetailItem purchase={purchase} onEdit={() => setEditing(true)} noBottomPadding={true}/>
        }
        <div className="table-row">
            <td colSpan={10} className="px-4 pt-1 pb-2">
                <PurchaseUsageTabs purchase={purchase}/>
            </td>
        </div>
    </>
})

export default PurchaseListEntry
