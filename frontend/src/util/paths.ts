import {UrlObject} from "url"

const paths = {
    home: "/",
    addresses: "/addresses",
    cities: "/addresses/city",
    countries: "/addresses/country",
    client: client,
    clients: "/client",
    new_client: "/client/add",
    purchases: "/purchases",
    products: "/purchases/product",
    productTypes: "/purchases/productType"
}

function client(clientID: string): UrlObject {
    return {
        pathname: "/client/[id]",
        query: {id: clientID},
    }
}

export default paths
