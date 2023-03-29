import pkg from "lodash/fp.js";
import { isObject } from "./utils.js";
const { cloneDeep } = pkg;

export const merge = (
    schemaOne: Record<string, unknown>,
    schemaTwo: Record<string, unknown>
  ): Record<string, unknown> => {
    const merged: Record<string, unknown> = cloneDeep(schemaOne);
    for (const [key, value] of Object.entries(schemaTwo)) {
      if (key in merged) {
        merged[key] = mergeSchemaValues(merged[key], value);
      } else {
        merged[key] = value;
      }
    }
    return merged;
  };
  

  const mergeSchemaValues = (valueOne: unknown, valueTwo: unknown): unknown => {
    if (valueOne instanceof Set && valueTwo instanceof Set) {
      const merged = new Set(valueOne);
      const objects = [...merged.values()].filter(isObject);
      if (objects.length > 1) {
        throw new Error("Each schema value can only contain one object");
      }
      const originalObject = objects[0];
      for (const x of valueTwo) {
        if (isObject(x) && originalObject) {
          // @ts-ignore For now I'm being pretty loose with the types
          merged.add(mergeSchemaObjects(originalObject, x));
          merged.delete(originalObject);
        } else {
          merged.add(x);
        }
      }
      return merged;
    } else {
      throw new Error("Schema values are not of type Set");
    }
  };
  
  const mergeSchemaObjects = (
    objOne: Record<string, unknown> | undefined,
    objTwo: Record<string, unknown>
  ): Record<string, unknown> => {
    const merged: Record<string, unknown> = cloneDeep(objTwo);
    if (!objOne) return merged;
    for (const [key, value] of Object.entries(objOne)) {
      if (key in merged) {
        merged[key] = mergeSchemaValues(merged[key], value);
      } else {
        merged[key] = value;
      }
    }
    return merged;
  };