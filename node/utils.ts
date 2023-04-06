// We're declaring that the values are sets, but we're not checking that they
// are. This is a bit risky.
export const isObject = (x: unknown): x is Record<string, Set<unknown>> => {
  return typeof x === "object" && !Array.isArray(x);
};
