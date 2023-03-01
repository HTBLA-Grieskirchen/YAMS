import {Layout} from "../../types/layout";
import paths from "../../util/paths";
import createSubmenuLayout from "../../components/SubmenuLayout";

const AddressLayout: Layout = createSubmenuLayout({
    "Treatment Appointment": {
        href: paths.treatmentAppointments,
        icon: "fa-folder-medical"
    },
    "Treatment": {
        href: paths.treatments,
        icon: "fa-notes-medical"
    },
    "Treatment Type": {
        href: paths.treatmenttypes,
        icon: "fa-staff-snake"
    }
})

export default AddressLayout