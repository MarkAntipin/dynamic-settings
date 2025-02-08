export enum SettingsValueType {
    Str = "str",
    Int = "int",
    Float = "float",
    Bool = "bool",
    Json = "json"
}

export interface Settings {
    key: string
    value: string
    type: SettingsValueType
}