import Animal from "../../model/animal"
import {Result} from "surrealdb.js"
import {query} from "./index"
import Race from "../../model/race";

export async function deleteAnimal(animal: Animal): Promise<Result<any>> {
    const checkResult = await query("SELECT * FROM client WHERE type::thing($animalTable, $animalID) IN animals", {
        animalTable: animal.record.table,
        animalID: animal.record.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("Animal is still used in some places")
        }
    }

    const response = await query("DELETE type::thing($animalTable, $animalID)", {
        animalTable: animal.record.table,
        animalID: animal.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export async function patchAnimal(animal: Animal | null, newBirthdate: Date, newName: string, race: Race): Promise<Result<any>> {
    let response;
    if (animal != null) {
        response = await query(`
        IF ( SELECT true FROM type::thing($animalTable, $animalID) ) THEN
            ( UPDATE type::thing($animalTable, $animalID) SET birthdate = $newBirthdate, name = $newName, race = type::thing($raceTable, $raceID) )
        ELSE 
            ( CREATE type::table($animalTable) SET birthdate = $newBirthdate, name = $newName, race = type::thing($raceTable, $raceID) )
        END
`, {
            raceTable: race.record.table,
            raceID: race.record.id,
            animalTable: animal.record.table,
            animalID: animal.record.id,
            newBirthdate: newBirthdate,
            newName: newName,
        })
    } else {
        response = await query(`
        CREATE type::table($animalTable) SET birthdate = $newBirthdate, name = $newName, race = type::thing($raceTable, $raceID)
        `, {
            raceTable: race.record.table,
            raceID: race.record.id,
            animalTable: Animal.TABLE,
            newBirthdate: newBirthdate,
            newName: newName,
        })
    }

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}