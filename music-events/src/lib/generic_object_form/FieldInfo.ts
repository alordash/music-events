import type { FieldTypes } from "./FieldTypes"
import type { GenericObject } from "./GenericObject";

export type FieldInfo = {
    fieldName: string,
    fieldType: FieldTypes,
    priority: number,
    objectExtractor: ObjectExtractor | undefined
}

export function FieldInfo(fieldName: string, fieldType: FieldTypes, priority = 100, objectExtractor: ObjectExtractor | undefined = undefined): FieldInfo {
    return { fieldName, fieldType, priority, objectExtractor };
}

export type ObjectExtractor = (id: number) => Promise<GenericObject | null>;