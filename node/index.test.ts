import { parse } from "./index";

describe("parse", () => {
  it("should parse a JSON Object", () => {
    expect(
      parse(`{
      "field1": ["String", "Int32"]
    }`)
    ).toEqual({ field1: new Set(["String", "Int32"]) });
  });

  it("should parse a nested JSON Object", () => {
    expect(
      parse(`{
      "field1": ["String", { "field2": ["Int32"] }]
    }`)
    ).toEqual({
      field1: new Set(["String", { field2: new Set(["Int32"]) }]),
    });
  });

  it("should parse a deeply nested JSON Object", () => {
    expect(
      parse(`{
      "field1": ["String", { "field2": ["Int32", { "field3": ["String"]}] }]
    }`)
    ).toEqual({
      field1: new Set([
        "String",
        { field2: new Set(["Int32", { field3: new Set(["String"]) }]) },
      ]),
    });
  });
});
