import store from "./index";
import {action, autorun, computed, makeAutoObservable, observable, runInAction} from "mobx";
import {live} from "../libs/database";
import Address, {AddressResponse} from "../model/address";
import City, {CityResponse} from "../model/city";
import Country, {CountryResponse} from "../model/country";
import {ano, no} from "../util/consts";
import Treatment_type, {TreatmentTypeResponse} from "../model/treatment_type";
import Treatment, {TreatmentResponse} from "../model/treatment";
import Treatment_Appointment, {TreatmentAppointmentResponse} from "../model/treatment_appointment";
import ClientFile, {ClientFileResponse} from "../model/clientfile";


export default class ClientFileStore{
    private root: typeof store
    private dataLive: Awaited<ReturnType<typeof live>>

    indexedClientFiles: Map<string,ClientFile>

    constructor(root: typeof store) {
        this.root = root
        this.dataLive = [[], ano, no]


        this.indexedClientFiles=observable.map()

        makeAutoObservable(this, {
            refresh: action.bound,
            close: action.bound,


            clientFiles: computed.struct

        })
    }


    get clientFiles(): ClientFile[] {
        return Array.from(this.indexedClientFiles.values())
    }


    async refresh() {
        const [_r, refresh, _c] = this.dataLive

        await refresh()
    }

    async setup() {
        this.dataLive = await live("SELECT * FROM client_file ORDER BY id")

        this.registerSyncData()
    }

    close() {
        const [_r, _u, clean] = this.dataLive

        clean()
    }

    private registerSyncData() {
        autorun(() => {
            this.syncData()
        })
    }

    private syncData() {
        const [result, _u, _c] = this.dataLive
        if (!(result.length > 0 && result[0].result)) return

        runInAction(() => {

            // Update treatments
            const clientFileIDs: Set<string> = new Set(
                result[0].result.map((item: any) => {
                    const response = ClientFileResponse.from(item)
                    if (!response) return

                    let client_file = this.indexedClientFiles.get(response.data.id)
                    if (client_file !== undefined) {
                        response.applyOn(client_file)
                    } else {
                        client_file = response.intoObject()
                        if (!client_file) return

                        this.indexedClientFiles.set(response.data.id, client_file)
                    }
                    return response.data.id
                }).filter((it: any) => it !== undefined))
            Array.from(this.indexedClientFiles.keys()).filter((id) => !clientFileIDs.has(id)).forEach((id) => this.indexedClientFiles.delete(id))
        })
    }
}