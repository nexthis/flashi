interface Paginate<T> {
    next: number | false
    previous: number | false
    max: number
    data: Array<T>
}
