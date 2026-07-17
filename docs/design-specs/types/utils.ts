


export type CustomData = { [i in string]: string }
export type WithCustomData<T> = T & CustomData
export type Optional<T> = T | undefined | null

