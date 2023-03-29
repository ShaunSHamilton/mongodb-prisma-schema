import { writeFile } from "fs/promises";
import pkg from "lodash/fp.js";
const { cloneDeep } = pkg;
import { readFileSync } from "fs";

const data = readFileSync("../schema-array.json", "utf8");
// import schema_array from "../schema-array.json";

const schemaArray = JSON.parse(data);

async function main() {
  const combined_schema = combine_schemas(schemaArray);
  await writeFile(
    "combined-schema.json",
    JSON.stringify(combined_schema, null, 2)
  );
}

/**
 * Recurse through the schema array and combine all the fields' values into a set
 * Rules:
 * - If a field is new, add undefined to the set
 *
 * **Example**:
 *
 * ```json
 * [
 *  {
 *   "field1": ["String", "Int32"]
 *  },
 *  {
 *  "field1": ["String", "Null"],
 *  "field2": [{
 *     "field3": [["String"], "String"]
 *    },
 *    "Boolean"
 *   ]
 *  }
 * ]
 * ```
 *
 * Should become:
 *
 * ```json
 * {
 *   "field1": ["String", "Int32", "Null"],
 *   "field2": [
 *     "Undefined",
 *     {
 *     "field3": [["String"], "String"]
 *     },
 *    "Boolean"
 *   ]
 * }
 * ```
 */

type SchemaValue =
  | "Double"
  | "String"
  | "Boolean"
  | "Null"
  | "Int32"
  | "Int64"
  | "ObjectId"
  | "DateTime"
  | "Undefined" // Not in array, but added in final schema
  | SchemaValue[]
  | { [key: string]: SchemaValue };

type Schema = { [key: string]: SchemaValue[] };

// If key is missing in schema, add whole json value
// If key is missing in json, add "Undefined" to schema

export const parse = (
  obj: Record<string, unknown>
): Record<string, unknown> => {
  // const raw = JSON.parse(json);
  const parsed: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(obj)) {
    parsed[key] = parseSchemaValues(value);
  }
  return parsed;
};

const parseSchemaValues = (value: unknown) => {
  const schemaValues = new Set();
  if (Array.isArray(value)) {
    for (const x of value) {
      if (typeof x === "object") {
        schemaValues.add(parseSchemaObject(x));
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

const isObject = (x: unknown): x is Record<string, unknown> => {
  return typeof x === "object" && !Array.isArray(x);
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

function combine_schemas(json_array: Schema[]): Record<string, unknown> {
  // Add first schema without "Undefined"s
  let result: Record<string, unknown> = json_array[0];

  const parsed = json_array.map(parse);

  // Skip first schema
  for (const schema of parsed.slice(1)) {
    result = merge(result, schema);
  }
  return result;
}

main();
