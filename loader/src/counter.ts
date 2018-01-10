export class Counter {
    constructor (private value: number = 0) {}

    public next(): number {
        return this.value++
    }
}
