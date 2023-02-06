import {observer} from "mobx-react";
import Client from "../../model/client";
import {useLive} from "../../libs/database";
import Animal from "../../model/animal";
import AnimalItem from "./AnimalItem";
import AnimalTableHeader from "./AnimalTableHeader";
import AnimalAddItem from "./AnimalAddItem";

const AnimalList = observer(({client}: { client: Client }) => {
    const [animalsRaw, refreshAnimals] = useLive("SELECT animals.*.* FROM type::thing($table, $id)", {
        table: Client.TABLE,
        id: client.record.id
    })
    console.log()
    const animals: Animal[] = animalsRaw.response && animalsRaw.response.length > 0 && animalsRaw.response[0].result && animalsRaw.response[0].result[0].animals ? animalsRaw.response[0].result[0].animals.map((animal: any) => {
        if (animal.id !== undefined && animal.name !== undefined && animal.race !== undefined && animal.birthdate !== undefined) {
            return new Animal(animal.id, new Date(animal.birthdate.split('T')[0]), animal.name, animal.race)
        }
    }).filter((it: any) => it !== undefined) : []

    return (
        <main className="flex flex-col w-full rounded-lg bg-white shadow-md">
            <div className="table table-auto w-full">
                <AnimalTableHeader/>
                {animals.length > 0 ??
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