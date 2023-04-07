import Client from "../../model/client";

export function clientSearched(client: Client, query: string) {
    return query.trim().split(" ").flatMap((item) => item.split(",")).every((segment) => {
        const trimmedSegment = segment.trim().toLowerCase()
        if (trimmedSegment.length < 0) return true

        return client.firstName.toLowerCase().includes(trimmedSegment) ||
            client.lastName.toLowerCase().includes(trimmedSegment) ||
            client.email.toLowerCase().includes(trimmedSegment) ||
            client.mobileNumber.toLowerCase().includes(trimmedSegment) ||
            client.customerNumber.toString().toLowerCase().includes(trimmedSegment)
    })
}
