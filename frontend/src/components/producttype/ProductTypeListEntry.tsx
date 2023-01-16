import {observer} from "mobx-react";
import ProductType from "../../model/productType";
import ProductTypeDetailItem from "./ProductTypeDetailItem";
import {useState} from "react";
import ProductTypeEditItem from "./ProductTypeEditItem";
import ProductTypeUsageTabs from "./ProductTypeUsageTabs";

const ProductTypeListEntry = observer((
    {productType}:
        { productType: ProductType }
) => {
    const [editing, setEditing] = useState(false)

    return <>
        {editing ?
            <ProductTypeEditItem productType={productType} onCancel={() => setEditing(false)} onConfirm={(s) => {
                if (s) setEditing(false)
            }} noBottomPadding={true}/> :
            <ProductTypeDetailItem productType={productType} onEdit={() => setEditing(true)} noBottomPadding={true}/>
        }
        <div className="table-row">
            <td colSpan={10} className="px-4 pt-1 pb-2">
                <ProductTypeUsageTabs productType={productType}/>
            </td>
        </div>
    </>
})

export default ProductTypeListEntry
