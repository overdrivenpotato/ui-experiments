import { Wrapper } from './wrapper'
import { conditionalCurry } from './util'
import { EventType } from './events'

interface BlocksEnv {
    blocks_out_println: (ptr: number, len: number) => void
    blocks_out_defer: (f: number) => void
    blocks_out_mount_id: (lengthPtr: number) => number
    blocks_out_create_element: (ptr: number, len: number) => number
    blocks_out_create_text_node: (ptr: number, len: number, parent: number) => number
    blocks_out_update_text_node: (ptr: number, len: number, id: number) => void
    blocks_out_delete_node: (id: number) => void
    blocks_out_node_text_to_element: (ptr: number, len: number) => void
    blocks_out_element_to_text_node: (ptr: number, len: number, id: number) => void
    blocks_out_update_element: (id: number, ptr: number, len: number) => void
    blocks_out_create_event: (atom: number, type: number) => void
    blocks_out_delete_event: (atom: number, type: number) => void
    blocks_out_inject_stylesheet: (ptr: number, len: number) => void
}

const load = async (binary: string, mount: string) => {
    let wrapper: Wrapper | null = null
    const deferList: Array<number> = []

    // Usage:
    //
    // ```
    // withWrapper((wrapper: Wrapper) => (arg1: number, arg2: number) => {
    //     // Here, wrapper is guaranteed not to be null.
    //     wrapper.println(...)
    // })
    // ```
    const withWrapper = conditionalCurry(() => wrapper, () => {
        'Wrapper was not initialized. This should be unreachable.'
    })

    const env: BlocksEnv = {
        blocks_out_println: withWrapper(wrapper => (ptr: number, len: number) => {
            const text = wrapper.readString(ptr, len)

            console.log(text)
        }),
        blocks_out_defer: (f) => {
            deferList.push(f)
        },
        blocks_out_mount_id: withWrapper(wrapper => (lengthPtr: number): number => {
            return wrapper.mountString(lengthPtr)
        }),
        blocks_out_create_element: withWrapper(wrapper => (ptr: number, len: number): number => {
            interface CreateElement {
                parent: number
                attributes: Array<[string, string]>
            }

            const params: CreateElement = JSON.parse(wrapper.readString(ptr, len))

            return wrapper.createElement(params.attributes, params.parent)
        }),
        blocks_out_create_text_node: withWrapper(wrapper => (ptr: number, len: number, parent: number): number => {
            return wrapper.createTextNode(wrapper.readString(ptr, len), parent)
        }),
        blocks_out_update_text_node: withWrapper(wrapper => (ptr: number, len: number, id: number) => {
            wrapper.updateTextNode(id, wrapper.readString(ptr, len))
        }),
        blocks_out_delete_node: withWrapper(wrapper => (id: number) => {
            wrapper.deleteAtom(id)
        }),
        blocks_out_node_text_to_element: withWrapper(wrapper => (ptr: number, len: number) => {
            interface TextToElement {
                id: number
                attributes: Array<[string, string]>
            }

            const params: TextToElement = JSON.parse(wrapper.readString(ptr, len))

            wrapper.textNodeToElement(params.id, params.attributes)
        }),
        blocks_out_element_to_text_node: withWrapper(wrapper => (ptr: number, len: number, id: number) => {
            wrapper.elementToTextNode(id, wrapper.readString(ptr, len))
        }),
        blocks_out_update_element: withWrapper(wrapper => (id: number, ptr: number, len: number) => {
            interface UpdateElement {
                attributes: Array<[string, string]>,
            }

            const params: UpdateElement = JSON.parse(wrapper.readString(ptr, len))

            wrapper.updateElement(id, params.attributes)
        }),
        blocks_out_create_event: withWrapper(wrapper => (atom: number, type: number) => {
            wrapper.registerEvent(atom, type)
        }),
        blocks_out_delete_event: withWrapper(wrapper => (atom: number, type: number) => {
            wrapper.deleteEvent(atom, type)
        }),
        blocks_out_inject_stylesheet: withWrapper(wrapper => (ptr: number, len: number) => {
            wrapper.injectStylesheet(wrapper.readString(ptr, len))
        })
    }

    const response = await fetch(binary)
    const bytes = await response.arrayBuffer()
    const wasmModule = await (window as any).WebAssembly.instantiate(bytes, { env })
    wrapper = new Wrapper(wasmModule, mount)

    // Call the deferred hooks.
    for (const f of deferList) {
        wrapper.callback0(f)
    }

    // Module is run...
}

export default () => {
    const SCRIPT_ID = `blocks-loader`

    const el = document.getElementById(SCRIPT_ID)

    if (!el) {
        throw `Could not find script with ID \`${SCRIPT_ID}\`.`
    }

    const binary = el.getAttribute('data-binary')
    const mount = el.getAttribute('data-mount')

    if (!binary) {
        throw `Script \`#${SCRIPT_ID}\` did not have attribute \`data-binary\``
    }

    if (!mount) {
        throw `Script \`#${SCRIPT_ID}\` did not have attribute \`data-mount\``
    }

    if (!document.getElementById(mount)) {
        throw `Element \`#${mount}\` does not exist.`
    }

    load(binary, mount).catch(e => console.error(e))
}
