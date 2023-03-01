import {Result} from "surrealdb.js";
import {query} from "./index";
import Treatment from "../../model/treatment";
import TreatmentAppointment from "../../model/treatmentAppointment";

export async function deleteTreatmentAppointment(treatmentAppointment: TreatmentAppointment): Promise<Result<any>> {
    // TODO: Add event check once implemented
    const checkResult = await query("SELECT id FROM client WHERE treatmentAppointment = type::thing($treatmentAppointmentTable, $treatmentAppointmentID)", {
        treatmentAppointmentTable: treatmentAppointment.record.table,
        treatmentAppointmentID: treatmentAppointment.record.id
    })
    if (checkResult[0] && checkResult[0].result.length > 0) {
        return {
            error: new Error("TreatmentAppointment is still used in some places")
        }
    }

    const response = await query("DELETE type::thing($treatmentAppointmentTable, $treatmentAppointmentID)", {
        treatmentAppointmentTable: treatmentAppointment.record.table,
        treatmentAppointmentID: treatmentAppointment.record.id
    })

    return response[0] ?? {
        error: new Error("No response at all")
    }
}

export async function patchTreatmentAppointment(treatmentAppointment: TreatmentAppointment, newCost: number, newExtra: string, newTreatment: Treatment): Promise<Result<any>> {
    const response = await query(`
IF ( SELECT true FROM type::thing($treatmentAppointmentTable, $treatmentAppointmentID) ) THEN
    ( UPDATE type::thing($treatmentAppointmentTable, $treatmentAppointmentID) SET cost = $cost, extra = $extra, treatment = type::thing($treatmentTable, $treatmentID) )
ELSE 
    ( CREATE type::table($treatmentAppointmentTable) SET cost = $cost, extra = $extra, treatment = type::thing($treatmentTable, $treatmentID) )
END<
`, {
        treatmentAppointmentTable: treatmentAppointment.record.table,
        treatmentAppointmentID: treatmentAppointment.record.id,
        treatmentTable: newTreatment.record.table,
        treatmentID: newTreatment.record.id,
        cost: newCost,
        extra: newExtra,
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}
