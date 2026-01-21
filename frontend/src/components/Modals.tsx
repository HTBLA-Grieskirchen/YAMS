import {observer} from "mobx-react";
import {useStore} from "../stores";
import { Modal, ModalContent } from "@heroui/react";

const Modals = observer(() => {
    const store = useStore()

    return <>
        {store.dialogStore.dialogs().map((dialog, index) =>
            <Modal 
                key={dialog.uuid || index} 
                isOpen={true} 
                onClose={() => store.dialogStore.closeDialog(false)}
                scrollBehavior="inside"
                placement="center"
                backdrop="blur"
            >
                <ModalContent>
                    {(onClose) => (
                        <div className="p-1">
                            {dialog.component(() => store.dialogStore.closeDialog(false))}
                        </div>
                    )}
                </ModalContent>
            </Modal>
        )}
    </>
})

export default Modals
