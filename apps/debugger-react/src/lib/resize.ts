export function beginResize(
  e: React.MouseEvent<HTMLDivElement>,
  startSize: number,
  axis: "x" | "y",
  invert: boolean,
  setSize: (newSize: number) => void,
  min: number,
  max: number,
) {
  e.preventDefault();
  const startPos = axis === "x" ? e.clientX : e.clientY;
  document.body.style.cursor = axis === "x" ? "col-resize" : "row-resize";
  document.body.style.userSelect = "none";
  function onMove(ev) {
    const pos = axis === "x" ? ev.clientX : ev.clientY;
    let delta = pos - startPos;
    if (invert) delta = -delta;
    let next = startSize + delta;
    next = Math.max(min, Math.min(max, next));
    setSize(next);
  }
  function onUp() {
    window.removeEventListener("mousemove", onMove);
    window.removeEventListener("mouseup", onUp);
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
  }
  window.addEventListener("mousemove", onMove);
  window.addEventListener("mouseup", onUp);
}
