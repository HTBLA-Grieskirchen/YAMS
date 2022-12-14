import {observer} from "mobx-react";
import CityDetailItem from "./CityDetailItem";
import {useState} from "react";
import CityEditItem from "./CityEditItem";
import CityUsageTabs from "./CityUsageTabs";
import City from "../../model/city";

const CityListEntry = observer((
    {city}:
        { city: City }
) => {
    const [editing, setEditing] = useState(false)

    return <>
        {editing ?
            <CityEditItem city={city} onCancel={() => setEditing(false)} onConfirm={(s) => {
                if (s) setEditing(false)
            }} noBottomPadding={true}/> :
            <CityDetailItem city={city} onEdit={() => setEditing(true)} noBottomPadding={true}/>
        }
        <div className="table-row">
            <td colSpan={10} className="px-4 pt-1 pb-2">
                <CityUsageTabs city={city}/>
            </td>
        </div>
    </>
})

export default CityListEntry
