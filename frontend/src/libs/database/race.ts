import Race from "../../model/race"
import {Result} from "surrealdb.js"
import {query} from "./index"

export async function deleteRace(race: Race): Promise<Result<any>> {
    const checkResult = await query("SELECT * FROM animal WHERE race = type::thing($raceTable, $raceID)", {
        raceTable: race.record.table,
        raceID: race.record.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("Race is still used in some places")
        }
    }

    const response = await query("DELETE type::thing($raceTable, $raceID)", {
        raceTable: race.record.table,
        raceID: race.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export async function patchRace(race: Race, newDescription: string, newSpecies: string): Promise<Result<any>> {
    const response = await query(`
        IF ( SELECT true FROM type::thing($raceTable, $raceID) ) THEN 
            ( CREATE type::table($raceTable) SET description = $newDescription, animal_species = $newSpecies )
        ELSE
            ( CREATE type::table($raceTable) SET description = $newDescription, animal_species = $newSpecies )            
        END
`, {
        raceTable: race.record.table,
        raceID: race.record.id,
        newDescription: newDescription,
        newSpecies: newSpecies,
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}