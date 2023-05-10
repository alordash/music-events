import type { FieldTypes } from "./FieldTypes"
import type { GenericObject } from "./GenericObject";
import type { ExplorationResult } from "./explorer/Paging";

export type FieldInfo = {
    fieldName: string,
    fieldType: FieldTypes,
    priority: number,
    objectExtractor: ObjectExtractor | undefined,
    objectExplorer: ObjectExplorer | undefined,
    totalCountExtractor: TotalCountExtractor | undefined,
    fieldComposer: FieldComposer | undefined,
    objectName: string | undefined
}

export function FieldInfo(
    fieldName: string,
    fieldType: FieldTypes,
    priority = 100,
    objectExtractor: ObjectExtractor | undefined = undefined,
    objectExplorer: ObjectExplorer | undefined = undefined,
    totalCountExtractor: TotalCountExtractor | undefined = undefined,
    fieldComposer: FieldComposer | undefined = undefined,
    objectName: string | undefined = undefined
): FieldInfo {
    return {
        fieldName,
        fieldType,
        priority,
        objectExtractor,
        objectExplorer,
        totalCountExtractor,
        fieldComposer,
        objectName
    };
}

export type ObjectExtractor = (id: number) => Promise<GenericObject | null>;
export type ObjectExplorer = (count: number, offset: number) => Promise<ExplorationResult>;
export type TotalCountExtractor = () => Promise<number>;
export type FieldComposer = (fieldName: string) => FieldInfo;