import { afterEach, describe, expect, test } from 'bun:test';
import { act, create, type ReactTestRenderer } from 'react-test-renderer';
import { useLatch } from '../../src/hooks/useLatch';

let latched: { current: boolean };
function Probe({ value }: { value: boolean }) {
  latched = useLatch(value);
  return null;
}

let renderer: ReactTestRenderer | undefined;
afterEach(async () => {
  if (renderer) await act(async () => renderer!.unmount());
  renderer = undefined;
});

async function render(value: boolean) {
  await act(async () => {
    if (renderer) renderer.update(<Probe value={value} />);
    else renderer = create(<Probe value={value} />);
  });
}

describe('latch', () => {
  test('stays false until the value is first true', async () => {
    await render(false);
    expect(latched.current).toBe(false);
  });

  test('stays true after the value turns false again', async () => {
    await render(false);
    await render(true);
    expect(latched.current).toBe(true);
    await render(false);
    expect(latched.current).toBe(true);
  });
});
