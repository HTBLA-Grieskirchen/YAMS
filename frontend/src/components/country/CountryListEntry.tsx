import {observer} from "mobx-react";
import Country from "../../model/country";
import CountryDetailItem from "./CountryDetailItem";
import {useState} from "react";
import CountryEditItem from "./CountryEditItem";
import CountryUsageTabs from "./CountryUsageTabs";

const CountryListEntry = observer((
    {country}:
        { country: Country }
) => {
    const [editing, setEditing] = useState(false)

    return <>
        {editing ?
            <CountryEditItem country={country} onCancel={() => setEditing(false)} onConfirm={(s) => {
                if (s) setEditing(false)
            }} noBottomPadding={true}/> :
            <CountryDetailItem country={country} onEdit={() => setEditing(true)} noBottomPadding={true}/>
        }
        <div className="table-row">
            <td colSpan={10} className="px-4 pt-1 pb-2">
                <CountryUsageTabs country={country}/>
            </td>
        </div>
    </>
})

export default CountryListEntry
