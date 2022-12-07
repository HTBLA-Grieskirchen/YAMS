import {observer} from "mobx-react";
import {useState} from "react";
import AddressEditItem from "./AddressEditItem";
import Country from "../../model/country";
import {Record} from "../../model/surreal";
import City from "../../model/city";
import Address from "../../model/address";

const AddressAddItem = observer(() => {
    const [adding, setAdding] = useState(false)

    return <>
        {adding ?
            <AddressEditItem address={
                new Address(new Record(Address.TABLE, "").join(),
                    new City(
                        new Record(City.TABLE, "").join(),
                        new Country(new Record(Country.TABLE, "").join(), "", ""),
                        "", ""),
                    "",
                    ""
                )
            } onCancel={() => setAdding(false)} onConfirm={(s) => {
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

export default AddressAddItem