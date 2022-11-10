import {Layout} from "../../types/layout";
import paths from "../../util/paths";
import createSubmenuLayout from "../../components/SubmenuLayout";

const AddressLayout: Layout = createSubmenuLayout({
    "Addresses": {
        href: paths.addresses,
        icon: "fa-address-book"
    },
    "Countries": {
        href: paths.countries,
        icon: "fa-earth-europe"
    }
})

export default AddressLayout