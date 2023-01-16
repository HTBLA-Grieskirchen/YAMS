import {observer} from "mobx-react";
import {useState} from "react";
import ProductEditItem from "./ProductEditItem";
import ProductType from "../../model/productType";
import {Record} from "../../model/surreal";
import Product from "../../model/product";

const ProductAddItem = observer(() => {
    const [adding, setAdding] = useState(false)

    return <>
        {adding ?
            <ProductEditItem product={
                new Product(
                    new Record(Product.TABLE, "").join(),
                    new Product(new Record(Product.TABLE, "").join(), "", ""),
                    "", "")
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

export default ProductAddItem