import pkg from "lodash/fp.js";
import { isObject } from "./utils.js";
const { cloneDeep, isEqual } = pkg;

const mergeSchemaValues = (valueOne: unknown, valueTwo: unknown): Set<unknown> => {
  if (valueOne === undefined && valueTwo instanceof Set) {
    const merged = new Set(valueTwo);
    merged.add("Undefined");
    return merged;
  } else if (valueOne instanceof Set && valueTwo === undefined) {
    const merged = new Set(valueOne);
    merged.add("Undefined");
    return merged;
  } else if (valueOne instanceof Set && valueTwo instanceof Set) {
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
    throw new Error("Neither Schema value is a set");
  }
};

export const mergeSets = (
  setOne: Set<unknown>,
  setTwo: Set<unknown>
): Set<unknown> => {
  const set = new Set(setOne);
  for (const x of setTwo) {
    if (!deepIncludes(setOne, x)) set.add(x);
  }
  return set;
};

const deepIncludes = (set: Set<unknown>, value: unknown): boolean => {
  for (const x of set) {
    if (isEqual(x, value)) return true;
  }
  return false;
};

const mergeSchemaArrays = (
  arrOne: Set<unknown>[],
  arrTwo: Set<unknown>[]
): Set<unknown>[] => {
  if (arrOne.length > 1 || arrTwo.length > 1) throw new Error("Invalid schema");
  if (isEqual(arrOne[0], arrTwo[0])) {
    return arrTwo;
  } else if (arrOne.length === 0) {
    return arrTwo;
  } else if (arrTwo.length === 0) {
    return arrOne;
  } else {
    return [mergeSets(arrOne[0], arrTwo[0])];
  }
};

const mergeSchemaObjects = (
  objOne: Record<string, Set<unknown>>,
  objTwo: Record<string, Set<unknown>>
): Record<string, Set<unknown>> => {
  const allKeys = new Set([...Object.keys(objOne), ...Object.keys(objTwo)]);
  const merged: Record<string, Set<unknown>> = {};
  for(const key of allKeys) {
    if(key in objOne && key in objTwo) {
      merged[key] = mergeSchemaValues(objOne[key], objTwo[key]);
    } else if(key in objOne) {
      const typeSet = new Set(objOne[key]);
      typeSet.add("Undefined")
      merged[key] = typeSet
    } else if(key in objTwo) {
      const typeSet = new Set(objTwo[key]);
      typeSet.add("Undefined")
      merged[key] = typeSet
    }
  }
  return merged;
};

export const merge = mergeSchemaObjects;
