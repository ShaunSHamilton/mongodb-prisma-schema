import { merge } from "./merger";

describe("merge", () => {
  it("should merge two simple schemas", () => {
    expect(
      merge(
        {
          field1: new Set(["String", "Int32"]),
        },
        {
          field1: new Set(["String", "Int64"]),
        }
      )
    ).toEqual({
      field1: new Set(["String", "Int32", "Int64"]),
    });
  });

  it("should merge two nested schemas", () => {
    expect(
      merge(
        {
          field1: new Set(["String", { field2: new Set(["Int32"]) }]),
        },
        {
          field1: new Set(["String", { field2: new Set(["Int64"]) }]),
        }
      )
    ).toEqual({
      field1: new Set(["String", { field2: new Set(["Int32", "Int64"]) }]),
    });
  });
});
