import { isObject } from "./utils.js";

export const parse = (
  obj: Record<string, unknown>
): Record<string, Set<unknown>> => {
  // const raw = JSON.parse(json);
  const parsed: Record<string,  Set<unknown>> = {};
  for (const [key, value] of Object.entries(obj)) {
    parsed[key] = parseSchemaValues(value);
  }
  return parsed;
};

const parseSchemaValues = (value: unknown) => {
  const schemaValues = new Set();
  if (Array.isArray(value)) {
    for (const x of value) {
      if (isObject(x)) {
        schemaValues.add(parseSchemaObject(x));
      } else if(Array.isArray(x)) {
        schemaValues.add([parseSchemaValues(x)]);
      } else {
        schemaValues.add(x);
      }
    }
  }
  return schemaValues;
};

const parseSchemaObject = (obj: Record<string, unknown>): unknown => {
  const parsed: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(obj)) {
    if (Array.isArray(value)) {
      parsed[key] = parseSchemaValues(value);
    } else {
      console.log(obj);
      throw new Error("Invalid schema");
    }
  }
  return parsed;
};
