import {observer} from "mobx-react";
import ProductDetailItem from "./ProductDetailItem";
import {useState} from "react";
import ProductEditItem from "./ProductEditItem";
import ProductUsageTabs from "./ProductUsageTabs";
import Product from "../../model/product";

const ProductListEntry = observer((
    {product}:
        { product: Product }
) => {
    const [editing, setEditing] = useState(false)

    return <>
        {editing ?
            <ProductEditItem product={product} onCancel={() => setEditing(false)} onConfirm={(s) => {
                if (s) setEditing(false)
            }} noBottomPadding={true}/> :
            <ProductDetailItem product={product} onEdit={() => setEditing(true)} noBottomPadding={true}/>
        }
        <div className="table-row">
            <td colSpan={10} className="px-4 pt-1 pb-2">
                <ProductUsageTabs product={product}/>
            </td>
        </div>
    </>
})

export default ProductListEntry
