import { useEffect, useRef, type MutableRefObject } from 'react';

/** A ref that turns true once `value` has been true and stays true for the component's life. */
export function useLatch(value: boolean): MutableRefObject<boolean> {
  const latched = useRef(value);
  useEffect(() => {
    if (value) latched.current = true;
  }, [value]);
  return latched;
}
