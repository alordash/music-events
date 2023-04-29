import type { FieldTypes } from "./FieldTypes";

export interface DisplayableObject {
    getFieldType(fieldName: string): FieldTypes;
    formatFieldName(fieldName: string): string;
}