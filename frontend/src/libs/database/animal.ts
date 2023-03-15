import Animal from "../../model/animal"
import {Result} from "surrealdb.js"
import {query} from "./index"
import Race from "../../model/race";

export async function deleteAnimal(animal: Animal): Promise<Result<any>> {
    const removeInClientArray = await query("UPDATE type::thing($animalTable, $animalID) SET animals = animals[where id != type::thing($animalTable, $animalID)]", {
        animalTable: animal.record.table,
        animalID: animal.record.id
    })
    const response = await query("DELETE type::thing($animalTable, $animalID)", {
        animalTable: animal.record.table,
        animalID: animal.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export async function patchAnimal(animal: Animal | null, newBirthdate: Date, newName: string, raceID: string): Promise<Result<any>> {
    let response;
    if (animal != null) {
        response = await query(`
        IF ( SELECT true FROM type::thing($animalTable, $animalID) ) THEN
            ( UPDATE type::thing($animalTable, $animalID) SET birthdate = $newBirthdate, name = $newName, race = type::thing($raceTable, $raceID) )
        ELSE 
            ( CREATE type::table($animalTable) SET birthdate = $newBirthdate, name = $newName, race = type::thing($raceTable, $raceID) )
        END
`, {
            raceTable: Race.TABLE,
            raceID: raceID,
            animalTable: animal.record.table,
            animalID: animal.record.id,
            newBirthdate: newBirthdate,
            newName: newName,
        })
    } else {
        response = await query(`
        CREATE type::table($animalTable) SET birthdate = $newBirthdate, name = $newName, race = type::thing($raceTable, $raceID)
        `, {
            raceTable: Race.TABLE,
            raceID: raceID,
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