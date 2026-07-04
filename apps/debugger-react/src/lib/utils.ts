export function hex(n: number, width = 16) {
  return (
    "0x" +
    Math.max(0, Math.trunc(n)).toString(16).toUpperCase().padStart(width, "0")
  );
}

export function byteHex(b: number) {
  return b.toString(16).toUpperCase().padStart(2, "0");
}

export function byteAscii(b: number) {
  return b >= 0x20 && b <= 0x7e ? String.fromCharCode(b) : ".";
}

export function stripIndents(
  strings: TemplateStringsArray,
  ...values: unknown[]
) {
  const raw = typeof strings === "string" ? [strings] : strings.raw;
  let result = "";

  for (let i = 0; i < raw.length; i++) {
    result += raw[i];
    if (i < values.length) {
      result += values[i];
    }
  }

  const lines = result.split("\n");
  const minIndent = lines
    .filter((line) => line.trim())
    .reduce((min, line) => {
      const indent = line.match(/^\s*/)[0].length;
      return Math.min(min, indent);
    }, Infinity);

  return lines.map((line) => line.slice(minIndent)).join("\n");
}
