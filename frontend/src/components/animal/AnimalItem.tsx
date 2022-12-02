import {observer} from "mobx-react";
import Animal from "../../model/animal";
import {useState} from "react";

const AnimalItem = observer(({animal}: { animal: Animal }) => {
    const [deleteSubmitted, setDeleteSubmitted] = useState(false)

    const deleteSubmit = async () => {
        setDeleteSubmitted(true)
        //TODO: finish implementing
    }

    return (
        <div className="flex flex-col w-fit">
            <div className="flex flex-row space-x-4 items-center">
                <div className="flex flex-row space-x-2">
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Name
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {animal.name}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Race
                        </label>
                        <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                            {animal.race}
                        </p>
                    </div>
                    <div className="flex flex-col">
                        <label className="text-gray-700 text-sm sm:w-48 w-fit">
                            Birthdate
                        </label>
                        {animal.birthdate != null ?
                            <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                                {animal.birthdate.toLocaleDateString()}
                            </p>
                            :
                            <p className="text-lg min-w-full xl:max-w-4xl sm:max-w-sm max-w-0 truncate">
                                Not Present
                            </p>
                        }

                    </div>
                </div>
                <div className="flex flex-row h-full items-center space-x-1">
                    <button type="button" onClick={e => {
                        deleteSubmit()
                    }} disabled={deleteSubmitted}
                            className="align-text-bottom text-3xl hover:text-4xl hover:text-red-700 text-red-600 w-8 h-8 transition-all">
                        <i className="fa-solid fa-remove"/>
                    </button>
                </div>
            </div>
        </div>
    )
})

export default AnimalItem