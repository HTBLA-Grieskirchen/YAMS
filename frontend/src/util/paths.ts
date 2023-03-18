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
    event_new: "/events/add",
    event: event,
    event_edit: eventEdit
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

function event(eventID: string): string {
    return `/events/${encodeURIComponent(eventID)}`
}

function eventEdit(eventID: string): string {
    return `/events/${encodeURIComponent(eventID)}/edit`
}

export default paths
