export default function (fn, wait) {
    let timer = null
    return () => {
        if (timer != null) {
            clearTimeout(timer)
        }
        timer = setTimeout(fn, wait)
    }
}