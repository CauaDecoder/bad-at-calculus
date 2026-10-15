/** A matrix being typed: entries are strings (`"1/2"`, `"-3"`, `""`). */
export type MatrixDraft = readonly (readonly string[])[];

export const MIN_MATRIX_SIZE = 1;
export const MAX_MATRIX_SIZE = 6;

/** Grows or shrinks the grid, keeping what was already typed; new cells are empty. */
export function resizeEntries(draft: MatrixDraft, rows: number, columns: number): MatrixDraft {
  const clampedRows = clampSize(rows);
  const clampedColumns = clampSize(columns);
  return Array.from({ length: clampedRows }, (_, row) =>
    Array.from({ length: clampedColumns }, (_, column) => draft[row]?.[column] ?? ''),
  );
}

export function withEntry(
  draft: MatrixDraft,
  row: number,
  column: number,
  value: string,
): MatrixDraft {
  return draft.map((entries, rowIndex) =>
    rowIndex === row
      ? entries.map((entry, columnIndex) => (columnIndex === column ? value : entry))
      : entries,
  );
}

export function draftSize(draft: MatrixDraft): { rows: number; columns: number } {
  return { rows: draft.length, columns: draft[0]?.length ?? 0 };
}

function clampSize(size: number): number {
  return Math.min(Math.max(Math.round(size), MIN_MATRIX_SIZE), MAX_MATRIX_SIZE);
}
