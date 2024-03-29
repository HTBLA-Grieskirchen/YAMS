import {observer} from "mobx-react";
import {useStore} from "../stores";

const Modals = observer(() => {
    const store = useStore()

    return <>
        {store.dialogStore.dialogs().map((dialog) =>
            <div key={dialog.uuid} className="flex fixed inset-0 bg-gray-600 bg-opacity-50
                overflow-y-auto h-full w-full items-center">
                <div
                    className={`modal modal-open ${dialog.type === "responsive" ? "modal-bottom md:modal-middle" : ""}`}>
                    {dialog.component(store.dialogStore.closeDialog)}
                </div>
            </div>
        )}
    </>
})

export default Modals
