import {Record} from "../../model/surreal";
import {query} from "./index";
import {Result} from "surrealdb.js";
import Event, {EventResponse} from "../../model/event";

export async function createEvent(
    maxParticipants: EventResponse["data"]["max_participants"],
    date: Date,
    locationName: EventResponse["data"]["location_name"],
    location: Record,
    seminar: Record
): Promise<Result<any>> {
    const actualLocationName = locationName?.trim()

    const response = await query(`
CREATE type::table($eventTable) SET max_participants = $maxParticipants, date = $date, location_name = $locationName, location = type::thing($locationTable, $locationID), seminar = type::thing($seminarTable, $seminarID);
`, {
        eventTable: Event.TABLE,
        maxParticipants: maxParticipants,
        date: date,
        locationName: actualLocationName,
        locationTable: location.table,
        locationID: location.id,
        seminarTable: seminar.table,
        seminarID: seminar.id
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}

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
