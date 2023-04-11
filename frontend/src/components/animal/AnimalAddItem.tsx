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
import Animal from "../../model/animal";

const AnimalAddItem = observer(({client}: { client: Client }) => {
    const [adding, setAdding] = useState(false)
    const [submitted, setSubmitted] = useState(false)

    const [name, setName] = useState('')
    const [birthdate, setBirthdate] = useState(new Date().toISOString().substring(0, 10))
    const race = useLocalObservable(() => ({
        value: null as Race | null,
        setValue(value: Race | null) {
            this.value = value
        }
    }))

    const handleSubmit = async () => {
        setSubmitted(true)
        await query("BEGIN TRANSACTION;")

        let gotError: boolean = false
        if (race.value != null) {
            const resultRace = await patchRace(race.value, race.value?.description, race.value?.animal_species)

            if (resultRace.error) {
                gotError = true
                notification.error({
                    title: "Update Race",
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

            if (resultRace.result != null || resultRace.result !== undefined) {
                const resultAnimal = await patchAnimal(null, new Date(birthdate), name, resultRace.result)

                if (resultAnimal.error) {
                    gotError = true
                    notification.error({
                        title: "Update Animal",
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

                if (gotError) {
                    await query("CANCEL TRANSACTION;")
                }

                console.log(client.record.id)
                console.log(resultAnimal.result)
                const responseClient = await query(
                    "UPDATE type::thing($clientTable, $clientID) SET animals += [type::thing($animalTable, $animalID)]",
                    {
                        clientTable: client.record.table,
                        clientID: client.record.id,
                        animalTable: Animal.TABLE,
                        animalID: resultAnimal.result[0].id
                    }
                )

                if (responseClient != null) {
                    await store.clientStore.refresh()
                }
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

    const validInput = () => {
        return name != '' && name != null && race.value != null
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
                                disabled={!validInput()}
                                type="button"
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

export default AnimalAddItem