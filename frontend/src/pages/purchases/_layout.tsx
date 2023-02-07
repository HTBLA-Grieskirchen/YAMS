import {Layout} from "../../types/layout";
import paths from "../../util/paths";
import createTabMenuLayout from "../../components/layout/TabMenuLayout";

const PurchaseLayout: Layout = createTabMenuLayout({
    "Purchase": {
        href: paths.purchases,
        icon: "fa-cart-shopping"
    },
    "Product": {
        href: paths.products,
        icon: "fa-p"
    },
    "ProductType": {
        href: paths.productTypes,
        icon: "fa-t"
    }
})

export default PurchaseLayout