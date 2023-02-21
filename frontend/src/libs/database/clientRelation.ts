import { Result } from "surrealdb.js";
import { query } from "./index";
import { Record } from "../../model/surreal";
import ClientRelation, { ClientRelationResponse } from "../../model/relation";

export async function relateClients(
    from: Record,
    dest: Record,
    type: ClientRelationResponse["data"]["relation_type"]
): Promise<Result<any>> {
    const actualType = type.trim()

    const response = await query(`
IF count(( SELECT id FROM type::table($relationTable) WHERE in = type::thing($fromTable, $fromID) AND out = type::thing($destTable, $destID) AND relation_type = $relationType )) < 1 THEN
    ( RELATE (type::thing($fromTable, $fromID))->${ClientRelation.TABLE}->(type::thing($destTable, $destID)) SET relation_type = $relationType )
END;
`, {
        fromTable: from.table,
        fromID: from.id,
        destTable: dest.table,
        destID: dest.id,
        relationTable: ClientRelation.TABLE,
        relationType: actualType
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}

export async function updateClientRelation(
    relation: ClientRelation,
    type: ClientRelationResponse["data"]["relation_type"]
): Promise<Result<any>> {
    const from = relation.from.record
    const dest = relation.dest.record
    const actualType = type.trim()

    const response = await query(`
IF count(( SELECT id FROM type::table($relationTable) WHERE in = type::thing($fromTable, $fromID) AND out = type::thing($destTable, $destID) AND relation_type = $relationType )) < 1 THEN
    ( UPDATE type::thing($relationTable, $relationID) SET relation_type = $relationType )
END;
`, {
        fromTable: from.table,
        fromID: from.id,
        destTable: dest.table,
        destID: dest.id,
        relationTable: relation.table,
        relationID: relation.record.id,
        relationType: actualType
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}

export async function deleteClientRelation(
    relation: Record
): Promise<Result<any>> {
    const response = await query(`
DELETE type::thing($relationTable, $relationID);
`, {
        relationTable: relation.table,
        relationID: relation.id
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}
