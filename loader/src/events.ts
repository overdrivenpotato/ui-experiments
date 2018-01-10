import { Counter } from './counter'

export enum EventType {
    Click = 0,
    MouseDown = 1,
    MouseUp = 2,
}

export namespace EventType {
    export const toName = (type: EventType): string => {
        switch(type) {
            case EventType.Click: return 'click'
            case EventType.MouseDown: return 'mousedown'
            case EventType.MouseUp: return 'mouseup'
        }
    }

    // Serialize event data.
    export const serialize = (type: EventType, data: any): string => {
        switch(type) {
            case EventType.Click: return JSON.stringify({
                x: data.clientX,
                y: data.clientY,
            })

            case EventType.MouseUp: return JSON.stringify({
                button: data.button,
                x: data.clientX,
                y: data.clientY,
            })

            case EventType.MouseDown: return JSON.stringify({
                button: data.button,
                x: data.clientX,
                y: data.clientY,
            })
        }
    }
}

interface EventInstance {
    atom: number
    type: EventType
}

export class EventMap {
    private map: { [k in string]?: EventInstance }
    private counter: Counter

    constructor() {
        this.map = { _: null } as any
        this.counter = new Counter()
    }

    create(instance: EventInstance): number {
        const id = this.counter.next()
        this.map[id] = instance
        return id
    }

    delete(id: number): EventInstance {
        const instance = this.map[id]
        delete this.map[id]
        return instance!
    }
}
