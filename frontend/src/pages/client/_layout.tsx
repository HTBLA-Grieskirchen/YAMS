import {Layout} from "../../types/layout";
import paths from "../../util/paths";
import createSubmenuLayout from "../../components/SubmenuLayout";

const ClientLayout: Layout = createSubmenuLayout({
    "Clients": {
        href: paths.clients,
        icon: "fa-address-book"
    },
    "New Client": {
        href: paths.new_client,
        icon: "fa-city"
    }
})

export default ClientLayout