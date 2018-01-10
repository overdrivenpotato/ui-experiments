export const conditionalCurry = <T>(getter: () => T | null, error: object | string) =>
    <F extends Function>(f: (t: T) => F): F =>
        ((...args: any[]) => {
            const value = getter()

            if (value != null) {
                return f(value)(...args)
            } else {
                throw error
            }
        }) as any as F
