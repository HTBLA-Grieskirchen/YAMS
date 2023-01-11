import {Layout} from "../../types/layout";
import paths from "../../util/paths";
import createTabMenuLayout from "../../components/layout/TabMenuLayout";

const ClientLayout: Layout = createTabMenuLayout({
    "Clients": {
        href: paths.clients,
        icon: "fa-address-book"
    },
    "New Client": {
        href: paths.new_client,
        icon: "fa-plus-square"
    }
})

export default ClientLayout