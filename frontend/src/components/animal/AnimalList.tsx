import {observer} from "mobx-react";
import Client from "../../model/client";
import AnimalAddItem from "./AnimalAddItem";
import {useStore} from "../../stores";
import React from "react";
import {AnimalRow} from "./AnimalTable";

const AnimalList = observer(({client}: { client: Client }) => {
    const store = useStore()
    const animals = client.animals

    return (
        <main className="w-full">
            {animals.map((animal) =>
                <AnimalRow key={animal.record.join()} animal={animal}/>
            )}
            <AnimalAddItem client={client}/>
        </main>
    )
})

export default AnimalList