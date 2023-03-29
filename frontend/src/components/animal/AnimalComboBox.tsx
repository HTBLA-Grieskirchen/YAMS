import React, {useEffect, useState} from 'react'
import {Combobox} from '@headlessui/react'
import {observer, useLocalObservable} from "mobx-react";
import {useStore} from "../../stores";
import Race from "../../model/race";
import dialog from "../../libs/dialog";
import {Record} from "../../model/surreal";

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
    }, [selectedRace])

    const filteredPeople =
        query === ''
            ? races
            : races.filter((race) => {
                return race.description.toLowerCase().includes(query.toLowerCase())
            })

    const submit = () => {
        let newRace = new Race(new Record(Race.TABLE, "").join(), raceValue.desc, raceValue.species)
        setSelectedRace(newRace)
    }

    const openNewRaceDialog = () => {
        dialog((close) =>
            <div className="modal-box">
                <p className="font-bold text-lg">Add new race</p>
                <form>
                    <div className="flex flex-row space-x-4 items-end">
                        <div>
                            <label className="label">
                                <span className="label-text">Race Description</span>
                            </label>
                            <input
                                type="text"
                                className="input input-bordered w-full max-w-xs"
                                placeholder="German Sheppard"
                                value={raceValue.desc}
                                onChange={e => raceValue.setDesc(e.target.value)}
                                required={true}
                            />
                        </div>
                        <div>
                            <label className="label">
                                <span className="label-text">Animal Species</span>
                            </label>
                            <input
                                type="text"
                                className="input input-bordered w-full max-w-xs"
                                placeholder="Dog"
                                value={raceValue.species}
                                onChange={e => raceValue.setSpecies(e.target.value)}
                                required={true}
                            />
                        </div>
                    </div>
                    <div className="flex flex-row space-x-4 items-end mt-3">
                        <button className="btn btn-sm btn-success" onClick={() => {
                            close()
                            submit()
                        }}>
                            Submit
                        </button>
                        <button className="btn btn-sm btn-error" onClick={() => {
                            close()
                        }}>
                            Cancel
                        </button>
                    </div>
                </form>
            </div>
        )
    }

    return (
        <>
            <label className="label">
                <span className="label-text">Race</span>
            </label>
            <Combobox value={selectedRace} onChange={setSelectedRace}>
                <Combobox.Input
                    onChange={(event) => setQuery(event.target.value)}
                    displayValue={(race: Race | null) => !!race ? race.description + ", " + race.animal_species : "Wö wos aus du huso"}
                    className="input shadow bg-base-100 rounded-box max-w-xs"
                />
                {(query.length > 0 || filteredPeople.length > 0) &&
                    <Combobox.Options
                        className="dropdown-content mt-1 menu p-2 shadow bg-base-100 rounded-box max-w-xs break-words">
                        {query.length > 0 && (
                            <Combobox.Option value={{id: null, name: query}} onClick={openNewRaceDialog}>
                                Create new race
                            </Combobox.Option>
                        )}
                        {filteredPeople.map((race: Race) => (
                            <Combobox.Option key={race.record.join()} value={race}>
                                {race.description + ", " + race.animal_species}
                            </Combobox.Option>
                        ))}
                    </Combobox.Options>
                }
            </Combobox>
        </>
    )
})

export default AnimalComboBox