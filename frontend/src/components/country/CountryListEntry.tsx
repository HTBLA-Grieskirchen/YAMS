import {observer} from "mobx-react";
import Country from "../../model/country";
import CountryDetailItem from "./CountryDetailItem";
import {useState} from "react";
import CountryEditItem from "./CountryEditItem";

const CountryListEntry = observer((
    {country}:
        { country: Country }
) => {
    const [editing, setEditing] = useState(false)

    return <>{editing ?
        <CountryEditItem country={country} onCancel={() => setEditing(false)} onConfirm={(s) => {
            if (s) setEditing(false)
        }}/> :
        <CountryDetailItem country={country} onEdit={() => setEditing(true)}/>
    }</>
})

export default CountryListEntry
