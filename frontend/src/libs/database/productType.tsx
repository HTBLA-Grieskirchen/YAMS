import ProductType from "../../model/productType";
import {Result} from "surrealdb.js";
import {query} from "./index";

export async function deleteProductType(productType: ProductType): Promise<Result<any>> {
    const checkResult = await query("SELECT * FROM product WHERE productType = type::thing($productTypeTable, $productTypeID)", {
        productTypeTable: productType.record.table,
        productTypeID: productType.record.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("ProductType is still used in some places")
        }
    }

    const response = await query("DELETE type::thing($productTypeTable, $productTypeID)", {
        productTypeTable: productType.record.table,
        productTypeID: productType.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export async function patchProductType(productType: ProductType, newName: string): Promise<Result<any>> {
    const shortTakenError = 2

    const response = await query(`
IF ( count(( SELECT id FROM product_type WHERE name = $name AND id != type::thing($productTypeTable, $productTypeID) )) > 0 ) THEN
    $nameError
ELSE IF ( SELECT true FROM type::thing($productTypeTable, $productTypeID) ) THEN
    ( UPDATE type::thing($productTypeTable, $productTypeID) SET name = $name)
ELSE 
    ( CREATE type::table($productTypeTable) SET name = $name)
END
`, {
        productTypeTable: productType.record.table,
        productTypeID: productType.record.id,
        name: newName,
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    switch (response[0].result) {
        case shortTakenError:
            return {
                error: new Error("That code is already used")
            }
    }

    return response[0]
}
