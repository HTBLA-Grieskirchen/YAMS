import {Result} from "surrealdb.js";
import {query} from "./index";
import Product from "../../model/product";
import Purchase from "../../model/purchase";

export async function deletePurchase(purchase: Purchase): Promise<Result<any>> {
    // TODO: Add event check once implemented
    const checkResult = await query("SELECT id FROM client WHERE purchase = type::thing($purchaseTable, $purchaseID)", {
        purchaseTable: purchase.record.table,
        purchaseID: purchase.record.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("Purchase is still used in some places")
        }
    }

    const response = await query("DELETE type::thing($purchaseTable, $purchaseID)", {
        purchaseTable: purchase.record.table,
        purchaseID: purchase.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export async function patchPurchase(purchase: Purchase, amount: number, newProduct: Product): Promise<Result<any>> {
    const response = await query(`
IF ( SELECT true FROM type::thing($purchaseTable, $purchaseID) ) THEN
    ( UPDATE type::thing($purchaseTable, $purchaseID) SET amount = $amount, product = type::thing($productTable, $productID) )
ELSE 
    ( CREATE type::table($purchaseTable) SET amount = $amount, product = type::thing($productTable, $productID) )
END
`, {
        purchaseTable: purchase.record.table,
        purchaseID: purchase.record.id,
        productTable: newProduct.record.table,
        productID: newProduct.record.id,
        amount: amount,
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}
