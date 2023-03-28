import { writeFile } from "fs/promises";
const SCHEMA_ARRAY_PATH = "../schema-array.json";

async function main() {
  const schema_array = (
    await import(SCHEMA_ARRAY_PATH, {
      assert: { type: "json" },
    })
  ).default;
  const combined_schema = combine_schemas(schema_array);
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

function combine_schemas(json_array: Schema[]): Schema {
  // Add first schema without "Undefined"s
  const result: Schema = json_array[0];

  // Skip first schema
  for (const schema of json_array.slice(1)) {
    for (const [key, value] of Object.entries(schema)) {
      if (key in result) {
        result[key] = compare(result[key], value);
      } else {
        result[key] = value;
      }
    }
  }
  return result;
}

function compare(result: SchemaValue, schema: SchemaValue): SchemaValue[] {
  const schema_value: SchemaValue[] = [];
  if (Array.isArray(result) && Array.isArray(schema)) {
    schema_value.push(...result);
    for (const value of schema) {
      if (!schema_value.includes(value)) {
        schema_value.push(value);
      }
    }
  }
  if (typeof result === "object" && typeof schema === "object") {
    for (const [key, value] of Object.entries(result)) {
      if (key in schema) {
        // @ts-ignore
        schema_value.push({ [key]: compare(value, schema[key]) });
      } else {
        schema_value.push({ [key]: value });
      }
    }
  }
  if (typeof result === "string" && typeof schema === "string") {
    if (result !== schema) {
      schema_value.push(result, schema);
    } else {
      schema_value.push(result);
    }
  }
  return schema_value;
}

await main();
