import { EventType } from './events'
import { Counter } from './counter'

interface EventHandler {
    // The callback added to the DOM.
    registeredEvent(data: any): void

    // A callback that can be changed without updating the DOM element.
    hook(data: any): void
}

class Atom {
    constructor(
        private inner: Node,
        private handlers: { [k in string]?: EventHandler } = {},
    ) {}

    public node(): Node {
        return this.inner
    }

    public registerEvent(type: EventType, callback: (json: string) => void) {
        const handler = this.handlers[type]

        const hook = (data: any) => {
            callback(EventType.serialize(type, data))
        }

        if (handler === undefined) {
            const registeredEvent = (data: any) => {
                this.handlers[type]!.hook(data)
            }

            this.inner.addEventListener(EventType.toName(type), registeredEvent)

            this.handlers[type] = { registeredEvent, hook }
        } else {
            handler.hook = hook
        }
    }

    public deleteEvent(type: EventType) {
        const { registeredEvent } = this.handlers[type]!
        this.inner.removeEventListener(EventType.toName(type), registeredEvent)
        delete this.handlers[type]
    }
}

// This is the atom ID for the mount point.
const MOUNT_ID = 0

export class Atoms {
    private idCounter: Counter
    private map: {
        [k in string]: Atom
    }

    constructor(mount: string) {
        this.map = {
            // Make sure this is not coerced into an array.
            _: (null as any)
        }

        this.idCounter = new Counter(MOUNT_ID)
        this.map[this.idCounter.next()] = new Atom(document.getElementById(mount)!)
    }

    public createTextNode(text: string, replace?: number): [number, Node] {
        const node = document.createTextNode(text)

        if(replace !== undefined) {
            this.map[replace] = new Atom(node)
            return [replace, node]
        } else {
            const id = this.idCounter.next()
            this.map[id.toString()] = new Atom(node)
            return [id, node]
        }
    }

    public createElement(attributes: Array<[string, string]>, replace?: number): [number, Node] {
        const node = document.createElement('div')

        for(const [k, v] of attributes) {
            node.setAttribute(k, v)
        }

        if(replace !== undefined) {
            this.map[replace] = new Atom(node)
            return [replace, node]
        } else {
            const id = this.idCounter.next()
            this.map[id] = new Atom(node)
            return [id, node]
        }
    }

    public getAtom(id: number): Atom {
        return this.map[id]
    }

    public deleteAtom(id: number) {
        delete this.map[id]
    }
}
