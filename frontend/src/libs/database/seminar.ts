import store from "../../stores";
import {makeRecordForTable} from "../../model/surreal";
import Seminar, {SeminarResponse} from "../../model/seminar";
import {Result} from "surrealdb.js";
import {query} from "./index";

export async function ensureSeminar(seminar: {
    title: string,
    price: number,
    duration: number | null
}) {
    const potentialDuplicate = store.eventStore.seminars.find((item) => {
        return (item.title ?? "") == seminar.title.trim() &&
            item.price == seminar.price &&
            (item.duration ?? 0) == (seminar.duration ?? 0)
    })

    if (potentialDuplicate !== undefined) {
        return potentialDuplicate.record
    } else {
        const response = await createSeminar(
            seminar.title.trim(),
            seminar.price,
            seminar.duration
        )

        if (response.error) {
            throw response.error
        }

        return makeRecordForTable(response.result[0].id, Seminar.TABLE)
    }
}

export async function createSeminar(
    title: SeminarResponse["data"]["title"],
    price: SeminarResponse["data"]["price"],
    duration: number | null,
): Promise<Result<any>> {
    const actualTitle = title.trim()
    const actualDuration = duration != null ? `${duration}ms` : null

    const response = await query(`
    CREATE type::table($seminarTable) SET title = $title, price = $price, duration = IF ($duration != NULL) THEN ( type::duration($duration) ) ELSE ( NULL ) END;
`, {
        seminarTable: Seminar.TABLE,
        title: actualTitle,
        price: price,
        duration: actualDuration
    })

    if (!response[0]) {
        return {
            error: new Error("No Response at all")
        }
    }

    return response[0]
}
