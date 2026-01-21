import React, {useEffect, useState, useMemo} from 'react'
import {observer, useLocalObservable} from "mobx-react";
import {useStore} from "../../stores";
import Race from "../../model/race";
import dialog from "../../libs/dialog";
import {Record} from "../../model/surreal";
import { Autocomplete, AutocompleteItem, Button, Input } from "@heroui/react";

const AnimalComboBox = observer((
    {race}:
        { race: { setValue: (value: Race | null) => void } }
) => {
    const store = useStore()
    const races = store.animalStore.races

    const [selectedRace, setSelectedRace] = useState<Race | null>(null)
    const [query, setQuery] = useState('')

    const raceValue = useLocalObservable(() => ({
        desc: "",
        species: "",
        setDesc(desc: string) {
            this.desc = desc
        },
        setSpecies(species: string) {
            this.species = species
        }
    }))

    useEffect(() => {
        race.setValue(selectedRace)
    }, [selectedRace, race])

    const filteredRaces = useMemo(() => {
        return query === ''
            ? races
            : races.filter((race) => {
                return race.description.toLowerCase().includes(query.toLowerCase())
            })
    }, [races, query])

    const allOptions = useMemo(() => {
        return [
            { id: 'new-race', label: `+ Create new race "${query}"`, isCustom: true },
            ...filteredRaces.map(r => ({ id: r.record.join(), label: `${r.description}, ${r.animal_species}`, isCustom: false }))
        ];
    }, [filteredRaces, query]);

    const submit = () => {
        let newRace = new Race(new Record(Race.TABLE, "").join(), raceValue.desc, raceValue.species)
        setSelectedRace(newRace)
    }

    const openNewRaceDialog = () => {
        dialog((close) =>
            <div className="p-4">
                <p className="font-bold text-lg mb-4">Add new race</p>
                <form className="space-y-4">
                    <div className="flex flex-col gap-4">
                        <Input
                            label="Race Description"
                            placeholder="German Sheppard"
                            value={raceValue.desc}
                            onValueChange={raceValue.setDesc}
                            isRequired
                            variant="bordered"
                        />
                        <Input
                            label="Animal Species"
                            placeholder="Dog"
                            value={raceValue.species}
                            onValueChange={raceValue.setSpecies}
                            isRequired
                            variant="bordered"
                        />
                    </div>
                    <div className="flex justify-end gap-2 mt-6">
                        <Button color="danger" variant="light" onClick={() => close()}>
                            Cancel
                        </Button>
                        <Button color="success" onClick={() => {
                            close()
                            submit()
                        }}>
                            Submit
                        </Button>
                    </div>
                </form>
            </div>
        )
    }

    return (
        <Autocomplete
            label="Race"
            labelPlacement="outside"
            placeholder="Search or create a race..."
            variant="bordered"
            onInputChange={setQuery}
            selectedKey={selectedRace ? selectedRace.record.join() : null}
            onSelectionChange={(key) => {
                if (key === "new-race") {
                    openNewRaceDialog()
                } else {
                    const found = races.find(r => r.record.join() === String(key))
                    if (found) setSelectedRace(found)
                }
            }}
            items={allOptions}
        >
            {(item) => (
                <AutocompleteItem key={item.id} textValue={item.label} color={item.isCustom ? "primary" : "default"}>
                    {item.label}
                </AutocompleteItem>
            )}
        </Autocomplete>
    )
})

export default AnimalComboBox
