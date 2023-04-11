import {makeAutoObservable, runInAction} from "mobx"
import {makeRecordForTable, Record, SurrealResponse} from "./surreal";
import Client from "./client";
import store from "../stores";
import Treatment from "./treatment";
import Treatment_Appointment from "./treatment_appointment";

export default class ClientFile {
    static readonly TABLE: string = "client_file"
    readonly table: string = ClientFile.TABLE
    readonly record: Record
    first_consultation:Date
    extra:string
    client:Client
    treatment_appointments:Treatment_Appointment[]

    constructor(id: string, extra:string, client:Client, first_consultation:Date, treatment_appointments:Treatment_Appointment[]) {
        this.record = makeRecordForTable(id, this.table)
        this.extra= extra
        this.first_consultation = first_consultation
        this.client = client
        this.treatment_appointments=treatment_appointments

        makeAutoObservable(this)
    }


}

export class ClientFileResponse implements SurrealResponse<ClientFile> {
    readonly data: {
        id: string,
        first_consultation: string,
        extra: string,
        client: string,
        treatment?: []
    }

    private constructor(data: ClientFileResponse["data"]) {
        this.data = data
    }

    static from(item: any): ClientFileResponse | undefined {
        if (
            item.id === undefined ||
            item.first_consultation === undefined || item.extra === undefined || item.client === undefined
        ) return

        return new ClientFileResponse(item)
    }

    applyOn(object: ClientFile): void {
        // Check if data is meant for object
        if (object.record.join() != this.data.id) return

        // Address properties are mutated, and we want to let MobX know about it,
        // so it can react on changes -> runInAction()
        runInAction(() => {
            if (this.data.client != object.client.record.join()) {
                const client = store.clientStore.indexedClients.get(this.data.client)
                if (client) object.client = client
            }

            if (this.data.extra != object.extra) object.extra = this.data.extra
            const first_consultation = new Date(this.data.first_consultation)
            if (first_consultation!= object.first_consultation) object.first_consultation = first_consultation
            var treatments=this.data.treatment?.map((appointment) => store.treatmentAppointmentStore.indexedTreatmentAppointments.get(appointment)).filter((it) => it !== undefined) as Treatment_Appointment[]|undefined
            const treatment_appointments = treatments ?? []
            if (treatment_appointments != object.treatment_appointments) object.treatment_appointments = treatment_appointments



        })
    }

    intoObject(): ClientFile | undefined {
        let client = store.clientStore.indexedClients.get(this.data.client)
        if (!client) {
            console.log("Find koa triedment")
            return
        }

        const first_consultation=new Date(this.data.first_consultation)


        try {
            console.log(this.data.treatment)
            var treatments=this.data.treatment?.map((appointment) => store.treatmentAppointmentStore.indexedTreatmentAppointments.get(appointment)).filter((it) => it !== undefined) as Treatment_Appointment[]|undefined
            return new ClientFile(this.data.id,this.data.extra, client, first_consultation, treatments ?? [])

        }
        catch(Error){

            return undefined

        }
    }
}