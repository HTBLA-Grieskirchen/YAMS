import {Result} from "surrealdb.js";
import {query} from "./index";
import {Record} from "../../model/surreal";
import ClientRelation, {ClientRelationResponse} from "../../model/relation";

export async function relateClients(
    from: Record,
    dest: Record,
    type: ClientRelationResponse["data"]["relation_type"]
): Promise<Result<any>> {
    const actualType = type.trim()

    const response = await query(`
RELATE (type::thing($fromTable, $fromID))->${ClientRelation.TABLE}->(type::thing($destTable, $destID)) SET relation_type = $relationType;
`, {
        fromTable: from.table,
        fromID: from.id,
        destTable: dest.table,
        destID: dest.id,
        relationType: actualType
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}