import dialog from "../../libs/dialog";
import { deleteEvent } from "../../libs/database/event";
import notification from "../../libs/notification";
import Event from "../../model/event"
import { SubmissionState } from "../../libs/form/submit";
import store from "../../stores";


export function askSubmitDeleteEvent(event: Event, state?: SubmissionState, callback?: () => Promise<void>) {
    dialog((close) => <div className="modal-box">
        <h3 className="font-bold text-lg">
            <i className="fa-solid fa-exclamation-triangle text-warning mr-1"/>
            This action will cancel this event!
        </h3>
        <p className="py-4">
            Are you sure you want to cancel <span className="font-bold">{event.seminar.title}</span>?
            This action cannot be undone.
        </p>
        <div className="modal-action">
            <button className="btn" onClick={e => close()}>Abort</button>
            <button className="btn btn-primary" onClick={e => {
                close()
                submitDeleteEvent(event, state, callback).then()
            }}>{"I'm sure!"}
            </button>
        </div>
    </div>)
}

export async function submitDeleteEvent(event: Event, state?: SubmissionState, callback?: () => Promise<void>) {
    state?.submit()

    const result = await deleteEvent(event.record)
    if (result.error) {
        notification.error({
            title: "Event could not be cancelled!",
            message: `"${result.error.message}". Do you want to try again?`
        }, 15, {
            "Retry": {
                action: async () => {
                    await submitDeleteEvent(event)
                    return true
                },
                disabled: () => state?.submitted ?? false
            }
        })
    } else {
        await store.eventStore.refresh()
        callback && await callback()
    }

    state?.clear()
}
