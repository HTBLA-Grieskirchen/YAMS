import {observer} from "mobx-react";
import Client from "../../model/client";
import {useLive} from "../../libs/database";
import Animal from "../../model/animal";
import AnimalItem from "./AnimalItem";

const AnimalList = observer(({client}: { client: Client }) => {
    const [animalsRaw, refreshAnimals] = useLive("SELECT animals.*.* FROM type::thing($table, $id)", {
        table: Client.TABLE_NAME,
        id: client.id
    })
    console.log()
    const animals: Animal[] = animalsRaw.response && animalsRaw.response.length > 0 && animalsRaw.response[0].result && animalsRaw.response[0].result[0].animals ? animalsRaw.response[0].result[0].animals.map((animal: any) => {
        if (animal.id !== undefined && animal.name !== undefined && animal.race !== undefined && animal.birthdate !== undefined) {
            return new Animal(animal.id, new Date(animal.birthdate.split('T')[0]), animal.name, animal.race)
        }
    }).filter((it: any) => it !== undefined) : []

    return (
        <main className="flex flex-col w-fit m-5 p-3 rounded-lg bg-gray-200 shadow">
            <div className="flex flex-col pt-3">
                {animals.length > 0 ?
                    <div className="divide-gray-400 divide-y">
                        {animals.map((animal) =>
                            <div key={animal.record()} className="p-2">
                                <AnimalItem animal={animal}/>
                            </div>
                        )}
                    </div>
                    :
                    <p className="p-2 text-gray-600">
                        No animals available!
                    </p>
                }
            </div>
        </main>
    )
})

export default AnimalList