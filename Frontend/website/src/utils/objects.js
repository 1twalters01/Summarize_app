/**
 * @param {Object} enumObj
 * @param {number|string} value - The value to find the key for
 * @returns {string|undefined} The key corresponding to the given value
 */
export function getKeyByValue(enumObj, value) {
  return Object.keys(enumObj).find((key) => enumObj[key] === value);
}
