import {observer} from "mobx-react";
import React, {useState} from "react";
import {useStore} from "../../stores";
import {deleteAnimal} from "../../libs/database/animal";
import Animal from "../../model/animal";

export const AnimalRow = observer((
    {animal}: { animal: Animal }
) => {
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)
    const store = useStore()

    const deleteSubmit = async () => {
        setDeleteSubmitted(true)
        const response = await deleteAnimal(animal)
        if (response.result != null) {
            await store.animalStore.refresh()
            await store.clientStore.refresh()
        }
    }

    return <div key={animal.record.join()} className="w-full flex place-content-between">
        <div className="flex flex-col my-1">
            <div className="flex flex-row">
                <p className="text-lg font-medium mb-1 mr-2">{animal.name}</p>
            </div>
            {animal.birthdate != null &&
                <div className="flex flex-row">
                    <p className="text-base font-medium mb-1 mr-2">{animal.birthdate.toLocaleDateString()}</p>
                </div>
            }
            <div className="flex flex-row">
                <p className="text-base font-medium mb-1 mr-2">Race: {animal.race.description}, {animal.race.animal_species}</p>
            </div>
        </div>
        <div className="tooltip tooltip-accent tooltip-left w-fit cursor-help my-auto"
             data-tip={`Delete ${animal.name}`}>
            <button type="button" onClick={e => {
                deleteSubmit()
            }} disabled={deleteSubmitted}
                    className="align-text-bottom text-3xl hover:text-4xl hover:text-red-700 text-red-600 w-8 h-8 transition-all">
                <i className="fa-solid fa-remove"/>
            </button>
        </div>
    </div>
})