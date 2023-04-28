import { parse } from "./parser";

describe("parse", () => {
  it("should parse an Object", () => {
    expect(
      parse({
        field1: ["String", "Int32"],
      })
    ).toEqual({ field1: new Set(["String", "Int32"]) });
  });

  it("should parse a nested Object", () => {
    expect(
      parse({
        field1: ["String", { field2: ["Int32"] }],
      })
    ).toEqual({
      field1: new Set(["String", { field2: new Set(["Int32"]) }]),
    });
  });

  it("should parse a deeply nested Object", () => {
    expect(
      parse({
        field1: ["String", { field2: ["Int32", { field3: ["String"] }] }],
      })
    ).toEqual({
      field1: new Set([
        "String",
        { field2: new Set(["Int32", { field3: new Set(["String"]) }]) },
      ]),
    });
  });
  it("should handle objects with arrays", () => {
    expect(
      parse({
        field1: ["String", []],
        field2: ["String", "Int32", ["String"]],
      })
    ).toEqual({
      field1: new Set(["String", [new Set()]]),
      field2: new Set(["String", "Int32", [new Set(["String"])]]),
    });
  });
});
