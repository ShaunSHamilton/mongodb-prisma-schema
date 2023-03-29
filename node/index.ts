import { writeFile } from "fs/promises";
// import schema_array from "../schema-array.json";

// const schemaArray = schema_array as unknown as Schema[];

// async function main() {
//   const combined_schema = combine_schemas(schemaArray);
//   await writeFile(
//     "combined-schema.json",
//     JSON.stringify(combined_schema, null, 2)
//   );
// }

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

function combine_schemas(json_array: Schema[]): Schema {
  // Add first schema without "Undefined"s
  const result: Schema = json_array[0];

  // Skip first schema
  for (const schema of json_array.slice(1)) {
    for (const [key, value] of Object.entries(schema)) {
      if (key in result) {
        result[key] = combine(result[key], value);
      } else {
        result[key] = value;
      }
    }
  }
  return result;
}

function combine(result: SchemaValue, schema: SchemaValue): SchemaValue[] {
  const schema_values: SchemaValue[] = [];
  // if both are arrays, add all new values to the result
  if (Array.isArray(result) && Array.isArray(schema)) {
    schema_values.push(...result);
    for (const value of schema) {
      if (!schema_values.includes(value)) {
        schema_values.push(value);
      }
    }
  }
  if (typeof result === "object" && typeof schema === "object") {
    for (const [key, value] of Object.entries(result)) {
      if (key in schema) {
        // @ts-ignore
        schema_values.push({ [key]: combine(value, schema[key]) });
      } else {
        schema_values.push({ [key]: value });
      }
    }
  }
  if (typeof result === "string" && typeof schema === "string") {
    if (result !== schema) {
      schema_values.push(result, schema);
    } else {
      schema_values.push(result);
    }
  }
  return schema_values;
}

// main();

export const parse = (json: string): unknown => {
  const raw = JSON.parse(json);
  const parsed: Record<string, unknown> = {};
  for (const [key, value] of Object.entries(raw)) {
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
      throw new Error("Invalid schema");
    }
  }
  return parsed;
};

