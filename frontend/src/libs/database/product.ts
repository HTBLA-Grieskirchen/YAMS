import ProductType from "../../model/productType";
import {Result} from "surrealdb.js";
import {query} from "./index";
import Product from "../../model/product";

export async function deleteProduct(product: Product): Promise<Result<any>> {
    const checkResult = await query("SELECT id FROM purchase WHERE product = type::thing($productTable, $productID)", {
        productTable: product.record.table,
        productID: product.record.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("City is still used in some places")
        }
    }

    const response = await query("DELETE type::thing($productTable, $productID)", {
        productTable: product.record.table,
        productID: product.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export async function patchProduct(product: Product, newName: string, newProductType: ProductType, price: number): Promise<Result<any>> {
    const response = await query(`
IF ( SELECT true FROM type::thing($productTable, $productID) ) THEN
    ( UPDATE type::thing($productTable, $productID) SET name = $name, price = $price, productType = type::thing($productTypeTable, $productTypeID) )
ELSE 
    ( CREATE type::table($productTable) SET name = $name, price = $price, productType = type::thing($productTypeTable, $productTypeID) )
END
`, {
        productTypeTable: newProductType.record.table,
        productTypeID: newProductType.record.id,
        productTable: product.record.table,
        productID: product.record.id,
        name: newName,
        price: price
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}
