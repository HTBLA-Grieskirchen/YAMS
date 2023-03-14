import {Record} from "../../model/surreal";
import {query} from "./index";

export async function deleteEvent(
    event: Record
) {
    const response = await query(`
DELETE type::thing($eventTable, $eventID);
`, {
        eventTable: event.table,
        eventID: event.id
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}
