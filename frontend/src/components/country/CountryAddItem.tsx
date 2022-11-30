import {observer} from "mobx-react";
import {useState} from "react";
import CountryEditItem from "./CountryEditItem";
import Country from "../../model/country";
import {Record} from "../../model/surreal";

const CountryAddItem = observer(() => {
    const [adding, setAdding] = useState(false)

    return <>
        {adding ?
            <CountryEditItem country={new Country(new Record(Country.TABLE, "").join(), "", "")}
                             onCancel={() => setAdding(false)} onConfirm={(s) => {
                if (s) {
                    setAdding(false)
                }
            }}/> :
            <div className="table-row">
                <div className="table-cell py-2 px-4">
                    <button
                        className="flex w-6 h-6 bg-gray-300 shadow rounded hover:bg-gray-400/75 hover:shadow-lg transition"
                        onClick={e => setAdding(true)}>
                        <i className="fa-solid fa-add m-auto"/>
                    </button>
                </div>
            </div>
        }
    </>
})

export default CountryAddItem