const paths = {
    home: "/",
    addresses: "/addresses",
    cities: "/addresses/city",
    countries: "/addresses/country",
    client: client,
    clientRelations: clientRelations,
    clients: "/client",
    new_client: "/client/add"
}

function client(clientID: string): string {
    return `/client/${encodeURIComponent(clientID)}`
}

function clientRelations(clientID: string): string {
    return `/client/${encodeURIComponent(clientID)}/relations`
}

export default paths
