import type { FieldTypes } from "../FieldTypes"

export type FieldInfo = {
    fieldName: string,
    fieldType: FieldTypes
}

export function FieldInfo(fieldName: string, fieldType: FieldTypes): FieldInfo {
    return { fieldName, fieldType };
}