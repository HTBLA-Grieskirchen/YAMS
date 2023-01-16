import {observer} from "mobx-react";
import {useState} from "react";
import ProductTypeEditItem from "./ProductTypeEditItem";
import ProductType from "../../model/productType";
import {Record} from "../../model/surreal";

const ProductTypeAddItem = observer(() => {
    const [adding, setAdding] = useState(false)

    return <>
        {adding ?
            <ProductTypeEditItem productType={new ProductType(new Record(ProductType.TABLE, "").join(), "")}
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

export default ProductTypeAddItem