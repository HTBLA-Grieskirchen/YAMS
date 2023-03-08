const paths = {
    home: "/",
    addresses: "/addresses",
    cities: "/addresses/city",
    countries: "/addresses/country",
    client: client,
    clientRelations: clientRelations,
    clientEdit: clientEdit,
    clients: "/client",
    new_client: "/client/add",

    events: "/events",
    event_new: "/events/add"
}

function client(clientID: string): string {
    return `/client/${encodeURIComponent(clientID)}`
}

function clientRelations(clientID: string): string {
    return `/client/${encodeURIComponent(clientID)}/relations`
}

function clientEdit(clientID: string): string {
    return `/client/${encodeURIComponent(clientID)}/edit`
}

export default paths
