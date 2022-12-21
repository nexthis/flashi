import { useI18n } from "vue-i18n"

export function timeSince(date: Date): string {
    const rtf = new Intl.RelativeTimeFormat(useI18n().locale.value, { numeric: "auto" })

    const seconds = Math.floor((Date.now() - date.getTime()) / 1000)

    let interval = seconds / 31536000
    if (interval > 1) {
        return rtf.format(-Math.floor(interval), "years")
    }
    interval = seconds / 2592000
    if (interval > 1) {
        return rtf.format(-Math.floor(interval), "months")
    }
    interval = seconds / 86400
    if (interval > 1) {
        return rtf.format(-Math.floor(interval), "days")
    }
    interval = seconds / 3600
    if (interval > 1) {
        return rtf.format(-Math.floor(interval), "hours")
    }
    interval = seconds / 60
    if (interval > 1) {
        return rtf.format(-Math.floor(interval), "minutes")
    }
    return rtf.format(-Math.floor(interval), "seconds")
}
