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
  
  it("should not duplicate array types", () => {
    expect(
      merge(
        {
          field1: new Set(["String", [new Set("Int32")]]),
        },
        {
          field1: new Set(["String", [new Set("Int32")]]),
        }
      )
    ).toEqual({
      field1: new Set(["String", [new Set("Int32")]]),
    });
  })
  
  it("should replace empty array with typed arrays", () => {
    expect(
      merge(
        {
          field1: new Set(["String", []]),
        },
        {
          field1: new Set(["String", [new Set("Int32")]]),
        }
      )
    ).toEqual({
      field1: new Set(["String", [new Set("Int32")]]),
    });
    
    expect(
      merge(
        {
          field1: new Set(["String", [new Set("Int32")]]),
        },
        {
          field1: new Set(["String", []]),
        }
      )
    ).toEqual({
      field1: new Set(["String", [new Set("Int32")]]),
    });
  })
});
