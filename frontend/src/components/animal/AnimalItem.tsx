import {observer} from "mobx-react";
import Animal from "../../model/animal";
import {useState} from "react";
import {useStore} from "../../stores";
import {deleteAnimal} from "../../libs/database/animal";

const AnimalItem = observer(({animal}: { animal: Animal }) => {
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

    return <div className="table-row">
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-900 whitespace-nowrap border-t-2 border-t-gray-200">
            <p className="text-lg xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                {animal.name}
            </p>
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200">
            <p className="text-lg xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                {animal.race.description}
            </p>
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200">
            <p className="text-lg xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                {animal.race.animal_species}
            </p>
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200">
            {animal.birthdate != null ?
                <p className="text-lg xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                    {animal.birthdate.toLocaleDateString()}
                </p>
                :
                <p className="text-lg xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                    Not Present
                </p>
            }
        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-gray-500 whitespace-nowrap border-t-2 border-t-gray-200">

        </div>
        <div
            className="table-cell py-2 px-4 text-sm font-medium text-center whitespace-nowrap border-t-2 border-t-gray-200">
            <button className="text-red-600 hover:underline disabled:text-red-600/50 disabled:hover:no-underline"
                    disabled={deleteSubmitted} onClick={deleteSubmit}>
                Delete
            </button>
        </div>
    </div>
})

export default AnimalItem