import pkg from "lodash/fp.js";
import { isObject } from "./utils.js";
const { cloneDeep, isEqual } = pkg;

export const merge = (
  schemaOne: Record<string, unknown>,
  schemaTwo: Record<string, unknown>
): Record<string, unknown> => {
  const merged: Record<string, unknown> = cloneDeep(schemaOne);

  for (const [key, value] of Object.entries(schemaTwo)) {
    if (key in merged) {
      try {
        merged[key] = mergeSchemaValues(merged[key], value);
      } catch (e) {
        console.log("Error merging schemas");
        console.log("Schema 1:");
        console.log(schemaOne);
        console.log("Schema 2:");
        console.log(schemaTwo);
        console.log(e);
        throw e;
      }
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
    const arrays = [...merged.values()].filter(Array.isArray);
    if (objects.length > 1) {
      throw new Error("Each schema value can only contain one object");
    }
    if (arrays.length > 1) {
      throw new Error("Each schema value can only contain one array");
    }
    const originalObject = objects[0];
    const originalArray = arrays[0];
    for (const x of valueTwo) {
      if (isObject(x) && originalObject) {
        // @ts-ignore For now I'm being pretty loose with the types
        merged.add(mergeSchemaObjects(originalObject, x));
        merged.delete(originalObject);
      } else if (Array.isArray(x)) {
        merged.delete(originalArray);
        merged.add(mergeSchemaArrays(originalArray, x));
      } else {
        merged.add(x);
      }
    }
    return merged;
  } else {
    console.log("valueOne", valueOne);
    console.log("valueTwo", valueTwo);
    throw new Error("Schema values are not of type Set");
  }
};

const mergeSchemaArrays = (arrOne: unknown[], arrTwo: unknown[]): unknown[] => {
  if (isEqual(arrOne[0], arrTwo[0])) {
    return arrTwo;
  } else if (arrOne.length === 0) {
    return arrTwo;
  } else if (arrTwo.length === 0) {
    return arrOne;
  } else {
    console.log("arrOne", arrOne);
    console.log("arrTwo", arrTwo);
    throw new Error("Schema arrays are not of the same type");
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
