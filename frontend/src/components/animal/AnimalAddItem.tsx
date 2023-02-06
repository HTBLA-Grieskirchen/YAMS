import {observer, useLocalObservable} from "mobx-react";
import React, {useState} from "react";
import AnimalComboBox from "./AnimalComboBox";
import Race from "../../model/race";
import {patchRace} from "../../libs/database/race";
import notification from "../../libs/notification";
import store from "../../stores";
import {patchAnimal} from "../../libs/database/animal";
import {query} from "../../libs/database";
import Client from "../../model/client";

const AnimalTableHeader = observer(({client}: { client: Client }) => {
    const [adding, setAdding] = useState(false)
    const [submitted, setSubmitted] = useState(false)

    const [name, setName] = useState('')
    const [birthdate, setBirthdate] = useState(new Date().toISOString().substring(0, 10))
    const race = useLocalObservable(() => ({
        value: null as Race | null,
        setValue(value: Race) {
            this.value = value
        }
    }))

    const handleSubmit = async () => {
        setSubmitted(true)
        //TODO: aufgrund eines Bugs von Surreal muss die Race_ID, welche in die animals_liste
        // gepeichert werden soll, doppelt hineingespeichert werden

        // updaten des clients : update client:7hts0tybxonc3n77dlmb SET animals = [animal:ircj4lk1n8h8uoe8ms52, animal:ircj4lk1n8h8uoe8ms52]
        // erstellen des races : CREATE race SET description = 'TestDesc', animal_species = 'TestSpec';

        await query("BEGIN TRANSACTION;")

        let gotError: boolean = false
        console.log(race.value)
        if (race.value != null) {
            const resultRace = await patchRace(race.value, race.value?.description, race.value?.animal_species)

            if (resultRace.error) {
                gotError = true
                notification.error({
                    title: "Update Country",
                    message: `${race.value.description} can not be updated. ${resultRace.error.message}.`
                }, 10, {
                    "Retry": {
                        action: async () => {
                            await handleSubmit()
                            return true
                        },
                        disabled: () => submitted
                    }
                })
                return
            } else {
                await store.animalStore.refresh()
            }

            console.log(resultRace.result)

            if (resultRace.result != null || resultRace.result !== undefined) {
                const resultAnimal = await patchAnimal(null, new Date(birthdate), name, resultRace.result?.id)

                if (resultAnimal.error) {
                    gotError = true
                    notification.error({
                        title: "Update Country",
                        message: `${name} can not be updated. ${resultAnimal.error.message}.`
                    }, 10, {
                        "Retry": {
                            action: async () => {
                                await handleSubmit()
                                return true
                            },
                            disabled: () => submitted
                        }
                    })
                } else {
                    await store.animalStore.refresh()
                }

                console.log(gotError)

                if (gotError) {
                    await query("CANCEL TRANSACTION;")
                }

                const responseClient = await query("UPDATE type::thing($clientTable, $clientID) " +
                    "SET animals = array::concat(animals, [type::thing($animalTable, $animalID), type::thing($animalTable, $animalID)])",
                    {
                        clientTable: client.record.table,
                        clientID: client.record.id,
                        animalTable: resultAnimal.result.animalTable,
                        animalID: resultAnimal.result.animalID
                    }
                )
            }

            if (!gotError) {
                await query("COMMIT TRANSACTION;")
            }
        }

        setSubmitted(false)
        setAdding(!adding)
    }

    const handleCancel = () => {
        setAdding(!adding)
    }

    return (
        <div className="m-2">
            {!adding ?
                <button
                    className="btn btn-primary btn-sm btn-square"
                    onClick={() => setAdding(!adding)}>
                    <i className="fa-solid fa-add m-auto"/>
                </button>
                :
                <div className="flex flex-col space-y-2">
                    <form>
                        <div className="flex flex-row space-x-4 items-end">
                            <div>
                                <label className="label">
                                    <span className="label-text">Name</span>
                                </label>
                                <input
                                    type="text"
                                    className="input input-bordered w-full max-w-xs"
                                    placeholder="Balu"
                                    value={name}
                                    onChange={e => setName(e.target.value)}
                                    required={true}
                                />
                            </div>
                            <div>
                                <label className="label">
                                    <span className="label-text">Birthdate</span>
                                </label>
                                <input
                                    type="date"
                                    className="input input-bordered w-full max-w-xs"
                                    placeholder={Date.now().toString()}
                                    value={birthdate}
                                    onChange={e => setBirthdate(e.target.value)}
                                    required={true}
                                />
                            </div>
                            <div>
                                <AnimalComboBox race={race}/>
                            </div>
                            <button
                                disabled={submitted}
                                type="submit"
                                onClick={event => {
                                    handleSubmit()
                                }}
                                className="btn btn-success btn-sm btn-square m-2">
                                <i className="fa-solid fa-check"/>
                            </button>
                            <button
                                type="button"
                                onClick={event => {
                                    handleCancel()
                                }}
                                className="btn btn-error btn-sm btn-square m-2">
                                <i className="fa-solid fa-cancel"/>
                            </button>
                        </div>
                    </form>
                </div>
            }

        </div>)
})

export default AnimalTableHeader