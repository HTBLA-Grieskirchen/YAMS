import {observer} from "mobx-react";
import {useStore} from "../stores";

const DialogOverlay = observer(() => {
    const store = useStore()

    return <>
        {store.dialogStore.dialogs().map((dialog) =>
            <div key={dialog.uuid} className="flex fixed inset-0 bg-gray-600 bg-opacity-50
                overflow-y-auto h-full w-full items-center"
            >
                <div className="p-2 rounded-lg bg-white self-center h-fit w-fit mx-auto shadow-lg">
                    {dialog.component(store.dialogStore.closeDialog)}
                </div>
            </div>
        )}
    </>
})

export default DialogOverlay
