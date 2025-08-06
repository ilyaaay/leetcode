function twoSumFirst(nums: number[], target: number): number[] {
  const indecesMap = new Map();

  for (const [idx, x] of nums.entries()) {
    indecesMap.set(x, idx);
  }

  for (let i = 0; i < nums.length; i++) {
    const idx = indecesMap.get(target - nums[i]);

    if (idx !== undefined && idx != i) {
      return [i, idx];
    }
  }

  return [];
}

function twoSumSecond(nums: number[], target: number): number[] {
  const indecesMap = new Map();

  for (let i = 0; i < nums.length; i++) {
    const idx = indecesMap.get(target - nums[i]);

    if (idx !== undefined) {
      return [i, idx];
    }

    indecesMap.set(nums[i], i);
  }

  return [];
}
