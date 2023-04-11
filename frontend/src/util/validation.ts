const RFC2822EmailRegEx =
    /(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])/

const PhoneNumberRegEx =
    /[+|0]([0-9\-()\/.]\s?){6,15}[0-9]$/


export function isValidEmail(email: string) {
    return String(email)
        .toLowerCase()
        .match(RFC2822EmailRegEx)
}

export function isValidMobilenumber(number: string) {
    return String(number)
        .match(PhoneNumberRegEx)
}

export function isValidTime(timeString: string) {
    const timeComponents = timeString.split(':')
    if (timeComponents.length < 2) {
        return false
    }

    const date = new Date()

    const hours = Number(timeComponents[0])
    date.setHours(hours)
    if (isNaN(hours) || date.getHours() !== hours) {
        return false
    }

    const minutes = Number(timeComponents[1])
    date.setMinutes(minutes)
    if (isNaN(minutes) || date.getMinutes() !== minutes) {
        return false
    }

    if (timeComponents.length > 2) {
        const seconds = Number(timeComponents[3])
        date.setSeconds(seconds)
        if (isNaN(seconds) || date.getSeconds() !== seconds) {
            return false
        }
    }

    return true
}
