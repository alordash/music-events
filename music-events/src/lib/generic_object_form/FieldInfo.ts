import type { FieldTypes } from "./FieldTypes"

export type FieldInfo = {
    fieldName: string,
    fieldType: FieldTypes,
    priority: number
}

export function FieldInfo(fieldName: string, fieldType: FieldTypes, priority = 100): FieldInfo {
    return { fieldName, fieldType, priority };
}