import {observer} from "mobx-react";
import Client from "../../model/client";
import AnimalItem from "./AnimalItem";
import AnimalTableHeader from "./AnimalTableHeader";
import AnimalAddItem from "./AnimalAddItem";
import {useStore} from "../../stores";

const AnimalList = observer(({client}: { client: Client }) => {
    const store = useStore()
    const animals = store.animalStore.animals

    return (
        <main className="flex flex-col w-full rounded-lg bg-white shadow-md">
            <div className="table table-auto w-full">
                <AnimalTableHeader/>
                {animals.length > 0 &&
                    <div className="table-row-group w-full bg-white divide-y-2">
                        {animals.map((animal) =>
                            <AnimalItem key={animal.record.join()} animal={animal}/>
                        )}
                    </div>}
            </div>
            <AnimalAddItem client={client}/>
        </main>
    )
})

export default AnimalList