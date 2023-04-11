import { Result } from "surrealdb.js";
import { query } from "./index";
import { Record } from "../../model/surreal";
import EventParticipation, { EventParticipationResponse } from "../../model/participation";

export async function relateClientParticipateEvent(
    from: Record,
    dest: Record,
    cost: EventParticipationResponse["data"]["cost"]
): Promise<Result<any>> {
    const response = await query(`
IF count(( SELECT id FROM type::table($participationTable) WHERE in = type::thing($fromTable, $fromID) AND out = type::thing($destTable, $destID) )) < 1 THEN
    ( RELATE (type::thing($fromTable, $fromID))->${EventParticipation.TABLE}->(type::thing($destTable, $destID)) SET cost = type::decimal($cost) )
END;
`, {
        fromTable: from.table,
        fromID: from.id,
        destTable: dest.table,
        destID: dest.id,
        participationTable: EventParticipation.TABLE,
        cost: cost
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}

export async function updateEventParticipation(
    relation: EventParticipation,
    cost: EventParticipationResponse["data"]["cost"]
): Promise<Result<any>> {
    const response = await query(`
UPDATE type::thing($relationTable, $relationID) SET cost = type::decimal($cost);
`, {
        relationTable: relation.record.table,
        relationID: relation.record.id,
        cost: cost
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}

export async function deleteEventParticipation(
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
