export const isObject = (x: unknown): x is Record<string, unknown> => {
    return typeof x === "object" && !Array.isArray(x);
  };
  