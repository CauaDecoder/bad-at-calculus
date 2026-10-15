import { describe, expect, it } from 'vitest';
import { resizeEntries, withEntry } from './matrix-draft';

describe('resizeEntries', () => {
  it('keeps typed entries and fills new cells with blanks', () => {
    expect(resizeEntries([['1', '2']], 2, 3)).toEqual([
      ['1', '2', ''],
      ['', '', ''],
    ]);
  });

  it('never goes below 1×1 or above 6×6', () => {
    expect(resizeEntries([['1']], 0, 9)).toHaveLength(1);
    expect(resizeEntries([['1']], 0, 9)[0]).toHaveLength(6);
  });
});

describe('withEntry', () => {
  it('replaces only the edited cell', () => {
    expect(
      withEntry(
        [
          ['1', '2'],
          ['3', '4'],
        ],
        1,
        0,
        '9',
      ),
    ).toEqual([
      ['1', '2'],
      ['9', '4'],
    ]);
  });
});
