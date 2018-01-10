import { Atoms } from './atoms'
import { EventMap, EventType } from './events'

interface Callbacks {
    callback0: (f: number) => void
    callbackEvent: (atom: number, type: number, ptr: number, len: number) => void
    createString: (length: number) => number
}

export class Wrapper {
    constructor(
        private module: any,
        private mount: string,
        private atoms: Atoms = new Atoms(mount),
    ) {}

    callbacks(): Callbacks {
        return {
            callback0: this.module.instance.exports.blocks_in_callback0,
            callbackEvent: this.module.instance.exports.blocks_in_callback_event,
            createString: this.module.instance.exports.blocks_in_create_string,
        }
    }

    mem(): Uint8Array {
        return new Uint8Array(this.module.instance.exports.memory.buffer)
    }

    callback0(f: number) {
        this.callbacks().callback0(f)
    }

    callbackEvent(id: number, type: EventType, json: string) {
        const ptr = this.createString(json)

        this.callbacks().callbackEvent(id, type, ptr, json.length)
    }

    createString(text: string): number {
        const mem = this.mem()
        const ptr = this.callbacks().createString(text.length)

        for (let i = 0; i < text.length; i++) {
            mem[ptr + i] = text.charCodeAt(i)
        }

        return ptr
    }

    readString(ptr: number, len: number): string {
        const mem = this.mem()

        let s = ''

        for (let i = 0; i < len; i++) {
            s += String.fromCharCode(mem[ptr + i])
        }

        return s
    }

    createTextNode(text: string, parentId: number): number {
        const [id, node] = this.atoms.createTextNode(text)
        const parent = this.atoms.getAtom(parentId).node()
        parent.appendChild(node)

        return id
    }

    createElement(attributes: Array<[string, string]>, parentId: number): number {
        const [id, node] = this.atoms.createElement(attributes)
        const parent = this.atoms.getAtom(parentId).node()
        parent.appendChild(node)

        return id
    }

    deleteAtom(id: number) {
        const node = this.atoms.getAtom(id).node()
        const parent = node.parentNode

        // The parent will always exist.
        parent!.removeChild(node)

        this.atoms.deleteAtom(id)
    }

    updateTextNode(id: number, text: string) {
        this.atoms.getAtom(id).node().nodeValue = text
    }

    textNodeToElement(id: number, attributes: Array<[string, string]>) {
        this.atoms.createElement(attributes, id)
    }

    elementToTextNode(id: number, text: string) {
        this.atoms.createTextNode(text, id)
    }

    updateElement(id: number, attributes: Array<[string, string]>) {
        const el = this.atoms.getAtom(id).node() as Element

        while (el.attributes.length > 0) {
            el.removeAttribute(el.attributes[0].name)
        }

        for (const [k, v] of attributes) {
            el.setAttribute(k, v)
        }
    }

    mountString(lengthPtr: number): number {
        const mem = this.mem()

        const length = this.mount.length
        const ptr = this.createString(this.mount)

        mem[lengthPtr] =     (length & 0x000000ff) >> 0
        mem[lengthPtr + 1] = (length & 0x0000ff00) >> 8
        mem[lengthPtr + 2] = (length & 0x00ff0000) >> 16
        mem[lengthPtr + 3] = (length & 0xff000000) >> 24

        return ptr
    }

    registerEvent(node: number, type: EventType) {
        this.atoms.getAtom(node).registerEvent(type, (json: string) => {
            this.callbackEvent(node, type, json)
        })
    }

    deleteEvent(node: number, type: EventType) {
        this.atoms.getAtom(node).deleteEvent(type)
    }

    injectStylesheet(contents: string) {
        const style = document.createElement('style')
        style.type = 'text/css'
        style.appendChild(document.createTextNode(contents))

        document.head.appendChild(style)
    }
}
